use leptos::prelude::*;

use web_sys::{window, Event};

use crate::components::PageWrapper;

#[component]
pub fn SettingsPage() -> impl IntoView {
    // Create signals for theme and team number
    let (theme, set_theme) = signal("dark".to_string());
    let (team_number, set_team_number) = signal("".to_string());

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
            }
        }
    });

    // Handle theme toggle
    let on_theme_change = move |ev: Event| {
        let is_checked = event_target_checked(&ev);
        let new_theme = if is_checked { "dark" } else { "light" };
        set_theme(new_theme.to_string());

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
                                />
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
