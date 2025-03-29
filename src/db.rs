#![cfg(feature = "ssr")]
use std::collections::HashMap;

use duckdb::Connection;
use once_cell::sync::OnceCell;
use reqwest::header;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::{
    data::{DataPoint, DataTypeName},
    MatchInfo, TeamInfo, TEAM_NAMES,
};

pub static DB: OnceCell<Mutex<Connection>> = OnceCell::new();

pub async fn migrate_db() -> duckdb::Result<()> {
    let conn = DB.get().expect("Database not initialized").lock().await;

    // Get current columns in the table
    let mut stmt = conn
        .prepare("SELECT * FROM information_schema.columns WHERE table_name = 'scout_entries'")?;
    let existing_columns: Vec<String> = stmt
        .query_map([], |row| Ok(row.get::<_, String>(3)?))?
        .collect::<Result<Vec<_>, _>>()?;

    // Get expected columns from DataPoint's metadata
    for (column_name, data_type) in DataPoint::get_columns() {
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

            let alter_sql = format!(
                "ALTER TABLE scout_entries ADD COLUMN {} {}",
                column_name, sql_type
            );
            conn.execute(&alter_sql, [])?;
        }
    }

    Ok(())
}

pub fn init_db() -> duckdb::Result<()> {
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

    Ok(())
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

#[derive(Debug, Serialize, Deserialize)]
struct Match {
    actual_time: u64,
    alliances: Alliances,
    comp_level: String,
    event_key: String,
    key: String,
    match_number: u32,
    predicted_time: u64,
    set_number: u32,
    time: u64,
    winning_alliance: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Alliances {
    blue: Alliance,
    red: Alliance,
}

#[derive(Debug, Serialize, Deserialize)]
struct Alliance {
    dq_team_keys: Vec<String>,
    score: u32,
    surrogate_team_keys: Vec<String>,
    team_keys: Vec<String>,
}

pub async fn get_match_info(match_number: u32, event: &str) -> Result<MatchInfo, anyhow::Error> {
    let mut headers = header::HeaderMap::new();
    headers.insert("accept", "application/json".parse()?);
    headers.insert("X-TBA-Auth-Key", std::env::var("TBA_API_KEY")?.parse()?);

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;
    let matches: Vec<Match> = serde_json::from_str(
        &client
            .get(format!(
                "https://www.thebluealliance.com/api/v3/event/{}/matches/simple",
                event
            ))
            .headers(headers)
            .send()
            .await?
            .text()
            .await?,
    )?;

    let target_match: &Match = matches
        .iter()
        .find(|x| x.match_number == match_number && x.comp_level == "qm")
        .unwrap();

    let red_team: Vec<usize> = target_match
        .alliances
        .red
        .team_keys
        .iter()
        .map(|x| x.trim_start_matches("frc").parse::<usize>().unwrap())
        .collect();

    if red_team.len() != 3 {
        return Err(anyhow::anyhow!(
            "Invalid red team data. Expected 3 teams but found {}",
            red_team.len()
        ));
    }

    let blue_team: Vec<usize> = target_match
        .alliances
        .blue
        .team_keys
        .iter()
        .map(|x| x.trim_start_matches("frc").parse::<usize>().unwrap())
        .collect();

    if blue_team.len() != 3 {
        return Err(anyhow::anyhow!(
            "Invalid blue team data. Expected 3 teams but found {}",
            blue_team.len()
        ));
    }

    let db = DB.get().expect("Database not initialized");
    let conn = db.lock().await;

    let mut stmt = conn
    .prepare("SELECT * FROM scout_entries WHERE team_number = ?1 OR team_number = ?2 OR team_number = ?3 OR team_number = ?4 OR team_number = ?5 OR team_number = ?6")?;
    let entry_iter = stmt.query_map(
        [
            &red_team[0],
            &red_team[1],
            &red_team[2],
            &blue_team[0],
            &blue_team[1],
            &blue_team[2],
        ],
        DataPoint::map_datapoint,
    )?;

    let data_points = entry_iter.collect::<Result<Vec<DataPoint>, _>>()?;

    let mut team_data: HashMap<u32, Vec<DataPoint>> = [
        (red_team[0] as u32, Vec::new()),
        (red_team[1] as u32, Vec::new()),
        (red_team[2] as u32, Vec::new()),
        (blue_team[0] as u32, Vec::new()),
        (blue_team[1] as u32, Vec::new()),
        (blue_team[2] as u32, Vec::new()),
    ]
    .into();

    for data in data_points {
        team_data.get_mut(&data.team_number).unwrap().push(data);
    }

    let mut match_info = MatchInfo::empty();

    for (team_number, data) in team_data {
        let is_blue_team = blue_team.contains(&(team_number as usize));
        let team_index = if is_blue_team {
            blue_team
                .iter()
                .position(|&x| x == team_number as usize)
                .unwrap()
        } else {
            red_team
                .iter()
                .position(|&x| x == team_number as usize)
                .unwrap()
        };
        let team_name = TEAM_NAMES
            .get()
            .unwrap()
            .get(team_number as usize)
            .cloned()
            .flatten();
        if data.is_empty() {
            if is_blue_team {
                match_info.blue[team_index] = TeamInfo {
                    team_number,
                    team_name,
                    team_data: None,
                };
            } else {
                match_info.red[team_index] = TeamInfo {
                    team_number,
                    team_name,
                    team_data: None,
                };
            }
            continue;
        }

        if is_blue_team {
            match_info.blue[team_index] = TeamInfo {
                team_number,
                team_name,
                team_data: Some(data),
            };
        } else {
            match_info.red[team_index] = TeamInfo {
                team_number,
                team_name,
                team_data: Some(data),
            };
        }
    }

    match_info.predicted_time = target_match.predicted_time;

    Ok(match_info)
}
