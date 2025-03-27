#![cfg(feature = "ssr")]
use std::{collections::HashMap, str::FromStr};

use duckdb::{params, Connection, Row};
use once_cell::sync::OnceCell;
use reqwest::header;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::{ClimbType, DataPoint, MatchInfo, TeamData, TeamInfo, TEAM_NAMES};

pub static DB: OnceCell<Mutex<Connection>> = OnceCell::new();

pub fn init_db() -> duckdb::Result<()> {
    let conn = Connection::open("scouting_data.db")?;

    conn.execute("INSTALL excel;", [])?;
    conn.execute("LOAD excel;", [])?;

    conn.execute(
        "CREATE SEQUENCE IF NOT EXISTS scout_entries_id_seq START 1;",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS scout_entries (
            id INTEGER PRIMARY KEY DEFAULT nextval('scout_entries_id_seq'),
            name TEXT,
            match_number USMALLINT,
            team_number UINTEGER,
            auto_coral USMALLINT,
            auto_algae USMALLINT,
            auto_leave BOOL,
            algae_clear BOOL,
            l1_coral USMALLINT,
            l2_coral USMALLINT,
            l3_coral USMALLINT,
            l4_coral USMALLINT,
            dropped_coral USMALLINT,
            algae_barge USMALLINT,
            algae_floor_hole USMALLINT,
            climb TEXT,
            defense_bot BOOL,
            notes TEXT
        );",
        [],
    )?;

    if DB.set(Mutex::new(conn)).is_err() {
        panic!("DB already initialized");
    };

    Ok(())
}

fn map_datapoint(row: &Row<'_>) -> duckdb::Result<DataPoint> {
    Ok(DataPoint {
        name: row.get(1)?,
        match_number: row.get(2)?,
        team_number: row.get(3)?,
        auto_coral: row.get(4)?,
        auto_algae: row.get(5)?,
        auto_leave: row.get(6)?,
        algae_clear: row.get(7)?,
        l1_coral: row.get(8)?,
        l2_coral: row.get(9)?,
        l3_coral: row.get(10)?,
        l4_coral: row.get(11)?,
        dropped_coral: row.get(12)?,
        algae_barge: row.get(13)?,
        algae_floor_hole: row.get(14)?,
        climb: {
            let str = row.get::<_, String>(15)?;
            match ClimbType::from_str(&str) {
                Ok(climb_type) => climb_type,
                Err(_) => {
                    tracing::error!("Unknown Climb Type: {}", str);
                    ClimbType::Unknown
                }
            }
        },
        defense_bot: row.get(16)?,
        notes: row.get(17)?,
    })
}

macro_rules! data_point_to_sql {
    ($datapoint:expr) => {
        params![
            $datapoint.name,
            $datapoint.match_number,
            $datapoint.team_number,
            $datapoint.auto_algae,
            $datapoint.auto_coral,
            $datapoint.auto_leave,
            $datapoint.algae_clear,
            $datapoint.l1_coral,
            $datapoint.l2_coral,
            $datapoint.l3_coral,
            $datapoint.l4_coral,
            $datapoint.dropped_coral,
            $datapoint.algae_barge,
            $datapoint.algae_floor_hole,
            $datapoint.climb.to_string(),
            $datapoint.defense_bot,
            $datapoint.notes
        ]
    };
}

pub async fn get_data() -> std::result::Result<Vec<DataPoint>, anyhow::Error> {
    let db = DB.get().expect("Database not initialized");
    let conn = db.lock().await;

    let mut stmt = conn.prepare("SELECT * FROM scout_entries")?;
    let entry_iter = stmt.query_map([], map_datapoint)?;

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

    stmt.execute(data_point_to_sql!(data_point))?;

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
        map_datapoint,
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
        let avg_coral = data
            .iter()
            .map(|x| (x.l4_coral + x.l3_coral + x.l2_coral + x.l1_coral) as u32)
            .sum::<u32>() as f64
            / data.len() as f64;
        let avg_auto_coral =
            data.iter().map(|x| x.auto_coral as u32).sum::<u32>() as f64 / data.len() as f64;
        let avg_barge_algae =
            data.iter().map(|x| x.algae_barge as u32).sum::<u32>() as f64 / data.len() as f64;
        let avg_floor_algae =
            data.iter().map(|x| x.algae_floor_hole as u32).sum::<u32>() as f64 / data.len() as f64;
        let (score_l1, score_l2, score_l3, score_l4) = (
            data.iter().filter(|x| x.l1_coral > 0).count() as u32,
            data.iter().filter(|x| x.l2_coral > 0).count() as u32,
            data.iter().filter(|x| x.l3_coral > 0).count() as u32,
            data.iter().filter(|x| x.l4_coral > 0).count() as u32,
        );
        let sum_of_deep_climbs = data
            .iter()
            .map(|x| (x.climb == ClimbType::Deep) as usize)
            .sum::<usize>() as u32;
        let sum_of_climb_not_attempted = data
            .iter()
            .map(|x| (x.climb == ClimbType::NotAttempted) as usize)
            .sum::<usize>() as u32;
        let team_data = TeamData {
            avg_coral,
            avg_auto_coral,
            avg_barge_algae,
            avg_floor_algae,
            score_l1,
            score_l2,
            score_l3,
            score_l4,
            sum_of_deep_climbs,
            sum_of_climb_not_attempted,
        };
        if is_blue_team {
            match_info.blue[team_index] = TeamInfo {
                team_number,
                team_name,
                team_data: Some(team_data),
            };
        } else {
            match_info.red[team_index] = TeamInfo {
                team_number,
                team_name,
                team_data: Some(team_data),
            };
        }
    }

    match_info.predicted_time = target_match.predicted_time;

    Ok(match_info)
}
