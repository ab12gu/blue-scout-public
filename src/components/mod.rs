//! Defines the main components of the application.

#![allow(clippy::must_use_candidate, clippy::exhaustive_structs)]
mod dock;
mod fallback_page;
mod home_page;
mod settings_page;
mod viewdata_page;
pub use dock::Dock;
pub use fallback_page::FallbackPage;
pub use home_page::HomePage;
use leptos::prelude::*;
use leptos_meta::Script;
pub use settings_page::SettingsPage;
pub use viewdata_page::ViewDataPage;

/// Provides a consistent page layout with a navigation dock.
///
/// This component wraps the main content of a page and includes a navigation
/// dock at the bottom. It also injects a script for handling navigation.
///
/// # Props
///
/// * `children`: The content to be displayed within the page.
#[component]
pub fn PageWrapper(children: Children) -> impl IntoView {
    view! {
        <Script src="/navigation.js"></Script>
        <div class="page-container">
            <div class="page p-8">{children()}</div>
        </div>
        <Dock />
    }
}

/// Provides a styled container for displaying content.
///
/// This component wraps its children in a card-like container with a
/// background color and shadow.
///
/// # Props
///
/// * `children`: The content to be displayed within the card.
#[component]
#[allow(dead_code)]
pub fn Card(children: Children) -> impl IntoView {
    view! {
        <div class="card bg-base-200 shadow-xl">
            <div class="card-body p-8">{children()}</div>
        </div>
    }
}
