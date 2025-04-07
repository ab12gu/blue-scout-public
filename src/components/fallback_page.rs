#[cfg(feature = "ssr")]
#[allow(unused_imports)]
use crate::api_config;

use leptos::prelude::*;

/// A component that renders a fallback page for 404 errors.
/// The page includes a message indicating that the page was not found
/// and a button to navigate back to the home page.
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
