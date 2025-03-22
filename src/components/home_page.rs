use leptos::prelude::*;

use crate::components::PageWrapper;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <PageWrapper>
            <div class="container mx-auto max-w-3xl">
                <h1 class="text-3xl font-bold text-center mb-8">Scouting Form</h1>
            </div>
        </PageWrapper>
    }
}
