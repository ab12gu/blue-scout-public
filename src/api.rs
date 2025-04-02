#![cfg(feature = "ssr")]

use std::collections::HashMap;

use chrono::{Date, Datelike, NaiveDate, Utc};
use reqwest::header;
use serde::{Deserialize, Serialize};

use crate::{data::DataPoint, db::DB, EventInfo, MatchInfo, TeamInfo, TEAM_NAMES};

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

// let now = chrono::Utc::now().date_naive();

// let all_events_past = events.iter().all(|x| x.end_date > now);

// let current_event = events
//     .iter()
//     .find(|x| now >= x.start_date && now <= x.end_date);

// if all_events_past {
//     let most_recent_event = events.iter().max_by_key(|x| x.end_date).unwrap();
//     return Ok(most_recent_event.clone());
// } else if let Some(current_event) = current_event {
//     return Ok(current_event.clone());
// } else {
//     let next_event = events
//         .iter()
//         .filter(|x| x.start_date > now)
//         .min_by_key(|x| x.start_date)
//         .unwrap();
//     return Ok(next_event.clone());
// }

pub async fn get_frc_events() -> Result<Vec<EventInfo>, anyhow::Error> {
    let mut headers = header::HeaderMap::new();
    headers.insert("accept", "application/json".parse()?);
    headers.insert("X-TBA-Auth-Key", std::env::var("TBA_API_KEY")?.parse()?);

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;
    let events: Vec<EventInfo> = serde_json::from_str(
        &client
            .get(format!(
                "https://www.thebluealliance.com/api/v3/events/{}",
                chrono::Utc::now().year()
            ))
            .headers(headers)
            .send()
            .await?
            .text()
            .await?,
    )?;

    Ok(events)
}
