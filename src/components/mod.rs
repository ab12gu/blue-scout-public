mod dock;
mod fallback_page;
mod home_page;
mod settings_page;
mod viewdata_page;
pub use dock::Dock;
pub use fallback_page::FallbackPage;
pub use home_page::HomePage;
use leptos::prelude::*;
use leptos_meta::*;
pub use settings_page::SettingsPage;
pub use viewdata_page::ViewDataPage;
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

#[component]
fn Card(children: Children) -> impl IntoView {
    view! {
        <div class="card bg-base-200 shadow-xl">
            <div class="card-body p-8">{children()}</div>
        </div>
    }
}
