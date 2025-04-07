//! Provide the `SettingsPage` component to change settings

#[cfg(feature = "ssr")]
#[allow(unused_imports)]
use crate::api_config;

use crate::components::PageWrapper;
use crate::BlueScoutError;
use leptos::{ev, logging, prelude::*, task::spawn_local};
use tbaapi::models::Event as TBAEvent;
use web_sys::{window, Event};

/// Fetches FRC events from the server.
///
/// # Returns
///
/// A `Result` containing a vector of `TBAEvent` on success, or a
/// `BlueScoutError` on failure.
#[server]
pub async fn get_frc_events() -> Result<Vec<TBAEvent>, BlueScoutError> {
    #[cfg(feature = "ssr")]
    return crate::api::get_frc_events()
        .await
        .map_err(BlueScoutError::api_error);
    #[cfg(not(feature = "ssr"))]
    {
        panic!("This should be called on the server!");
    };
}

/// The `SettingsPage` component allows users to configure settings such as
/// theme, team number, and event selection.
#[component]
pub fn SettingsPage() -> impl IntoView {
    // Create signals for theme and team number
    let (theme, set_theme) = signal("dark".to_owned());
    let (team_number, set_team_number) = signal(String::new());
    let (event_name, set_event_name) = signal(String::new());
    let (events_list, set_events_list) = signal(Vec::<TBAEvent>::new());

    Effect::new(move |_| {
        spawn_local(async move {
            let events = get_frc_events().await;
            match events {
                Ok(events) => set_events_list(events),
                Err(err) => logging::error!("Failed to load events list: {}", err),
            }
        });
    });

    let (dropdown_visible, set_dropdown_visible) = signal(false);

    let filtered_options = Memo::new(move |_| {
        let input = event_name.get().to_lowercase();

        events_list
            .get()
            .iter()
            .filter(|event| {
                event
                    .short_name
                    .as_ref()
                    .is_some_and(|x| x.to_lowercase().contains(&input))
            })
            .cloned()
            .collect::<Vec<_>>()
    });

    // Handle input changes
    let handle_input_change = move |ev: Event| {
        let value = event_target_value(&ev);
        set_event_name(value);
        set_dropdown_visible(true);
    };

    // Handle option selection
    let select_option = move |option: String| {
        set_event_name(option.clone());

        // Save to localStorage
        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Some(current_event) = events_list
                    .get_untracked()
                    .iter()
                    .find(|x| x.short_name.as_ref().is_some_and(|x| x == option.as_str()))
                {
                    let _ = storage.set_item("currentEvent", &current_event.key);
                }
                let _ = storage.set_item("currentEventName", &option);
            }
        }
        set_dropdown_visible(false);
    };

    // Handle focus to show dropdown
    let handle_focus = move |_| {
        set_dropdown_visible(true);
    };

    // Initialize values from localStorage on component mount
    Effect::new(move |_| {
        if let Some(window) = window() {
            if let Ok(storage) = window.local_storage()
                && let Some(storage) = storage
            {
                // Get saved theme
                if let Ok(Some(saved_theme)) = storage.get_item("theme") {
                    set_theme(saved_theme);
                }

                // Get saved team number
                if let Ok(Some(saved_team_number)) = storage.get_item("teamNumber") {
                    set_team_number(saved_team_number);
                }

                // Get saved event
                if let Ok(Some(saved_event)) = storage.get_item("currentEventName") {
                    set_event_name(saved_event);
                }
            }
        }
    });

    // Handle theme toggle
    let on_theme_change = move |ev: Event| {
        let is_checked = event_target_checked(&ev);
        let new_theme = if is_checked { "dark" } else { "light" };
        set_theme(new_theme.to_owned());

        // Update localStorage and document
        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("theme", new_theme);
            }

            if let Some(document) = window.document() {
                if let Some(html) = document.document_element() {
                    let _ = html.set_attribute("data-theme", new_theme);
                }
            }
        }
    };

    // Handle team number change
    let on_team_number_change = move |ev: Event| {
        let new_value = event_target_value(&ev);
        set_team_number(new_value.clone());

        // Save to localStorage
        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("teamNumber", &new_value);
            }
        }
    };

    let on_event_change = move |ev: Event| {
        let new_value = event_target_value(&ev);
        set_event_name(new_value.clone());

        // Save to localStorage
        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Some(current_event) = events_list.get_untracked().iter().find(|x| {
                    x.short_name
                        .as_ref()
                        .is_some_and(|x| x == new_value.as_str())
                }) {
                    let _ = storage.set_item("currentEvent", &current_event.key);
                }
                let _ = storage.set_item("currentEventName", &new_value);
            }
        }
    };

    let prevent_invalid_input = |ev: ev::KeyboardEvent| {
        if ev.key() == "-" || ev.key() == "." {
            ev.prevent_default();
        }
    };

    view! {
        <PageWrapper>
            <div class="container mx-auto max-w-3xl">
                <h1 class="text-3xl font-bold text-center mb-8">Settings</h1>
                <div class="card bg-base-200 shadow-xl">
                    <div class="card-body p-8">
                        <div class="form-control">
                            <label class="label cursor-pointer">
                                <span class="label-text text-lg">Dark Theme</span>
                                <input
                                    type="checkbox"
                                    class="toggle toggle-primary"
                                    id="themeToggle"
                                    checked=move || theme() == "dark"
                                    on:change=on_theme_change
                                />
                            </label>
                        </div>
                        <div class="form-control">
                            <div class="w-96 relative">
                                <label class="label cursor-pointer">
                                    <span class="label-text text-lg">Enter Team Number:</span>
                                </label>
                                <input
                                    type="number"
                                    class="input input-primary"
                                    id="teamNumberInput"
                                    prop:value=team_number
                                    on:change=on_team_number_change
                                    on:keydown=prevent_invalid_input
                                />
                            </div>
                        </div>
                        <div class="form-control">
                            <div class="w-96 relative">
                                <label class="label cursor-pointer">
                                    <span class="label-text text-lg">Enter Event:</span>
                                </label>
                                <input
                                    class="input input-primary"
                                    id="eventInput"
                                    prop:value=move || event_name.get()
                                    on:input=handle_input_change
                                    on:focus=handle_focus
                                    on:change=on_event_change
                                />
                                <ul
                                    id="eventInputAutocomplete"
                                    class="dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-52 max-h-80 flex-nowrap overflow-auto absolute"
                                    class:hidden=move || {
                                        !dropdown_visible.get()
                                            || filtered_options.with(Vec::is_empty)
                                    }
                                >
                                    <Suspense>
                                        {move || {
                                            if events_list.get().is_empty() {
                                                // Show loading indicator while resource is loading
                                                view! {
                                                    <li>
                                                        <span class="opacity-70">Loading...</span>
                                                    </li>
                                                }
                                                    .into_any()
                                            } else {
                                                filtered_options
                                                    .with(|options| {
                                                        if options.is_empty() {
                                                            view! {
                                                                <li>
                                                                    <span class="opacity-70">No matches found</span>
                                                                </li>
                                                            }
                                                                .into_any()
                                                        } else {
                                                            options
                                                                .iter()
                                                                .map(|event| {
                                                                    let event_name = event.short_name.clone();
                                                                    view! {
                                                                        <li>
                                                                            <a on:click=move |_| select_option(
                                                                                event_name.clone().unwrap_or_default(),
                                                                            )>{event.short_name.clone()}</a>
                                                                        </li>
                                                                    }
                                                                })
                                                                .collect_view()
                                                                .into_any()
                                                        }
                                                    })
                                            }
                                        }}
                                    </Suspense>
                                </ul>
                            </div>
                        </div>
                        <div class="form-control mt-6">
                            <label class="label">
                                <span class="label-text text-lg">About</span>
                            </label>
                            <div class="ml-4 mt-2">
                                <p>4682 Scouting App v1.0</p>
                                <p class="text-sm text-gray-400 mt-2">
                                    Created by Team 4682 {"\"CyBears\""}
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </PageWrapper>
    }
}
