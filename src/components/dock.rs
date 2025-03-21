use leptos::prelude::*;
use leptos_router::hooks::use_location;

#[component]
pub fn Dock() -> impl IntoView {
    view! {
        <div class="dock">
            <button class="outline-none" class:dock-active=move || use_location().pathname.read().as_str() == "/" id="addDataButton">
                <svg
                    enable-background="new 0 0 50 50"
                    height="50px"
                    id="Layer_1"
                    version="1.1"
                    viewBox="0 0 50 50"
                    width="50px"
                    xml:space="preserve"
                    xmlns="http://www.w3.org/2000/svg"
                    xmlns:xlink="http://www.w3.org/1999/xlink"
                >
                    <rect fill="none" height="50" width="50" />
                    <line
                        fill="none"
                        stroke="currentColor"
                        stroke-miterlimit="10"
                        stroke-width="8"
                        x1="0"
                        x2="50"
                        y1="25"
                        y2="25"
                    />
                    <line
                        fill="none"
                        stroke="currentColor"
                        stroke-miterlimit="10"
                        stroke-width="8"
                        x1="25"
                        x2="25"
                        y1="0"
                        y2="50"
                    />
                </svg>
                <span class="dock-label">Add Data</span>
            </button>

            <button class="outline-none" class:dock-active=move || use_location().pathname.read().as_str() == "/view-data" id="viewDataButton">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    xmlns:xlink="http://www.w3.org/1999/xlink"
                    version="1.1"
                    width="512"
                    height="512"
                    viewBox="0 0 256 256"
                    xml:space="preserve"
                >
                    <defs></defs>
                    <g
                        style="
                        stroke: none;
                        stroke-width: 0;
                        stroke-dasharray: none;
                        stroke-linecap: butt;
                        stroke-linejoin: miter;
                        stroke-miterlimit: 10;
                        fill: none;
                        fill-rule: nonzero;
                        opacity: 1;
                        "
                        transform="translate(1.4065934065934016 1.4065934065934016) scale(2.81 2.81)"
                    >
                        <path
                            d="M 87.994 0 H 69.342 c -1.787 0 -2.682 2.16 -1.418 3.424 l 5.795 5.795 l -33.82 33.82 L 28.056 31.196 l -3.174 -3.174 c -1.074 -1.074 -2.815 -1.074 -3.889 0 L 0.805 48.209 c -1.074 1.074 -1.074 2.815 0 3.889 l 3.174 3.174 c 1.074 1.074 2.815 1.074 3.889 0 l 15.069 -15.069 l 14.994 14.994 c 1.074 1.074 2.815 1.074 3.889 0 l 1.614 -1.614 c 0.083 -0.066 0.17 -0.125 0.247 -0.202 l 37.1 -37.1 l 5.795 5.795 C 87.84 23.34 90 22.445 90 20.658 V 2.006 C 90 0.898 89.102 0 87.994 0 z"
                            style="
                            stroke: none;
                            stroke-width: 1;
                            stroke-dasharray: none;
                            stroke-linecap: butt;
                            stroke-linejoin: miter;
                            stroke-miterlimit: 10;
                            fill: currentColor;
                            fill-rule: nonzero;
                            opacity: 1;
                            "
                            transform=" matrix(1 0 0 1 0 0) "
                            stroke-linecap="round"
                        />
                        <path
                            d="M 65.626 37.8 v 49.45 c 0 1.519 1.231 2.75 2.75 2.75 h 8.782 c 1.519 0 2.75 -1.231 2.75 -2.75 V 23.518 L 65.626 37.8 z"
                            style="
                            stroke: none;
                            stroke-width: 1;
                            stroke-dasharray: none;
                            stroke-linecap: butt;
                            stroke-linejoin: miter;
                            stroke-miterlimit: 10;
                            fill: currentColor;
                            fill-rule: nonzero;
                            opacity: 1;
                            "
                            transform=" matrix(1 0 0 1 0 0) "
                            stroke-linecap="round"
                        />
                        <path
                            d="M 47.115 56.312 V 87.25 c 0 1.519 1.231 2.75 2.75 2.75 h 8.782 c 1.519 0 2.75 -1.231 2.75 -2.75 V 42.03 L 47.115 56.312 z"
                            style="
                            stroke: none;
                            stroke-width: 1;
                            stroke-dasharray: none;
                            stroke-linecap: butt;
                            stroke-linejoin: miter;
                            stroke-miterlimit: 10;
                            fill: currentColor;
                            fill-rule: nonzero;
                            opacity: 1;
                            "
                            transform=" matrix(1 0 0 1 0 0) "
                            stroke-linecap="round"
                        />
                        <path
                            d="M 39.876 60.503 c -1.937 0 -3.757 -0.754 -5.127 -2.124 l -6.146 -6.145 V 87.25 c 0 1.519 1.231 2.75 2.75 2.75 h 8.782 c 1.519 0 2.75 -1.231 2.75 -2.75 V 59.844 C 41.952 60.271 40.933 60.503 39.876 60.503 z"
                            style="
                            stroke: none;
                            stroke-width: 1;
                            stroke-dasharray: none;
                            stroke-linecap: butt;
                            stroke-linejoin: miter;
                            stroke-miterlimit: 10;
                            fill: currentColor;
                            fill-rule: nonzero;
                            opacity: 1;
                            "
                            transform=" matrix(1 0 0 1 0 0) "
                            stroke-linecap="round"
                        />
                        <path
                            d="M 22.937 46.567 L 11.051 58.453 c -0.298 0.298 -0.621 0.562 -0.959 0.8 V 87.25 c 0 1.519 1.231 2.75 2.75 2.75 h 8.782 c 1.519 0 2.75 -1.231 2.75 -2.75 V 48.004 L 22.937 46.567 z"
                            style="
                            stroke: none;
                            stroke-width: 1;
                            stroke-dasharray: none;
                            stroke-linecap: butt;
                            stroke-linejoin: miter;
                            stroke-miterlimit: 10;
                            fill: currentColor;
                            fill-rule: nonzero;
                            opacity: 1;
                            "
                            transform=" matrix(1 0 0 1 0 0) "
                            stroke-linecap="round"
                        />
                    </g>
                </svg>
                <span class="dock-label">View Data</span>
            </button>

            <button class="outline-none" class:dock-active=move || use_location().pathname.read().as_str() == "/settings" id="settingsButton">
                <svg class="size-[1.2em]" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                    <g fill="currentColor" stroke-linejoin="miter" stroke-linecap="butt">
                        <circle
                            cx="12"
                            cy="12"
                            r="3"
                            fill="none"
                            stroke="currentColor"
                            stroke-linecap="square"
                            stroke-miterlimit="10"
                            stroke-width="2"
                        ></circle>
                        <path
                            d="m22,13.25v-2.5l-2.318-.966c-.167-.581-.395-1.135-.682-1.654l.954-2.318-1.768-1.768-2.318.954c-.518-.287-1.073-.515-1.654-.682l-.966-2.318h-2.5l-.966,2.318c-.581.167-1.135.395-1.654.682l-2.318-.954-1.768,1.768.954,2.318c-.287.518-.515,1.073-.682,1.654l-2.318.966v2.5l2.318.966c.167.581.395,1.135.682,1.654l-.954,2.318,1.768,1.768,2.318-.954c.518.287,1.073.515,1.654.682l.966,2.318h2.5l.966-2.318c.581-.167,1.135-.395,1.654-.682l2.318.954,1.768-1.768-.954-2.318c.287-.518.515-1.073.682-1.654l2.318-.966Z"
                            fill="none"
                            stroke="currentColor"
                            stroke-linecap="square"
                            stroke-miterlimit="10"
                            stroke-width="2"
                        ></path>
                    </g>
                </svg>
                <span class="dock-label">Settings</span>
            </button>
        </div>
    }
}
