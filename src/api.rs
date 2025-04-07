//! This module defines API endpoints for fetching match and event data.
//!
//! It uses the `tbaapi` crate to interact with The Blue Alliance (TBA) API
//! and the `duckdb` crate to query a local database for scouting data.

#![cfg(feature = "ssr")]

use std::collections::HashMap;

use chrono::Datelike as _;
use frozen_collections::MapQuery as _;
use tbaapi::{
    apis::{event_api::get_events_by_year, match_api::get_event_matches_simple},
    models::{match_simple::CompLevel, Event},
};

use crate::{api_config, data::DataPoint, db::DB, BlueScoutError, MatchInfo, TeamInfo, TEAM_NAMES};

/// Fetches match information for a given match number and event.
///
/// # Arguments
///
/// * `match_number` - The match number to fetch information for.
/// * `event` - The event key.
///
/// # Returns
///
/// A `Result` containing `MatchInfo` on success or `BlueScoutError` on failure.
///
/// # Errors
///
/// This function returns an error if the match number is not found or if there
/// is an issue with the database connection or api.
///
/// # Panics
///
/// Panics if the database connection is not initialized or if the team numbers
/// are not integers.
pub async fn get_match_info(match_number: i32, event: &str) -> Result<MatchInfo, BlueScoutError> {
    let matches = get_event_matches_simple(api_config(), event)
        .await
        .map_err(BlueScoutError::api_error)?;

    let target_match = matches
        .iter()
        .find(|x| x.match_number == match_number && x.comp_level == CompLevel::Qm)
        .ok_or_else(|| anyhow::anyhow!("Match number not found"))?;

    let red_team: Vec<usize> = target_match
        .alliances
        .red
        .team_keys
        .iter()
        .map(|x| {
            x.trim_start_matches("frc")
                .parse::<usize>()
                .expect("Team number should be a number")
        })
        .collect();

    if red_team.len() != 3 {
        return Err(BlueScoutError::api_error(format!(
            "Invalid red team data. Expected 3 teams but found {}",
            red_team.len()
        )));
    }

    let blue_team: Vec<usize> = target_match
        .alliances
        .blue
        .team_keys
        .iter()
        .map(|x| {
            x.trim_start_matches("frc")
                .parse::<usize>()
                .expect("Team number should be a number")
        })
        .collect();

    if blue_team.len() != 3 {
        return Err(BlueScoutError::api_error(format!(
            "Invalid blue team data. Expected 3 teams but found {}",
            blue_team.len()
        )));
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

    drop(conn);

    let err_map = |_| anyhow::anyhow!("Team number shouldn't be larger than 32 bits");

    let mut team_data: HashMap<u32, Vec<DataPoint>> = [
        (u32::try_from(red_team[0]).map_err(err_map)?, Vec::new()),
        (u32::try_from(red_team[1]).map_err(err_map)?, Vec::new()),
        (u32::try_from(red_team[2]).map_err(err_map)?, Vec::new()),
        (u32::try_from(blue_team[0]).map_err(err_map)?, Vec::new()),
        (u32::try_from(blue_team[1]).map_err(err_map)?, Vec::new()),
        (u32::try_from(blue_team[2]).map_err(err_map)?, Vec::new()),
    ]
    .into();

    for data in data_points {
        team_data
            .get_mut(&data.team_number)
            .expect("Team number should have been inserted earlier")
            .push(data);
    }

    let mut match_info = MatchInfo::empty();

    for (team_number, data) in team_data {
        let is_blue_team = blue_team.contains(&(team_number as usize));
        let team_index = if is_blue_team {
            blue_team
                .iter()
                .position(|&x| x == team_number as usize)
                .expect("team number should have been inserted earlier")
        } else {
            red_team
                .iter()
                .position(|&x| x == team_number as usize)
                .expect("team number should have been inserted earlier")
        };
        let team_name = TEAM_NAMES
            .get()
            .expect("TEAM_NAMES should have been initialized")
            .get(&team_number)
            .cloned();
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

    match_info.predicted_time = target_match.predicted_time.unwrap_or(0);

    Ok(match_info)
}

/// Fetches the list of FRC events for the current year.
///
/// # Returns
///
/// A `Result` containing a vector of `Event` on success or `anyhow::Error` on
/// failure.
///
/// # Errors
///
/// This function returns an error if the API request fails.
pub async fn get_frc_events() -> Result<Vec<Event>, anyhow::Error> {
    Ok(get_events_by_year(api_config(), chrono::Utc::now().year()).await?)
}
