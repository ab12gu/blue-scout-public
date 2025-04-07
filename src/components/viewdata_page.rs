//! Provide the `ViewDataPage` component to view scouting data and match
//! information.

#![allow(clippy::needless_return, clippy::missing_docs_in_private_items)]

#[cfg(feature = "ssr")]
#[allow(unused_imports)]
use crate::api_config;

use chrono::{DateTime, Local};
use leptos::{ev, prelude::*, task::spawn_local};
use web_sys::{window, Event, HtmlInputElement};

use crate::{components::PageWrapper, data::DataPoint, BlueScoutError, MatchInfo};

/// Default match number to display when the page loads.
const DEFAULT_MATCH: u32 = 1;

/// Enables debug mode for the next match feature.
const DEBUG_NEXT_MATCH: bool = true;

#[cfg(feature = "hydrate")]
mod jsalert {
    //! Rust bindings for `SweetAlert2`.
    use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

    #[wasm_bindgen]
    extern "C" {
        /// Calls the SweetAlert2 `fire` function with the given options.
        #[wasm_bindgen(js_namespace = Swal)]
        pub fn fire(options: &JsValue) -> js_sys::Promise;
    }
}

/// Displays an error message using `SweetAlert2`.
///
/// # Arguments
///
/// * `title` - The title of the error message.
/// * `text` - The text of the error message.
///
/// # Returns
///
/// A `js_sys::Promise` that resolves when the alert is closed.
fn show_error(title: &str, text: &str) -> js_sys::Promise {
    #[cfg(feature = "hydrate")]
    {
        use js_sys::{Object, Reflect};
        use jsalert::fire;
        use web_sys::{Document, Window};
        let data_theme = window()
            .and_then(|window: Window| window.document())
            .and_then(|document: Document| document.document_element())
            .and_then(|html| html.get_attribute("data-theme"))
            .unwrap_or_else(|| "light".to_owned());

        let options = Object::new();
        Reflect::set(&options, &"icon".into(), &"error".into())
            .expect("Target should be an object");
        Reflect::set(&options, &"title".into(), &title.into()).expect("Target should be an object");
        Reflect::set(&options, &"text".into(), &text.into()).expect("Target should be an object");
        Reflect::set(&options, &"theme".into(), &data_theme.into())
            .expect("Target should be an object");

        fire(&options)
    }
    #[cfg(not(feature = "hydrate"))]
    unreachable!("This should be called on the client");
}

/// Fetches the next match number for a given team at a specific event.
///
/// # Arguments
///
/// * `team_number` - The team number to fetch the next match for.
/// * `event` - The event to fetch the next match for.
///
/// # Returns
///
/// A `Result` containing an `Option` with the next match number or a
/// `BlueScoutError`.
#[server]
pub async fn next_team_match(
    team_number: u32,
    event: String,
) -> Result<Option<u32>, BlueScoutError> {
    #[cfg(feature = "ssr")]
    return {
        use tbaapi::apis::team_api::get_team_event_matches;
        Ok(
            get_team_event_matches(api_config(), &format!("frc{team_number}"), &event)
                .await
                .map_err(BlueScoutError::api_error)?
                .iter()
                .filter(|&x| x.actual_time.is_none() || DEBUG_NEXT_MATCH)
                .map(|x| {
                    u32::try_from(x.match_number).expect("Match number should not be negative")
                })
                .min(),
        )
    };
    #[cfg(not(feature = "ssr"))]
    {
        tracing::error!("Server function called without ssr feature enabled");
        unreachable!("This should only be called on the server");
    }
}

/// Fetches match data for a given match number and event.
///
/// # Arguments
///
/// * `match_number` - The match number to fetch data for.
/// * `event` - The event to fetch match data for.
///
/// # Returns
///
/// A `Result` containing `MatchInfo` or a `BlueScoutError`.
#[server(endpoint = "fetch_match_data")]
pub async fn fetch_match_data(
    match_number: i32,
    event: String,
) -> Result<MatchInfo, BlueScoutError> {
    #[cfg(feature = "ssr")]
    return {
        use crate::api::get_match_info;
        get_match_info(match_number, &event).await
    };
    #[cfg(not(feature = "ssr"))]
    {
        tracing::error!("Server function called without ssr feature enabled");
        unreachable!("This should only be called on the server");
    }
}

/// Fetches scouting data from the database.
///
/// # Returns
///
/// A `Result` containing a `Vec` of `DataPoint` or a `BlueScoutError`.
#[server(endpoint = "fetch_scouting_data")]
pub async fn fetch_scouting_data() -> Result<Vec<DataPoint>, BlueScoutError> {
    #[cfg(feature = "ssr")]
    {
        use crate::db::get_data as get_db_data;
        return get_db_data().await.map_err(BlueScoutError::database_error);
    }
    #[cfg(not(feature = "ssr"))]
    {
        tracing::error!("Server function called without ssr feature enabled");
        unreachable!("This should only be called on the server");
    }
}

/// Displays a boolean value as a string.
///
/// # Arguments
///
/// * `b` - The boolean value to display.
///
/// # Returns
///
/// A `String` representing the boolean value.
#[inline]
fn display_bool(b: bool) -> String {
    if b {
        "Yes".to_owned()
    } else {
        "No".to_owned()
    }
}

/// Macro to generate a view for team data.
///
/// # Arguments
///
/// * `$current_match` - The current match data.
/// * `$team` - The team (red or blue).
/// * `$index` - The index of the team
macro_rules! team_data_view {
    ($current_match:expr, $team:ident, $index:expr) => {
        move || match $current_match.get() {
            Some(Some(match_data)) => {
                let team_data = &match_data.$team[$index];
                team_data.team_data.as_ref().map_or_else(
                    || view! { <span class = "team-number"> No stats available </span> }.into_any(),
                    |data| DataPoint::view_team_data(data),
                )
            }
            Some(None) => view! {
                <span>Error loading stats...</span>
            }
            .into_any(),
            None => view! {
                <span>Loading stats...</span>
            }
            .into_any(),
        }
    };
}

/// Macro to generate a view for team numbers.
///
/// # Arguments
///
/// * `$current_match` - The current match data.
/// * `$team` - The team (red or blue).
/// * `$index` - The index of the team data.
macro_rules! team_number_view {
    ($current_match:expr, $team:ident, $index:expr) => {
        move || {
            let content = match $current_match.get() {
                Some(Some(match_data)) => {
                    let team_name = match_data.$team[$index]
                        .team_name
                        .as_ref()
                        .map(String::as_str);
                    format!(
                        "{}{}",
                        match_data.$team[$index].team_number,
                        if let Some(team_name) = team_name {
                            format!("  \"{}\"", team_name)
                        } else {
                            String::new()
                        }
                    )
                }
                Some(None) => "Error loading data...".to_owned(),
                None => "Loading data...".to_owned(),
            };
            view! {
                <span class="team-number">{content}</span>
            }
            .into_any()
        }
    };
}

/// Formats a timestamp into a human-readable string.
///
/// # Arguments
///
/// * `timestamp` - The timestamp to format.
///
/// # Returns
///
/// A `String` representing the formatted timestamp.
fn format_timestamp(timestamp: i64) -> String {
    let datetime = DateTime::from_timestamp(timestamp, 0)
        .expect("DateTime should be valid")
        .with_timezone(&Local);
    datetime.format("%a %-I:%M %p").to_string()
}

/// Component to display the ViewDataPage.
#[component]
pub fn ViewDataPage() -> impl IntoView {
    let (use_full_names, set_use_full_names) = signal(false);

    let (current_event, set_current_event) = signal(None::<String>);

    let (team_number, set_team_number) = signal(None::<String>);

    let (current_match_num, set_current_match_num) = signal(DEFAULT_MATCH);

    // Initialize values from localStorage on component mount
    Effect::new(move |_| {
        if let Some(window) = window() {
            if let Ok(storage) = window.local_storage()
                && let Some(storage) = storage
            {
                // Get saved team number
                if let Ok(saved_team_number) = storage.get_item("teamNumber") {
                    set_team_number(Some(saved_team_number.unwrap_or_default()));
                }

                // Get saved event id
                if let Ok(saved_event) = storage.get_item("currentEvent") {
                    set_current_event(Some(saved_event.unwrap_or_default()));
                }
            }
        }
    });

    let column_names = move || {
        if use_full_names() {
            DataPoint::field_pretty_names()
                .iter()
                .map(|&(_name, pretty_name)| view! { <th>{pretty_name.to_owned()}</th> })
                .collect_view()
        } else {
            DataPoint::reduced_column_names()
                .iter()
                .map(|&name| view! { <th>{name.to_owned()}</th> })
                .collect_view()
        }
    };

    let data = Resource::new(move || use_full_names.get(), move |_| fetch_scouting_data());
    let current_match = Resource::new(
        move || (current_event.get(), current_match_num.get()),
        move |(current_event, current_match_num)| async move {
            fetch_match_data(
                i32::try_from(current_match_num)
                    .expect("Current match number should fit into a 32 bit signed integer"),
                current_event.unwrap_or_default(),
            )
            .await
            .ok()
        },
    );

    #[cfg(feature = "hydrate")]
    let mut table_filter = {
        use crate::tablefilterjs::TableFilter;
        None::<TableFilter>
    };

    #[cfg(feature = "hydrate")]
    let mut init_table_filters = move |destroy_old: bool| {
        use crate::data::FilterType;
        use crate::tablefilterjs::TableFilter;
        use blue_scout_macros::js_json;
        use js_sys::Reflect;
        use wasm_bindgen::{JsCast as _, JsValue};
        use web_sys::{window, HtmlTableElement};

        if destroy_old && table_filter.is_some() {
            let tf = table_filter
                .take()
                .expect("Table filter should exist as it has already been checked");
            tf.destroy();
        }

        let document = window()
            .expect("Window should exist")
            .document()
            .expect("Document should exist");
        let table = document
            .get_element_by_id("scouting_data_table")
            .expect("Table element should exist")
            .dyn_into::<HtmlTableElement>()
            .expect("Element should be a table");

        let base_options = js_json!({
            "base_path": "tablefilter/",
            "sticky_headers": true,
            "rows_counter": true,
            "flt_css_class": "input input-primary",
            "div_checklist_css_class": "card bg-base-100 border border-primary border-solid",
            "checklist_css_class": "m-[10]",
            "clear_filter_text": "None",
            "enable_checklist_reset_filter": false,
            "checklist_selected_item_css_class": "text-neutral-50",
            "themes": [
              {
                "name": "transparent",
              },
            ],
        });

        for (i, &(_, filter_type)) in (if use_full_names.get_untracked() {
            DataPoint::field_filter_types()
        } else {
            DataPoint::field_filter_types_reduced()
        })
        .iter()
        .enumerate()
        {
            if filter_type != FilterType::Normal {
                Reflect::set(
                    &base_options,
                    &JsValue::from_str(&format!("col_{i}")),
                    &JsValue::from_str(&filter_type.to_string()),
                )
                .expect("Base options should be an object");
            }
        }

        let tf = TableFilter::new(&table, &base_options);
        tf.init();
        table_filter = Some(tf);
    };

    Effect::watch(
        move || use_full_names.get(),
        move |_, _, _| {
            #[cfg(feature = "hydrate")]
            {
                init_table_filters(true);
            }
            #[cfg(feature = "ssr")]
            {
                unreachable!("Effects should be used in the client-side only");
            }
        },
        true,
    );

    let update_match_number = move |ev: Event| {
        let value = event_target_value(&ev);
        let el: HtmlInputElement = event_target(&ev);
        if !value.is_empty() {
            // Parse the current value as a number
            if let Ok(num_val) = value.parse::<usize>() {
                // Get min and max attributes if they exist
                let min = el.min().parse::<usize>().ok();
                let max = el.max().parse::<usize>().ok();

                // Enforce minimum value if present
                if let Some(min_val) = min {
                    if num_val < min_val {
                        el.set_value(&min_val.to_string());
                        return;
                    }
                }

                // Enforce maximum value if present
                if let Some(max_val) = max {
                    if num_val > max_val {
                        el.set_value(&max_val.to_string());
                        return;
                    }
                }
            } else {
                // If the value isn't a valid number, reset to empty or min
                if let Ok(min_val) = el.min().parse::<usize>() {
                    el.set_value(&min_val.to_string());
                } else {
                    el.set_value("");
                }
            }
        }
        set_current_match_num.set(el.value().parse().unwrap_or(1));
        println!("{}", current_match_num.get());
    };

    let prevent_invalid_input = move |ev: ev::KeyboardEvent| {
        if ev.key() == "-" || ev.key() == "." {
            ev.prevent_default();
        }
    };

    let set_next_team_match = move |_: ev::MouseEvent| {
        spawn_local(async move {
            if let Ok(team_number) = team_number.get_untracked().unwrap_or_default().parse() {
                if let Ok(match_num) = next_team_match(
                    team_number,
                    current_event.get_untracked().unwrap_or_default(),
                )
                .await
                    && let Some(match_num) = match_num
                {
                    set_current_match_num(match_num);
                }
            } else {
                let _ = show_error(
                    "Error",
                    "Team Number needs to be set in settings for this feature to work!",
                );
            }
        });
    };

    view! {
        <Suspense>
            <script src="/tablefilter/tablefilter.js"></script>
            <script src="/sweetalert2.min.js"></script>
        </Suspense>
        <PageWrapper>
            <div class="container mx-auto">
                <h1 class="text-3xl font-bold text-center mb-8">View Scouting Data</h1>
                <div class="card bg-base-200 shadow-xl">
                    <div class="card-body p-8">
                        <div class="overflow-x-auto">
                            <label class="label-text text-lg font-medium" for="fullDataCheckbox">
                                Show Full Data
                            </label>
                            <input
                                class="checkbox checkbox-primary"
                                type="checkbox"
                                id="fullDataCheckbox"
                                name="fullDataCheckbox"
                                on:input=move |ev| {
                                    set_use_full_names(event_target_checked(&ev));
                                }
                            />
                            <br />
                            <br />
                            <table class="table" id="scouting_data_table">
                                <thead>
                                    <tr>{move || column_names()}</tr>
                                </thead>
                                <tbody>
                                    <Suspense fallback=move || {
                                        view! { <span>Loading...</span> }
                                    }>
                                        {move || match *data.read() {
                                            Some(Ok(ref items)) => {
                                                items
                                                    .iter()
                                                    .map(|item| {
                                                        view! {
                                                            <tr class="hover:bg-base-300">
                                                                {if use_full_names.get() {
                                                                    DataPoint::field_names()
                                                                        .iter()
                                                                        .map(|name| {
                                                                            use crate::data::DataType;
                                                                            let value = item
                                                                                .get_field(name)
                                                                                .expect("Field should exist");
                                                                            let string_value = match value {
                                                                                DataType::U16(val) => val.to_string(),
                                                                                DataType::U32(val) => val.to_string(),
                                                                                DataType::U64(val) => val.to_string(),
                                                                                DataType::I16(val) => val.to_string(),
                                                                                DataType::I32(val) => val.to_string(),
                                                                                DataType::I64(val) => val.to_string(),
                                                                                DataType::String(val) => val,
                                                                                DataType::Bool(val) => display_bool(val),
                                                                                DataType::Float(val) => format!("{val:.2}"),
                                                                            };
                                                                            view! { <td>{string_value}</td> }
                                                                        })
                                                                        .collect_view()
                                                                        .into_any()
                                                                } else {
                                                                    item.get_reduced_columns()
                                                                        .iter()
                                                                        .cloned()
                                                                        .map(|(_, value)| {
                                                                            view! { <td>{value}</td> }
                                                                        })
                                                                        .collect_view()
                                                                        .into_any()
                                                                }}
                                                            </tr>
                                                        }
                                                    })
                                                    .collect_view()
                                                    .into_any()
                                            }
                                            Some(Err(_)) => {
                                                view! {
                                                    <tr>
                                                        <td>Error loading data...</td>
                                                    </tr>
                                                }
                                                    .into_any()
                                            }
                                            None => {
                                                view! {
                                                    <tr>
                                                        <td>Loading...</td>
                                                    </tr>
                                                }
                                                    .into_any()
                                            }
                                        }}
                                    </Suspense>
                                </tbody>
                            </table>
                            <br />
                            <div class="flex justify-center">
                                <a
                                    href="/download-xlsx"
                                    class="btn btn-primary"
                                    download="data.xlsx"
                                >
                                    Download Spreadsheet
                                </a>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            <div class="container mx-auto mt-[69px]">
                <h1 class="text-3xl font-bold text-center mb-8">View Match</h1>
                <Suspense>
                    {move || {
                        if current_event.get().is_none() {
                            view! { <h2 class="text-2xl text-center mb-16">Loading...</h2> }
                                .into_any()
                        } else if current_event.get().is_some_and(|x| x.is_empty()) {
                            view! {
                                <h2 class="text-2xl text-center mb-16 text-error">
                                    Event Name needs to be set in settings for this feature to work!
                                </h2>
                            }
                                .into_any()
                        } else {
                            view! {
                                <div class="card bg-base-200 shadow-xl">
                                    <div class="card-body p-8">
                                        <div class="card-body p-8">
                                            <div class="flex flex-col sm:flex-row gap-4 justify-center">
                                                <div class="flex-1">
                                                    <h2 class="text-xl font-bold text-center text-error mb-4">
                                                        Red Alliance
                                                    </h2>
                                                    <div class="rounded-lg p-4 space-y-2">
                                                        <div class="team-container" id="red1">
                                                            <span class="font-bold">{"Team 1: "}</span>
                                                            <Suspense fallback=move || {
                                                                view! { Loading... }
                                                            }>{team_number_view!(current_match, red, 0)}</Suspense>
                                                            <Suspense fallback=move || {
                                                                view! { <p>"Loading stats..."</p> }
                                                            }>
                                                                <div class="text-sm opacity-75 team-stats">{}</div>
                                                            </Suspense>
                                                        </div>
                                                        <div class="team-container" id="red2">
                                                            <span class="font-bold">{"Team 2: "}</span>
                                                            <Suspense fallback=move || {
                                                                view! { Loading... }
                                                            }>{team_number_view!(current_match, red, 1)}</Suspense>
                                                            <Suspense fallback=move || {
                                                                view! { <p>"Loading stats..."</p> }
                                                            }>
                                                                <div class="text-sm opacity-75 team-stats">
                                                                    {team_data_view!(current_match, red, 1)}
                                                                </div>
                                                            </Suspense>
                                                        </div>
                                                        <div class="team-container" id="red3">
                                                            <span class="font-bold">{"Team 3: "}</span>
                                                            <Suspense fallback=move || {
                                                                view! { Loading... }
                                                            }>{team_number_view!(current_match, red, 2)}</Suspense>
                                                            <Suspense fallback=move || {
                                                                view! { <p>"Loading stats..."</p> }
                                                            }>
                                                                <div class="text-sm opacity-75 team-stats">
                                                                    {team_data_view!(current_match, red, 2)}
                                                                </div>
                                                            </Suspense>
                                                        </div>
                                                    </div>
                                                </div>

                                                <div class="text-center flex flex-col justify-center items-center">
                                                    <div class="flex justify-center mb-6">
                                                        <button
                                                            on:click=set_next_team_match
                                                            class="btn btn-primary"
                                                        >
                                                            Next Team Match
                                                        </button>
                                                    </div>
                                                    <div class="text-2xl font-bold mb-2" id="matchNumber">
                                                        Match
                                                        <input
                                                            min="1"
                                                            max="999"
                                                            style="width: 60px !important; font-size: var(--text-xl) !important;"
                                                            class=".transparent-num !input !input-primary"
                                                            type="number"
                                                            prop:value=move || current_match_num.get().to_string()
                                                            on:keydown=prevent_invalid_input
                                                            on:onchange=update_match_number
                                                        />
                                                    </div>
                                                    <div class="badge badge-neutral" id="matchTime">

                                                        Time:
                                                        <Suspense fallback=move || {
                                                            view! { <span>Loading...</span> }
                                                        }>
                                                            {move || {
                                                                match current_match.get() {
                                                                    Some(Some(match_data)) => {
                                                                        format_timestamp(match_data.predicted_time)
                                                                    }
                                                                    Some(None) => "Error".to_owned(),
                                                                    None => "TBD".to_owned(),
                                                                }
                                                            }}
                                                        </Suspense>

                                                    </div>
                                                </div>

                                                <div class="flex-1">
                                                    <h2 class="text-xl font-bold text-center text-primary mb-4 text-blue-600">
                                                        Blue Alliance
                                                    </h2>
                                                    <div class="rounded-lg p-4 space-y-2">
                                                        <div class="team-container" id="blue1">
                                                            <span class="font-bold">{"Team 1: "}</span>
                                                            <Suspense fallback=move || {
                                                                view! { Loading... }
                                                            }>{team_number_view!(current_match, blue, 0)}</Suspense>
                                                            <Suspense fallback=move || {
                                                                view! { <p>"Loading stats..."</p> }
                                                            }>
                                                                <div class="text-sm opacity-75 team-stats">
                                                                    {team_data_view!(current_match, blue, 0)}
                                                                </div>
                                                            </Suspense>
                                                        </div>
                                                        <div class="team-container" id="blue2">
                                                            <span class="font-bold">{"Team 2: "}</span>
                                                            <Suspense fallback=move || {
                                                                view! { Loading... }
                                                            }>{team_number_view!(current_match, blue, 1)}</Suspense>
                                                            <Suspense fallback=move || {
                                                                view! { <p>"Loading stats..."</p> }
                                                            }>
                                                                <div class="text-sm opacity-75 team-stats">
                                                                    {team_data_view!(current_match, blue, 1)}
                                                                </div>
                                                            </Suspense>
                                                        </div>
                                                        <div class="team-container" id="blue3">
                                                            <span class="font-bold">{"Team 3: "}</span>
                                                            <Suspense fallback=move || {
                                                                view! { Loading... }
                                                            }>{team_number_view!(current_match, blue, 2)}</Suspense>
                                                            <Suspense fallback=move || {
                                                                view! { <p>"Loading stats..."</p> }
                                                            }>
                                                                <div class="text-sm opacity-75 team-stats">
                                                                    {team_data_view!(current_match, blue, 2)}
                                                                </div>
                                                            </Suspense>
                                                        </div>
                                                    </div>
                                                </div>
                                            </div>
                                            <div class="flex justify-center mt-6">
                                                <button
                                                    class="btn btn-outline"
                                                    id="refreshNextMatch"
                                                    on:click=move |_| {
                                                        current_match.refetch();
                                                        data.refetch();
                                                    }
                                                >
                                                    <svg
                                                        xmlns="http://www.w3.org/2000/svg"
                                                        class="h-5 w-5 mr-2"
                                                        fill="none"
                                                        viewBox="0 0 24 24"
                                                        stroke="currentColor"
                                                    >
                                                        <path
                                                            stroke-linecap="round"
                                                            stroke-linejoin="round"
                                                            stroke-width="2"
                                                            d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
                                                        />
                                                    </svg>
                                                    Refresh
                                                </button>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            }
                                .into_any()
                        }
                    }}
                </Suspense>
            </div>
        </PageWrapper>
    }
}
