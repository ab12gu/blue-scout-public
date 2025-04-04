#![cfg(feature = "ssr")]

use std::collections::HashMap;

use chrono::Datelike;

use serde::{Deserialize, Serialize};
use tbaapi::{
    apis::{event_api::get_events_by_year, match_api::get_event_matches_simple},
    models::{match_simple::CompLevel, Event},
};

use crate::{api_config, data::DataPoint, db::DB, MatchInfo, TeamInfo, TEAM_NAMES};

#[derive(Debug, Serialize, Deserialize)]
struct Match {
    actual_time: Option<u64>,
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

pub async fn get_match_info(match_number: i32, event: &str) -> Result<MatchInfo, anyhow::Error> {
    let matches = get_event_matches_simple(api_config(), event).await?;

    let target_match = matches
        .iter()
        .find(|x| x.match_number == match_number && x.comp_level == CompLevel::Qm)
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

    match_info.predicted_time = target_match.predicted_time.unwrap_or(0) as u64;

    Ok(match_info)
}

pub async fn get_frc_events() -> Result<Vec<Event>, anyhow::Error> {
    Ok(get_events_by_year(api_config(), chrono::Utc::now().year()).await?)
}
