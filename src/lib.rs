#![feature(let_chains, extern_types)]
#![recursion_limit = "256"]

pub const LEPTOS_HYDRATED: &str = "_leptos_hydrated";

use std::sync::atomic::AtomicBool;

use chrono::NaiveDate;
use data::DataPoint;
use serde::{Deserialize, Serialize};
//#![allow(dead_code, unused_variables)]
pub mod api;
pub mod app;
pub mod components;
pub mod data;
pub mod db;
mod tablefilterjs;

#[cfg(feature = "ssr")]
pub static TEAM_NAMES: std::sync::OnceLock<Vec<Option<String>>> = std::sync::OnceLock::new();

pub static HYDRATED: AtomicBool = AtomicBool::new(false);

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use std::sync::atomic::Ordering;

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
    HYDRATED.store(true, Ordering::Relaxed);
    leptos::logging::log!("dispatched hydrated event");
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TeamInfo {
    team_number: u32,
    team_name: Option<String>,
    team_data: Option<Vec<DataPoint>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MatchInfo {
    predicted_time: u64,
    red: [TeamInfo; 3],
    blue: [TeamInfo; 3],
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EventInfo {
    key: String,
    short_name: String,
    start_date: NaiveDate,
    end_date: NaiveDate,
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
