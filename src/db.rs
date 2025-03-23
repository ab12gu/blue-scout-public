#![cfg(feature = "ssr")]
use std::collections::HashMap;

use axum::{http::StatusCode, Json};
use duckdb::{params, Connection, Result};
use once_cell::sync::OnceCell;
use reqwest::header;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
pub struct FormResponse {
    status: &'static str,
    message: &'static str,
}

static DB: OnceCell<Mutex<Connection>> = OnceCell::new();

pub fn init_db() -> rusqlite::Result<()> {
    let conn = Connection::open("scouting_data.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS scout_entries (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT,
            match_number TEXT,
            team_number TEXT,
            auto_algae TEXT,
            auto_coral_number TEXT,
            auto_leave TEXT CHECK (auto_leave IN ('Yes', 'No')),
            algae_clear TEXT CHECK (algae_clear IN ('Yes', 'No')),
            coral_l1 TEXT,
            coral_l2 TEXT,
            coral_l3 TEXT,
            coral_l4 TEXT,
            coral_dropped TEXT,
            algae_floor_hole TEXT,
            algae_barge TEXT,
            climb TEXT,
            defense_bot TEXT CHECK (defense_bot IN ('Yes', 'No')),
            notes TEXT
        );",
        (),
    )?;

    if DB.set(Mutex::new(conn)).is_err() {
        panic!("DB already initialized");
    };

    Ok(())
}

#[derive(Debug, Clone, Serialize)]
pub struct DataPoint {
    name: String,
    match_number: usize,
    team_number: usize,
    auto_algae_number: usize,
    auto_coral_number: usize,
    auto_leave: bool,
    algae_clear: bool,
    coral_l1: usize,
    coral_l2: usize,
    coral_l3: usize,
    coral_l4: usize,
    coral_dropped: usize,
    floor_hole: usize,
    barge: usize,
    climb: String,
    defense_bot: bool,
    notes: String,
}

fn map_datapoint(row: &Row<'_>) -> rusqlite::Result<DataPoint> {
    Ok(DataPoint {
        name: row.get(1)?,
        match_number: row.get::<_, String>(2)?.parse().unwrap(),
        team_number: row.get::<_, String>(3)?.parse().unwrap(),
        auto_algae_number: row.get::<_, String>(4)?.parse().unwrap(),
        auto_coral_number: row.get::<_, String>(5)?.parse().unwrap(),
        auto_leave: row.get::<_, bool>(6)?,
        algae_clear: row.get::<_, String>(7)? == "Yes",
        coral_l1: row.get::<_, String>(8)?.parse().unwrap(),
        coral_l2: row.get::<_, String>(9)?.parse().unwrap(),
        coral_l3: row.get::<_, String>(10)?.parse().unwrap(),
        coral_l4: row.get::<_, String>(11)?.parse().unwrap(),
        coral_dropped: row.get::<_, String>(12)?.parse().unwrap(),
        floor_hole: row.get::<_, String>(13)?.parse().unwrap(),
        barge: row.get::<_, String>(14)?.parse().unwrap(),
        climb: row.get(15)?,
        defense_bot: row.get::<_, String>(16)? == "Yes",
        notes: row.get(17)?,
    })
}

pub async fn get_data() -> std::result::Result<Json<Vec<DataPoint>>, (StatusCode, String)> {
    let db = DB.get().expect("Database not initialized");
    let conn = db.lock().await;

    let mut stmt = conn.prepare("SELECT * FROM scout_entries").unwrap();
    let entry_iter = stmt.query_map([], map_datapoint).unwrap();

    let data_points = entry_iter.collect::<Result<Vec<DataPoint>, _>>().unwrap();
    Ok(Json(data_points))
}

// Helper to safely extract string values from the JSON
fn get_string_value(data: &Value, key: &str) -> String {
    match data.get(key) {
        Some(value) => {
            if let Some(s) = value.as_str() {
                s.to_string()
            } else if let Some(n) = value.as_number() {
                n.to_string()
            } else if value.is_null() {
                "".to_string()
            } else {
                value.to_string()
            }
        }
        None => "".to_string(),
    }
}

#[inline]
pub fn extract_checkbox(value: Option<String>) -> bool {
    value.map(|x| x == "on").unwrap_or(false)
}

// Insert the form data into the SQLite database
async fn insert_form_data(
    name: String,
    match_number: u32,
    team_number: u32,
    auto_algae: u32,
    auto_coral: u32,
    auto_leave: Option<String>,
    algae_clear: Option<String>,
    l1_coral: u32,
    l2_coral: u32,
    l3_coral: u32,
    l4_coral: u32,
    dropped_coral: u32,
    algae_barge: u32,
    algae_floor_hole: u32,
    climb: String,
    defense_bot: Option<String>,
    notes: String,
) -> rusqlite::Result<()> {
    let db = DB.get().expect("Database not initialized");
    let conn = db.lock().await;

    let mut stmt = conn.prepare("INSERT INTO scout_entries (name, match_number, team_number, auto_algae, auto_coral, auto_leave, algae_clear, l1_coral, l2_coral, l3_coral, l4_coral, dropped_coral, algae_barge, algae_floor_hole, climb, defense_bot, notes) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")?;

    stmt.execute(params![
        name,
        match_number,
        team_number,
        auto_algae,
        auto_coral,
        extract_checkbox(auto_leave),
        extract_checkbox(algae_clear),
        l1_coral,
        l2_coral,
        l3_coral,
        l4_coral,
        dropped_coral,
        algae_barge,
        algae_floor_hole,
        climb,
        extract_checkbox(defense_bot),
        notes
    ])?;

    Ok(())
}

#[derive(Debug, Serialize)]
pub struct TeamInfo {
    number: usize,
    avg_coral: f64,
    avg_barge_algae: f64,
    avg_floor_algae: f64,
    score_l1: bool,
    score_l2: bool,
    score_l3: bool,
    score_l4: bool,
    sum_of_deep_climbs: u32,
    sum_of_climb_not_attempted: u32,
}

#[derive(Debug, Serialize)]
pub struct MatchInfo {
    red: Vec<TeamInfo>,
    blue: Vec<TeamInfo>,
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

pub async fn get_match_info(match_number: u32) -> Result<Json<MatchInfo>, anyhow::Error> {
    let mut headers = header::HeaderMap::new();
    headers.insert("accept", "application/json".parse()?);
    headers.insert("X-TBA-Auth-Key", std::env::var("TBA_API_KEY")?.parse()?);

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;
    let matches: Vec<Match> = serde_json::from_str(
        &client
            .get("https://www.thebluealliance.com/api/v3/event/2025wabon/matches/simple")
            .headers(headers)
            .send()
            .await?
            .text()
            .await?,
    )?;

    let target_match: &Match = matches
        .iter()
        .find(|x| x.match_number == match_number)
        .unwrap();

    let red_team: Vec<usize> = target_match
        .alliances
        .red
        .team_keys
        .iter()
        .map(|x| x.trim_start_matches("frc").parse::<usize>().unwrap())
        .collect();

    let blue_team: Vec<usize> = target_match
        .alliances
        .blue
        .team_keys
        .iter()
        .map(|x| x.trim_start_matches("frc").parse::<usize>().unwrap())
        .collect();

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
        map_datapoint,
    )?;

    let data_points = entry_iter.collect::<Result<Vec<DataPoint>, _>>()?;

    let mut team_data: HashMap<usize, Vec<DataPoint>> = HashMap::new();

    for data in data_points {
        team_data.entry(data.team_number).or_default().push(data);
    }

    for (team_number, data) in team_data {
        // Process data for each team
        // Example: Calculate average score for each team
        let avg_coral = data
            .iter()
            .map(|x| x.coral_l4 + x.coral_l3 + x.coral_l2 + x.coral_l1)
            .sum::<usize>() as f64
            / data.len() as f64;
        let avg_barge_algae =
            data.iter().map(|x| x.barge).sum::<usize>() as f64 / data.len() as f64;
        let avg_floor_algae =
            data.iter().map(|x| x.floor_hole).sum::<usize>() as f64 / data.len() as f64;
        let (score_l1, score_l2, score_l3, score_l4) = (
            data.iter().map(|x| x.coral_l1).any(|x| x > 0),
            data.iter().map(|x| x.coral_l2).any(|x| x > 0),
            data.iter().map(|x| x.coral_l3).any(|x| x > 0),
            data.iter().map(|x| x.coral_l4).any(|x| x > 0),
        );
        let sum_of_deep_climbs = data
            .iter()
            .map(|x| (x.climb == "Deep") as usize)
            .sum::<usize>();
        let sum_of_deep_climbs = data
            .iter()
            .map(|x| (x.climb == "Not Attempted") as usize)
            .sum::<usize>();
    }

    todo!()
}
