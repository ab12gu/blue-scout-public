//! This is the main entry point for the Blue Scout application.

use axum::body::Body;
#[cfg(feature = "ssr")]
use axum::response::IntoResponse;
use blue_scout::db::get_conn;
use frozen_collections::FzScalarMap;
use tbaapi::apis::configuration::Configuration;

/// Initializes the team names by fetching data from the TBA API and storing it
/// in a global variable.
///
/// This function is only compiled and executed if the `ssr` feature is enabled.
///
/// # Errors
///
/// Returns an `anyhow::Error` if there is an issue with the API request or data
/// processing.
#[cfg(feature = "ssr")]
async fn init_team_names() -> anyhow::Result<()> {
    use blue_scout::{api_config, TEAM_NAMES};
    use tbaapi::apis::default_api::get_search_index;

    let res = get_search_index(api_config()).await?;

    let info: Vec<(u32, String)> = res
        .teams
        .into_iter()
        .filter_map(|team| match team.key.trim_start_matches("frc").parse() {
            Ok(val) => Some((val, team.nickname)),
            Err(err) => {
                tracing::error!("Error parsing team key: {err}");
                None
            }
        })
        .collect();

    if info.is_empty() {
        return Err(anyhow::anyhow!("Team keys is empty"));
    }

    let team_names = FzScalarMap::new(info);

    TEAM_NAMES
        .set(team_names)
        .expect("TEAM_NAMES should not be set yet");

    Ok(())
}

/// Generates an XLSX file containing data from the `scout_entries` table in the
/// database.
///
/// This function queries all columns (excluding `id`) from the `scout_entries`
/// table, formats the data, and writes it to an XLSX file. The file is then
/// returned as an HTTP response with the appropriate content type.
///
/// # Errors
///
/// This function can return an `anyhow::Error` in the following cases:
///
/// - Failure to acquire a database connection.
/// - Failure to prepare the SQL query.
/// - Failure to execute the SQL query.
/// - Failure to write data to the XLSX file.
/// - Failure to save the workbook to the writer.
/// - Failure to build the HTTP response.
///
/// # Panics
///
/// This function may panic if an unsupported data type is encountered when
/// reading from the database.
///
/// # Returns
///
/// Returns a `Result` containing an `impl IntoResponse`, which represents the
/// HTTP response with the XLSX file as the body.
#[cfg(feature = "ssr")]
pub async fn generate_xlsx() -> anyhow::Result<impl IntoResponse> {
    use std::io::Cursor;

    use axum::response::Response;
    use blue_scout::data::DataPoint;
    use duckdb::arrow::datatypes::DataType;
    use reqwest::header::CONTENT_TYPE;
    use rust_xlsxwriter::{workbook::Workbook, Format};

    let conn = get_conn().await;

    let mut stmt = conn.prepare("SELECT * EXCLUDE(id) FROM scout_entries")?;

    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet().set_name("Data")?;

    let bold = Format::new().set_bold();

    worksheet.write_row_with_format(
        0,
        0,
        DataPoint::field_pretty_names()
            .iter()
            .map(|&(_, x)| x.to_owned()),
        &bold,
    )?;

    let mut current_row = 1;
    stmt.query_map([], |row| {
        for i in 0..row.as_ref().column_count() {
            let t: DataType = row.as_ref().column_type(i);
            let current_column = u16::try_from(i).expect("Current column should be u16");
            match t {
                DataType::Null => worksheet
                    .write_string(current_row, current_column, "NULL")
                    .expect("Writing to excel file should not fail"),
                DataType::Boolean => worksheet
                    .write_string(
                        current_row,
                        current_column,
                        if row.get::<_, bool>(i)? { "Yes" } else { "No" },
                    )
                    .expect("Writing to excel file should not fail"),
                DataType::Int8 | DataType::Int16 | DataType::Int32 | DataType::Int64 => worksheet
                    .write_number(current_row, current_column, row.get::<_, i64>(i)? as f64)
                    .expect("Writing to excel file should not fail"),
                DataType::UInt8 | DataType::UInt16 | DataType::UInt32 | DataType::UInt64 => {
                    worksheet
                        .write_number(current_row, current_column, row.get::<_, u64>(i)? as f64)
                        .expect("Writing to excel file should not fail")
                }
                DataType::Float16 | DataType::Float32 | DataType::Float64 => worksheet
                    .write_number(current_row, current_column, row.get::<_, f64>(i)?)
                    .expect("Writing to excel file should not fail"),
                DataType::Utf8 => worksheet
                    .write_string(current_row, current_column, row.get::<_, String>(i)?)
                    .expect("Writing to excel file should not fail"),
                _ => unimplemented!("Unsupported data type: {:?}", t),
            };
        }
        current_row += 1;
        Ok(())
    })?
    .count();

    drop(conn);

    worksheet.autofit_to_max_width(300);

    let mut buf = Cursor::new(Vec::new());
    workbook.save_to_writer(&mut buf)?;

    Ok(Response::builder()
        .header(
            CONTENT_TYPE,
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        )
        .body(Body::from(buf.into_inner()))?)
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    async fn handle_anyhow_error(err: anyhow::Error) -> (StatusCode, String) {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {err}"),
        )
    }

    use axum::{error_handling::HandleError, Router};
    use blue_scout::{
        app::{shell, App},
        db::init_db,
        API_CONFIG,
    };
    use dotenv::dotenv;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes as _};
    use reqwest::StatusCode;
    use tbaapi::apis::configuration::ApiKey;

    assert!(
        !(cfg!(feature = "ssr") && cfg!(feature = "hydrate")),
        "Both SSR and Hydration features are enabled! TODO: Fix"
    );

    tracing_subscriber::fmt::init();
    if dotenv().is_err() {
        tracing::warn!("No .env file found");
    }

    let config = Configuration {
        api_key: Some(ApiKey {
            prefix: None,
            key: std::env::var("TBA_API_KEY").expect("TBA_API_KEY must be set"),
        }),
        ..Configuration::default()
    };

    API_CONFIG.set(config).expect("This should not be set yet");

    init_team_names().await.expect("TODO: Handle error");

    init_db()
        .await
        .expect("DB should be able to be initialized");

    let conf = get_configuration(None).expect("Configuration should be set");
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
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Port should be free");
    axum::serve(listener, app.into_make_service())
        .await
        .expect("Server should start");
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
