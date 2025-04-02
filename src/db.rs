#![cfg(feature = "ssr")]

use duckdb::Connection;
use once_cell::sync::OnceCell;
use tokio::sync::Mutex;

use crate::data::{DataPoint, DataTypeName};

pub static DB: OnceCell<Mutex<Connection>> = OnceCell::new();

pub async fn migrate_db() -> duckdb::Result<()> {
    let conn = DB.get().expect("Database not initialized").lock().await;

    // Get current columns in the table
    let mut stmt = conn
        .prepare("SELECT * FROM information_schema.columns WHERE table_name = 'scout_entries'")?;
    let existing_columns: Vec<String> = stmt
        .query_map([], |row| row.get::<_, String>(3))?
        .collect::<Result<Vec<_>, _>>()?;

    // Get expected columns from DataPoint's metadata
    for (column_name, data_type) in DataPoint::field_metadata() {
        if !existing_columns.contains(&column_name.to_string()) {
            let sql_type = match data_type {
                DataTypeName::U16 => "SMALLINT",
                DataTypeName::U32 => "INTEGER",
                DataTypeName::U64 => "BIGINT",
                DataTypeName::I16 => "SMALLINT",
                DataTypeName::I32 => "INTEGER",
                DataTypeName::I64 => "BIGINT",
                DataTypeName::String => "VARCHAR",
                DataTypeName::Bool => "BOOLEAN",
                DataTypeName::Float => "REAL",
            };
            let default_val = match data_type {
                DataTypeName::U16
                | DataTypeName::U32
                | DataTypeName::U64
                | DataTypeName::I16
                | DataTypeName::I32
                | DataTypeName::I64 => "DEFAULT 0",
                DataTypeName::String => "DEFAULT ''",
                DataTypeName::Bool => "DEFAULT FALSE",
                DataTypeName::Float => "DEFAULT 0.0",
            };

            let alter_sql = format!(
                "ALTER TABLE scout_entries ADD COLUMN {column_name} {sql_type} {default_val}",
            );
            conn.execute(&alter_sql, [])?;
        }
    }

    Ok(())
}

pub async fn init_db() -> duckdb::Result<()> {
    let conn = Connection::open("scouting_data.db")?;

    conn.execute("INSTALL excel;", [])?;
    conn.execute("LOAD excel;", [])?;

    conn.execute(
        "CREATE SEQUENCE IF NOT EXISTS scout_entries_id_seq START 1;",
        [],
    )?;

    conn.execute(DataPoint::get_create_table_sql(), [])?;

    if DB.set(Mutex::new(conn)).is_err() {
        panic!("DB already initialized");
    };

    migrate_db().await
}

pub async fn get_data() -> std::result::Result<Vec<DataPoint>, anyhow::Error> {
    let db = DB.get().expect("Database not initialized");
    let conn = db.lock().await;
    let mut stmt = conn.prepare("SELECT * FROM scout_entries")?;
    let entry_iter = stmt.query_map([], DataPoint::map_datapoint)?;

    let data_points = entry_iter.collect::<Result<Vec<DataPoint>, _>>()?;
    Ok(data_points)
}

#[inline]
pub fn extract_checkbox(value: Option<String>) -> bool {
    value.map(|x| x == "on").unwrap_or(false)
}

// Insert the form data into the SQLite database
pub async fn insert_form_data(data_point: DataPoint) -> duckdb::Result<()> {
    let db = DB.get().expect("Database not initialized");
    let conn = db.lock().await;

    let mut stmt = conn.prepare("INSERT INTO scout_entries (name, match_number, team_number, auto_algae, auto_coral, auto_leave, algae_clear, l1_coral, l2_coral, l3_coral, l4_coral, dropped_coral, algae_barge, algae_floor_hole, climb, defense_bot, notes) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")?;

    stmt.execute(data_point.to_sql())?;

    Ok(())
}
