#[cfg(feature = "ssr")]
async fn init_team_names() -> anyhow::Result<()> {
    use blue_scout::TEAM_NAMES;
    use reqwest::header;
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct TeamResponse {
        key: String,
        nickname: String,
    }

    #[derive(Deserialize)]
    pub struct Response {
        teams: Vec<TeamResponse>,
    }

    let mut headers = header::HeaderMap::new();
    headers.insert("accept", "application/json".parse().unwrap());

    let api_key = std::env::var("TBA_API_KEY");

    if api_key.is_err() {
        return Err(anyhow::anyhow!("Expected TBA_API_KEY in .env file"));
    }

    headers.insert("X-TBA-Auth-Key", std::env::var("TBA_API_KEY")?.parse()?);

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;
    let res = client
        .get("https://www.thebluealliance.com/api/v3/search_index")
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    let info: Vec<(u32, String)> = serde_json::from_str::<Response>(&res)?
        .teams
        .into_iter()
        .map(|team| {
            (
                team.key.trim_start_matches("frc").parse().unwrap(),
                team.nickname,
            )
        })
        .collect();

    let mut team_names: Vec<Option<String>> =
        vec![None; info.iter().map(|x| x.0).max().unwrap() as usize + 1];

    for team in info {
        team_names[team.0 as usize] = Some(team.1);
    }

    TEAM_NAMES.set(team_names).unwrap();

    Ok(())
}

#[cfg(feature = "ssr")]
use axum::response::IntoResponse;

#[cfg(feature = "ssr")]
pub async fn generate_xlsx() -> anyhow::Result<impl IntoResponse> {
    use std::io::Cursor;

    use axum::response::Response;
    use blue_scout::{data::DataPoint, db::DB};
    use duckdb::arrow::datatypes::DataType;
    use reqwest::header::CONTENT_TYPE;
    use rust_xlsxwriter::{workbook::Workbook, Format};

    let conn = DB.get().unwrap().lock().await;

    let mut stmt = conn.prepare("SELECT * EXCLUDE(id) FROM scout_entries")?;

    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet().set_name("Data")?;

    let bold = Format::new().set_bold();

    worksheet.write_row_with_format(
        0,
        0,
        DataPoint::field_pretty_names()
            .iter()
            .map(|(_, x)| x.to_string())
            .collect::<Vec<String>>(),
        &bold,
    )?;

    let mut current_row = 1;
    stmt.query_map([], |row| {
        for i in 0..row.as_ref().column_count() {
            let t: DataType = row.as_ref().column_type(i);
            match t {
                DataType::Null => worksheet
                    .write_string(current_row, i as u16, "NULL")
                    .unwrap(),
                DataType::Boolean => worksheet
                    .write_string(
                        current_row,
                        i as u16,
                        if row.get::<_, bool>(i)? { "Yes" } else { "No" },
                    )
                    .unwrap(),
                DataType::Int8 | DataType::Int16 | DataType::Int32 | DataType::Int64 => worksheet
                    .write_number(current_row, i as u16, row.get::<_, i64>(i)? as f64)
                    .unwrap(),
                DataType::UInt8 | DataType::UInt16 | DataType::UInt32 | DataType::UInt64 => {
                    worksheet
                        .write_number(current_row, i as u16, row.get::<_, u64>(i)? as f64)
                        .unwrap()
                }
                DataType::Float16 | DataType::Float32 | DataType::Float64 => worksheet
                    .write_number(current_row, i as u16, row.get::<_, f64>(i)?)
                    .unwrap(),
                DataType::Utf8 => worksheet
                    .write_string(current_row, i as u16, row.get::<_, String>(i)?)
                    .unwrap(),
                _ => unimplemented!("Unsupported data type: {:?}", t),
            };
        }
        current_row += 1;
        Ok(())
    })?
    .count();

    worksheet.autofit_to_max_width(300);

    let mut buf = Cursor::new(Vec::new());
    workbook.save_to_writer(&mut buf)?;

    Ok(Response::builder()
        .header(
            CONTENT_TYPE,
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        )
        .body(axum::body::Body::from(buf.into_inner()))?)
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    if cfg!(feature = "ssr") && cfg!(feature = "hydrate") {
        panic!("Both SSR and Hydration features are enabled! TODO: Fix");
    }
    use axum::{error_handling::HandleError, Router};
    use blue_scout::{app::*, db::init_db};
    use dotenv::dotenv;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use reqwest::StatusCode;

    async fn handle_anyhow_error(err: anyhow::Error) -> (StatusCode, String) {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {err}"),
        )
    }

    tracing_subscriber::fmt::init();
    dotenv().ok();

    init_team_names().await.unwrap();

    init_db().await.unwrap();

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options)
        .route_service(
            "/download-xlsx",
            HandleError::new(
                tower::service_fn(|_req| async {
                    let res = generate_xlsx().await?;
                    Ok::<_, anyhow::Error>(res)
                }),
                handle_anyhow_error,
            ),
        );

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
