#![feature(let_chains)]
#![recursion_limit = "256"]

pub const LEPTOS_HYDRATED: &str = "_leptos_hydrated";

use serde::{Deserialize, Serialize};
//#![allow(dead_code, unused_variables)]
pub mod app;
pub mod components;
pub mod data;
pub mod db;

#[cfg(feature = "ssr")]
pub static TEAM_NAMES: std::sync::OnceLock<Vec<Option<String>>> = std::sync::OnceLock::new();

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use web_sys::js_sys;

    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);

    let window = leptos::prelude::window();
    js_sys::Reflect::set(
        &window,
        &wasm_bindgen::JsValue::from_str(LEPTOS_HYDRATED),
        &wasm_bindgen::JsValue::TRUE,
    )
    .expect("error setting hydrated status");
    let event = web_sys::Event::new(LEPTOS_HYDRATED).expect("error creating hydrated event");
    let document = leptos::prelude::document();
    document
        .dispatch_event(&event)
        .expect("error dispatching hydrated event");
    leptos::logging::log!("dispatched hydrated event");
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
