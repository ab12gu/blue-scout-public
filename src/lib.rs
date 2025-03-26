#![feature(let_chains)]
#![recursion_limit = "256"]

use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};
//#![allow(dead_code, unused_variables)]
pub mod app;
pub mod components;
pub mod db;

#[cfg(feature = "ssr")]
pub static TEAM_NAMES: std::sync::OnceLock<Vec<Option<String>>> = std::sync::OnceLock::new();

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DataPoint {
    pub name: String,
    pub match_number: u16,
    pub team_number: u32,
    pub auto_coral: u16,
    pub auto_algae: u16,
    pub auto_leave: bool,
    pub algae_clear: bool,
    pub l1_coral: u16,
    pub l2_coral: u16,
    pub l3_coral: u16,
    pub l4_coral: u16,
    pub dropped_coral: u16,
    pub algae_barge: u16,
    pub algae_floor_hole: u16,
    pub climb: ClimbType,
    pub defense_bot: bool,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ClimbType {
    Deep,
    Shallow,
    Park,
    NotAttempted,
    Unknown,
}

impl Display for ClimbType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClimbType::Deep => write!(f, "Deep"),
            ClimbType::Shallow => write!(f, "Shallow"),
            ClimbType::Park => write!(f, "Park"),
            ClimbType::NotAttempted => write!(f, "Not Attempted"),
            ClimbType::Unknown => write!(f, "Unknown"),
        }
    }
}

impl FromStr for ClimbType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Deep" => Ok(ClimbType::Deep),
            "Shallow" => Ok(ClimbType::Shallow),
            "Park" => Ok(ClimbType::Park),
            "Not Attempted" => Ok(ClimbType::NotAttempted),
            _ => Err(format!("Invalid ClimbType: {}", s)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct TeamData {
    avg_coral: f64,
    avg_auto_coral: f64,
    avg_barge_algae: f64,
    avg_floor_algae: f64,
    score_l1: u32,
    score_l2: u32,
    score_l3: u32,
    score_l4: u32,
    sum_of_deep_climbs: u32,
    sum_of_climb_not_attempted: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TeamInfo {
    team_number: u32,
    team_name: Option<String>,
    team_data: Option<TeamData>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MatchInfo {
    predicted_time: u64,
    red: [TeamInfo; 3],
    blue: [TeamInfo; 3],
}

impl MatchInfo {
    pub fn empty() -> Self {
        MatchInfo {
            predicted_time: 0,
            red: core::array::from_fn(|_| TeamInfo {
                team_number: 0,
                team_name: None,
                team_data: None,
            }),
            blue: core::array::from_fn(|_| TeamInfo {
                team_number: 0,
                team_name: None,
                team_data: None,
            }),
        }
    }
}
