use leptos::{prelude::*, task::spawn_local};

use crate::components::PageWrapper;

const CURRENT_EVENT: &str = "2025wabon";
const CURRENT_MATCH: usize = 1;

#[server]
pub async fn fetch_data(match_number: u32) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::db::get_match_info;
        println!("{:?}", get_match_info(match_number).await);
        Ok(())
    }
    #[cfg(not(feature = "ssr"))]
    {
        tracing::error!("Server function called without ssr feature enabled");
        unreachable!("This should only be called on the server");
    }
}

#[component]
pub fn ViewDataPage() -> impl IntoView {
    view! {
        <PageWrapper>
            <div class="container mx-auto max-w-3xl">
                <h1 class="text-3xl font-bold text-center mb-8">View Scouting Data</h1>

                <div class="card bg-base-200 shadow-xl">
                    <div class="card-body p-8">
                        <div class="overflow-x-auto">
                            <table class="table">
                                <thead>
                                    <tr>
                                        <th>Name</th>
                                        <th>Match #</th>
                                        <th>Team #</th>
                                        <th>Auto Algae #</th>
                                        <th>Auto Coral #</th>
                                        <th>Auto Leave</th>
                                        <th>Algae Clear</th>
                                        <th>L1</th>
                                        <th>L2</th>
                                        <th>L3</th>
                                        <th>L4</th>
                                        <th>Dropped</th>
                                        <th>Floor Hole</th>
                                        <th>Barge</th>
                                        <th>Climb</th>
                                        <th>Defense</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    <tr class="hover:bg-base-300"></tr>
                                </tbody>
                            </table>
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
                                            <span class="font-bold">Team 1:</span>
                                            <span class="team-number">Loading...</span>
                                            <div class="text-sm opacity-75 team-stats">
                                                Loading stats...
                                            </div>
                                        </div>
                                        <div class="team-container" id="red2">
                                            <span class="font-bold">Team 2:</span>
                                            <span class="team-number">Loading...</span>
                                            <div class="text-sm opacity-75 team-stats">
                                                Loading stats...
                                            </div>
                                        </div>
                                        <div class="team-container" id="red3">
                                            <span class="font-bold">Team 3:</span>
                                            <span class="team-number">Loading...</span>
                                            <div class="text-sm opacity-75 team-stats">
                                                Loading stats...
                                            </div>
                                        </div>
                                    </div>
                                </div>

                                <div class="text-center flex flex-col justify-center items-center">
                                    <div class="text-2xl font-bold mb-2" id="matchNumber">
                                        Match #
                                    </div>
                                    <div
                                        class="badge badge-neutral"
                                        id="matchTime"
                                        on:click=move |_| {
                                            spawn_local(async {
                                                fetch_data(CURRENT_MATCH as u32).await.unwrap();
                                            });
                                        }
                                    >
                                        Time: TBD
                                    </div>
                                </div>

                                <div class="flex-1">
                                    <h2 class="text-xl font-bold text-center text-primary mb-4 text-blue-600">
                                        Blue Alliance
                                    </h2>
                                    <div class="rounded-lg p-4 space-y-2">
                                        <div class="team-container" id="blue1">
                                            <span class="font-bold">Team 1:</span>
                                            <span class="team-number">Loading...</span>
                                            <div class="text-sm opacity-75 team-stats">
                                                Loading stats...
                                            </div>
                                        </div>
                                        <div class="team-container" id="blue2">
                                            <span class="font-bold">Team 2:</span>
                                            <span class="team-number">Loading...</span>
                                            <div class="text-sm opacity-75 team-stats">
                                                Loading stats...
                                            </div>
                                        </div>
                                        <div class="team-container" id="blue3">
                                            <span class="font-bold">Team 3:</span>
                                            <span class="team-number">Loading...</span>
                                            <div class="text-sm opacity-75 team-stats">
                                                Loading stats...
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>

                            <div class="flex justify-center mt-6">
                                <button class="btn btn-outline" id="refreshNextMatch">
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
