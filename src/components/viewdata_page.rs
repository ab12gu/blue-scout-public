use std::ops::Deref;

use chrono::{DateTime, Local};
use leptos::prelude::*;

use crate::{components::PageWrapper, DataPoint, MatchInfo, TeamData};

const CURRENT_EVENT: &str = "2025wabon";
const CURRENT_MATCH: usize = 1;

#[server(endpoint = "fetch_match_data")]
pub async fn fetch_match_data(
    match_number: u32,
    event: String,
) -> Result<MatchInfo, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::db::get_match_info;
        get_match_info(match_number, &event)
            .await
            .map_err(ServerFnError::new)
    }
    #[cfg(not(feature = "ssr"))]
    {
        tracing::error!("Server function called without ssr feature enabled");
        unreachable!("This should only be called on the server");
    }
}

pub const FULL_COLUMN_NAMES: &[&str] = &[
    "Name",
    "Match",
    "Team",
    "Auto Coral",
    "Auto Algae",
    "Auto Leave",
    "Algae Clear",
    "L1",
    "L2",
    "L3",
    "L4",
    "Dropped",
    "Barge",
    "Floor Hole",
    "Climb",
    "Defense",
];

pub const REDUCED_COLUMN_NAMES: &[&str] = &[
    "Match",
    "Team",
    "Auto Coral",
    "Auto Leave",
    "Algae Clear",
    "Teleop Coral",
    "Teleop Algae",
    "Climb",
    "Defense",
];

#[server(endpoint = "fetch_scouting_data")]
pub async fn fetch_scouting_data(
    team_number_filter: Option<u32>,
) -> Result<Vec<DataPoint>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::db::get_data as get_db_data;
        get_db_data()
            .await
            .map(|x| {
                x.into_iter()
                    .filter(|data| {
                        team_number_filter.is_none_or(|filter| data.team_number == filter)
                    })
                    .collect()
            })
            .map_err(ServerFnError::new)
    }
    #[cfg(not(feature = "ssr"))]
    {
        tracing::error!("Server function called without ssr feature enabled");
        unreachable!("This should only be called on the server");
    }
}

#[inline]
fn display_bool(b: bool) -> String {
    if b {
        "Yes".to_string()
    } else {
        "No".to_string()
    }
}

fn display_team_data(team_data: &TeamData) -> AnyView {
    view! {
        <div class="team-data">
            <p>Avg Coral: {format!("{:.1}", team_data.avg_coral)}</p>
            <p>Avg Auto Coral: {format!("{:.1}", team_data.avg_auto_coral)}</p>
            <p>Avg Barge Algae: {format!("{:.1}", team_data.avg_barge_algae)}</p>
            <p>
                Scoring Locations:
                {
                    let locations = [
                        ("L1", team_data.score_l1),
                        ("L2", team_data.score_l2),
                        ("L3", team_data.score_l3),
                        ("L4", team_data.score_l4),
                    ]
                        .iter()
                        .filter_map(|x| if x.1 > 0 { Some(x.0) } else { None })
                        .collect::<Vec<&str>>()
                        .join(", ");
                    if locations.is_empty() { "None".to_string() } else { locations }
                }
            </p>
            <p>Sum of Deep Climbs: {team_data.sum_of_deep_climbs.to_string()}</p>
            <p>Sum of Climb Not Attempted: {team_data.sum_of_climb_not_attempted.to_string()}</p>
        </div>
    }
    .into_any()
}

macro_rules! team_data_view {
    ($current_match:expr, $team:ident, $index:expr) => {
        move || match $current_match.get() {
            Some(Some(match_data)) => {
                let team_data = &match_data.$team[$index];
                match team_data.team_data {
                    Some(data) => display_team_data(&data),
                    None => view! {
                        <span class="team-number">No stats available</span>
                    }
                    .into_any(),
                }
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

fn format_timestamp(timestamp: i64) -> String {
    let datetime = DateTime::from_timestamp(timestamp, 0)
        .unwrap()
        .with_timezone(&Local);
    datetime.format("%a %-I:%M %p").to_string()
}

#[component]
pub fn ViewDataPage() -> impl IntoView {
    let (use_full_names, set_use_full_names) = signal(false);
    let (team_number_filter, set_team_number_filter) = signal(None::<u32>);

    let column_names = move || {
        (if use_full_names() {
            FULL_COLUMN_NAMES
        } else {
            REDUCED_COLUMN_NAMES
        })
        .iter()
        .map(|name| view! { <th>{name.to_string()}</th> })
        .collect_view()
    };

    let data = Resource::new(
        move || (use_full_names.get(), team_number_filter.get()),
        move |(_, team_number_filter_value)| fetch_scouting_data(team_number_filter_value),
    );
    let current_match = Resource::new(
        || (),
        |_| async move {
            fetch_match_data(CURRENT_MATCH as u32, CURRENT_EVENT.to_string())
                .await
                .ok()
        },
    );

    view! {
        <Suspense>
            <script src="/tablefilter/tablefilter.js"></script>
        </Suspense>
        <script src="/viewdata_page.js"></script>
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
                                on:input=move |ev| set_use_full_names(event_target_checked(&ev))
                            />
                            <br />
                            <br />
                            <div class="form-control w-full mb-8">
                                <label
                                    class="label-text text-lg font-medium"
                                    for="teamNumberFilter"
                                >
                                    Team Number Filter
                                </label>
                                <input
                                    name="teamNumberFilter"
                                    class="input input-bordered w-full"
                                    style="outline: none;"
                                    type="number"
                                    placeholder="Enter team number"
                                    required
                                    onkeydown="preventMinusSign(event)"
                                    on:keydown=move |ev: leptos::ev::KeyboardEvent| {
                                        if ev.key() == "Enter" {
                                            let value = event_target_value(&ev);
                                            set_team_number_filter.set(value.parse().ok());
                                        }
                                    }
                                    on:blur=move |ev| {
                                        let value = event_target_value(&ev);
                                        set_team_number_filter.set(value.parse().ok());
                                    }
                                />
                            </div>
                            <table class="table" id="scouting_data_table">
                                <thead>
                                    <tr>{move || column_names()}</tr>
                                </thead>
                                <tbody>
                                    <Suspense fallback=move || {
                                        view! { <span>Loading...</span> }
                                    }>
                                        {move || match data.read().deref() {
                                            Some(Ok(items)) => {
                                                items
                                                    .iter()
                                                    .map(|item| {
                                                        view! {
                                                            <tr class="hover:bg-base-300">
                                                                {if use_full_names.get() {
                                                                    view! {
                                                                        <td>{item.name.clone()}</td>
                                                                        <td>{item.match_number}</td>
                                                                        <td>{item.team_number}</td>
                                                                        <td>{item.auto_coral}</td>
                                                                        <td>{item.auto_algae}</td>
                                                                        <td>{display_bool(item.auto_leave)}</td>
                                                                        <td>{display_bool(item.algae_clear)}</td>
                                                                        <td>{item.l1_coral}</td>
                                                                        <td>{item.l2_coral}</td>
                                                                        <td>{item.l3_coral}</td>
                                                                        <td>{item.l4_coral}</td>
                                                                        <td>{item.dropped_coral}</td>
                                                                        <td>{item.algae_barge}</td>
                                                                        <td>{item.algae_floor_hole}</td>
                                                                        <td>{item.climb.to_string()}</td>
                                                                        <td>{display_bool(item.defense_bot)}</td>
                                                                    }
                                                                        .into_any()
                                                                } else {
                                                                    view! {
                                                                        <td>{item.match_number}</td>
                                                                        <td>{item.team_number}</td>
                                                                        <td>{item.auto_coral}</td>
                                                                        <td>{display_bool(item.auto_leave)}</td>
                                                                        <td>{display_bool(item.algae_clear)}</td>
                                                                        <td>
                                                                            {item.l1_coral + item.l2_coral + item.l3_coral
                                                                                + item.l4_coral}
                                                                        </td>
                                                                        <td>{item.algae_barge + item.algae_floor_hole}</td>
                                                                        <td>{item.climb.to_string()}</td>
                                                                        <td>{display_bool(item.defense_bot)}</td>
                                                                    }
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

            <div class="container mx-auto max-w-3xl mt-[69px]">
                <h1 class="text-3xl font-bold text-center mb-8">View Next Match</h1>
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
                                                <div class="text-sm opacity-75 team-stats">
                                                    {team_data_view!(current_match, red, 0)}
                                                </div>
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
                                    <div class="text-2xl font-bold mb-2" id="matchNumber">
                                        Match
                                        {CURRENT_MATCH}
                                    </div>
                                    <div class="badge badge-neutral" id="matchTime">

                                        Time:
                                        <Suspense fallback=move || {
                                            view! { <span>Loading...</span> }
                                        }>
                                            {move || {
                                                match current_match.get() {
                                                    Some(Some(match_data)) => {
                                                        format_timestamp(match_data.predicted_time as i64)
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
            </div>
        </PageWrapper>
    }
}
