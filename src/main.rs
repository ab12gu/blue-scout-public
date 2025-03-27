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
pub async fn generate_xlsx() -> impl IntoResponse {
    use axum::response::Response;
    use blue_scout::db::DB;
    use reqwest::header::CONTENT_TYPE;
    use std::io::Cursor;

    let conn = DB.get().unwrap().lock().await;

    let mut buffer = Cursor::new(Vec::new());

    // Query the table's columns
    let mut stmt = conn.prepare("PRAGMA table_info(scout_entries)").unwrap();
    let columns = stmt
        .query_map([], |row| {
            let name: String = row.get(1)?;
            let column_type: String = row.get(2)?;
            Ok((name, column_type))
        })
        .unwrap();

    let mut column_names = Vec::new();
    let mut select_columns = Vec::new();

    // Iterate through the columns and exclude 'id' while checking the type
    for (col_name, col_type) in columns.filter_map(|col| col.ok()) {
        if col_name != "id" {
            column_names.push(col_name.clone());

            // Check if the column is of type BOOLEAN and apply the CASE conversion
            if col_type.to_lowercase() == "boolean" {
                select_columns.push(format!(
                    "CASE WHEN {} THEN 'Yes' ELSE 'No' END AS {}",
                    col_name, col_name
                ));
            } else {
                select_columns.push(col_name);
            }
        }
    }

    // Join the selected columns into a comma-separated string
    let column_list = select_columns.join(", ");

    // Build the SQL query
    let query = format!(
        "COPY (SELECT {} FROM scout_entries) TO 'output.xlsx' (FORMAT xlsx, SHEET 'Data', HEADER true);",
        column_list
    );

    // Execute the query to export the data
    conn.execute(&query, []).unwrap();

    //workbook.save_to_writer(&mut buffer).unwrap();

    Response::builder()
        .header(
            CONTENT_TYPE,
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        )
        .body(axum::body::Body::from(buffer.into_inner()))
        .unwrap()
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{routing::get, Router};
    use blue_scout::{app::*, db::init_db};
    use dotenv::dotenv;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};

    tracing_subscriber::fmt::init();
    dotenv().ok();

    init_team_names().await.unwrap();

    init_db().unwrap();

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
        .with_state(leptos_options);

    let app = app.route("/download-xlsx", get(generate_xlsx));

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
