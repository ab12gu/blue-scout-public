//! # Blue Scout

#![feature(let_chains, extern_types)]
#![recursion_limit = "256"]

use core::sync::atomic::AtomicBool;

use chrono::NaiveDate;
use data::DataPoint;
use serde::{Deserialize, Serialize};
pub mod api;
pub mod app;
pub mod components;
pub mod data;
pub mod db;
mod error;
mod tablefilterjs;
pub use error::BlueScoutError;

#[cfg(feature = "ssr")]
use {
    frozen_collections::FzScalarMap, once_cell::sync::OnceCell, std::sync::OnceLock,
    tbaapi::apis::configuration::Configuration,
};

/// Global static variable to store the TBA API configuration.
/// Only available when the `ssr` feature is enabled.
#[cfg(feature = "ssr")]
pub static API_CONFIG: OnceCell<Configuration> = OnceCell::new();

/// Returns a reference to the global TBA API configuration.
///
/// # Panics
///
/// Panics if the configuration has not been initialized with
/// `API_CONFIG.set()`.
#[cfg(feature = "ssr")]
pub fn api_config() -> &'static Configuration {
    API_CONFIG.get().expect("API_CONFIG should have been set")
}

/// A constant string used as a key to indicate whether the Leptos application
/// has been hydrated.
pub const LEPTOS_HYDRATED: &str = "_leptos_hydrated";

/// Global static variable to store team names, mapping team numbers to team
/// names. Only available when the `ssr` feature is enabled.
#[cfg(feature = "ssr")]
pub static TEAM_NAMES: OnceLock<FzScalarMap<u32, String>> = OnceLock::new();

/// Global static variable to track whether the application has been hydrated.
pub static HYDRATED: AtomicBool = AtomicBool::new(false);

/// Hydrates the Leptos application on the client-side.
/// This function is only available when the `hydrate` feature is enabled.
///
/// # Panics
///
/// This function may panic if JavaScript reflection or event creation fails.
#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use core::sync::atomic::Ordering;

    use web_sys::js_sys;

    use crate::app::App;
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

/// Represents information about a team.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TeamInfo {
    /// The team number.
    team_number: u32,
    /// The team name (optional).
    team_name: Option<String>,
    /// The team's data points (optional).
    team_data: Option<Vec<DataPoint>>,
}

/// Represents information about a match.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MatchInfo {
    /// The predicted time of the match.
    predicted_time: i64,
    /// Information about the red alliance teams.
    red: [TeamInfo; 3],
    /// Information about the blue alliance teams.
    blue: [TeamInfo; 3],
}

/// Represents information about an event.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct EventInfo {
    /// The event key.
    key: String,
    /// The short name of the event.
    short_name: String,
    /// The start date of the event.
    start_date: NaiveDate,
    /// The end date of the event.
    end_date: NaiveDate,
}

impl MatchInfo {
    /// Creates an empty `MatchInfo` struct with default values.
    #[must_use]
    pub fn empty() -> Self {
        Self {
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
