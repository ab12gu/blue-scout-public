#![allow(dead_code, unused_variables)]

use leptos::{ev, prelude::*};
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use leptos::wasm_bindgen::{self, JsCast};

use web_sys::{window, HtmlInputElement};

use crate::components::Dock;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en" data-theme="dark">
            <head>
                <script>{include_str!("../embed/theme.js")}</script>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <script src="https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4"></script>
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/blue_scout.css" />
        <Stylesheet id="leptos" href="/tailwind.css" />

        <Title text="4682's Scouting Site" />

        <Router>
            <main>
                <Routes fallback=FallbackPage>
                    <Route path=StaticSegment("/") view=HomePage />
                    <Route path=StaticSegment("/view-data") view=ViewDataPage />
                    <Route path=StaticSegment("/settings") view=SettingsPage />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn FallbackPage() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-base-200 flex items-center justify-center">
            <div class="text-center max-w-md p-6">
                <h1 class="text-9xl font-bold text-primary mb-4">404</h1>
                <div class="divider"></div>
                <h2 class="text-2xl font-semibold mb-4">Page Not Found</h2>
                <p class="mb-8 text-base-content/70">
                    {"Oops! The page you're looking for doesn't exist or has been moved."}
                </p>
                <a onclick="window.location.href = '/'" class="btn btn-primary">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke-width="1.5"
                        stroke="currentColor"
                        class="w-6 h-6 mr-2"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            d="M2.25 12l8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125 1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504 1.125-1.125V9.75M8.25 21h8.25"
                        />
                    </svg>
                    Back to Home
                </a>
            </div>
        </div>
    }
}

#[component]
pub fn ViewDataPage() -> impl IntoView {
    view! {
        <PageWrapper>
            <div class="container mx-auto max-w-3xl">
                <h1 class="text-3xl font-bold text-center mb-8">View Scouting Data</h1>
            </div>
        </PageWrapper>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <PageWrapper>
            <div class="container mx-auto max-w-3xl">
                <h1 class="text-3xl font-bold text-center mb-8">Scouting Form</h1>
            </div>
        </PageWrapper>
    }
}

#[component]
fn SettingsPage() -> impl IntoView {
    // Initialize state with localStorage values
    let initial_theme = window()
        .and_then(|win| win.local_storage().ok().flatten())
        .and_then(|storage| storage.get_item("theme").ok().flatten())
        .unwrap_or_else(|| "dark".into());

    let is_dark_theme = RwSignal::new(initial_theme == "dark");

    // Get saved team number if available
    let initial_team_number = window()
        .and_then(|win| win.local_storage().ok().flatten())
        .and_then(|storage| storage.get_item("teamNumber").ok().flatten())
        .unwrap_or_default();

    // Set up effects that run when the component mounts
    Effect::new(move |_| {
        if let Some(window) = window() {
            // Set the initial theme
            if let Some(document) = window.document() {
                if let Some(html) = document.document_element() {
                    let _ = html.set_attribute(
                        "data-theme",
                        if is_dark_theme() { "dark" } else { "light" },
                    );
                }
            }
        }
    });

    // Handler for theme toggle
    let toggle_theme = move |ev: ev::Event| {
        let input_el = event_target::<HtmlInputElement>(&ev);
        let checked = input_el.map(|input| input.checked()).unwrap_or(false);

        is_dark_theme.set(checked);

        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                // Save to localStorage
                let theme = if checked { "dark" } else { "light" };
                let _ = storage.set_item("theme", theme);

                // Update document theme
                if let Some(document) = window.document() {
                    if let Some(html) = document.document_element() {
                        let _ = html.set_attribute("data-theme", theme);
                    }
                }
            }
        }
    };

    // Handler for team number input
    let save_team_number = move |ev: ev::Event| {
        let input_el = event_target::<HtmlInputElement>(&ev);
        let value = input_el.map(|input| input.value()).unwrap_or_default();

        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("teamNumber", &value);
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
                                            prop:checked=move || is_dark_theme()
                                            id="themeToggle"
                                            on:change=toggle_theme
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
                                            prop:value=initial_team_number
                                            on:change=save_team_number
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

// Helper function to get element from event
fn event_target<T: JsCast>(ev: &ev::Event) -> Option<T> {
    ev.target().and_then(|target| target.dyn_into::<T>().ok())
}

#[component]
fn PageWrapper(children: Children) -> impl IntoView {
    view! {
        <Script src="/navigation.js"></Script>
        <div class="page-container">
            <div class="page p-8">{children()}</div>
        </div>
        <Dock />
    }
}

#[component]
fn Card(children: Children) -> impl IntoView {
    view! {
        <div class="card bg-base-200 shadow-xl">
            <div class="card-body p-8">{children()}</div>
        </div>
    }
}
