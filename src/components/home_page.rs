use leptos::prelude::*;
use leptos_meta::*;

use crate::components::PageWrapper;

#[server]
#[allow(clippy::too_many_arguments)]
pub async fn insert_data(
    name: String,
    match_number: u32,
    team_number: u32,
    auto_algae: u32,
    auto_coral: u32,
    auto_leave: Option<String>,
    algae_clear: Option<String>,
    l1_coral: u32,
    l2_coral: u32,
    l3_coral: u32,
    l4_coral: u32,
    dropped_coral: u32,
    algae_barge: u32,
    algae_floor_hole: u32,
    climb: String,
    defense_bot: Option<String>,
    notes: String,
) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        Ok(())
    }
    #[cfg(not(feature = "ssr"))]
    {
        tracing::error!("Server function called without ssr feature enabled");
        unreachable!("This should only be called on the server");
    }
}

#[component]
pub fn HomePage() -> impl IntoView {
    let insert_data = ServerAction::<InsertData>::new();

    view! {
        <Script src="/home.js"></Script>
        <PageWrapper>
            <div class="container mx-auto max-w-3xl">
                <h1 class="text-3xl font-bold text-center mb-8">4682 Scouting Form</h1>

                <div class="card bg-base-200 shadow-xl">
                    <div class="card-body p-8">
                        <ActionForm action=insert_data>
                            <div class="form-control w-full mb-8">
                                <label class="label pb-2">
                                    <span class="label-text text-lg font-medium">Name</span>
                                </label>
                                <input
                                    class="input input-bordered w-full"
                                    type="text"
                                    placeholder="Enter your name"
                                    name="name"
                                    required
                                />
                            </div>

                            <div class="form-control w-full mb-8">
                                <label class="label pb-2">
                                    <span class="label-text text-lg font-medium">Match Number</span>
                                </label>
                                <input
                                    class="input input-bordered w-full"
                                    type="number"
                                    placeholder="Enter match number"
                                    name="match_number"
                                    required
                                />
                            </div>

                            <div class="form-control w-full mb-8">
                                <label class="label pb-2">
                                    <span class="label-text text-lg font-medium">Team Number</span>
                                </label>
                                <input
                                    class="input input-bordered w-full"
                                    type="number"
                                    placeholder="Enter team number"
                                    name="team_number"
                                    required
                                />
                            </div>

                            <div class="form-control w-full mb-8">
                                <label class="label pb-2">
                                    <span class="label-text text-lg font-medium">
                                        Auto Algae Number
                                    </span>
                                </label>
                                <input
                                    class="input input-bordered w-full"
                                    type="number"
                                    placeholder="Enter auto algae number"
                                    name="auto_algae"
                                    value="0"
                                    onchange="updateAutoInput('algae')"
                                />
                            </div>

                            <div class="form-control w-full mb-8">
                                <label class="label pb-2">
                                    <span class="label-text text-lg font-medium">
                                        Auto Coral Number
                                    </span>
                                </label>
                                <input
                                    class="input input-bordered w-full"
                                    type="number"
                                    placeholder="Enter auto coral number"
                                    name="auto_coral"
                                    value="0"
                                    onchange="updateAutoInput('coral')"
                                />
                            </div>

                            <div class="form-control w-full mb-8">
                                <label class="label cursor-pointer py-2">
                                    <span class="label-text text-lg font-medium">Auto Leave</span>
                                    <input
                                        type="checkbox"
                                        class="checkbox checkbox-primary"
                                        name="auto_leave"
                                    />
                                </label>
                            </div>

                            <div class="form-control w-full mb-8">
                                <label class="label cursor-pointer py-2">
                                    <span class="label-text text-lg font-medium">Algae Clear</span>
                                    <input
                                        type="checkbox"
                                        class="checkbox checkbox-primary"
                                        name="algae_clear"
                                    />
                                </label>
                            </div>

                            <div class="form-control w-full mb-8">
                                <label class="label pb-2">
                                    <span class="label-text text-lg font-medium">Teleop Coral</span>
                                </label>
                                <div class="grid grid-cols-5 gap-4 ml-4 mt-3">
                                    <div class="flex flex-col items-center">
                                        <span class="label-text mb-1 text-2xl">L1</span>
                                        <button
                                            type="button"
                                            class="btn btn-sm btn-soft mb-1 outline-none"
                                            onclick="incrementCoral('L1')"
                                        >
                                            +
                                        </button>
                                        <input
                                            id="coralL1Count"
                                            class="input input-bordered w-16 text-center py-1 text-xl transparent-num mr-3"
                                            type="number"
                                            value="0"
                                            min="0"
                                            onchange="updateCoralInput('L1')"
                                        />
                                        <button
                                            type="button"
                                            class="btn btn-sm btn-soft mt-1 outline-none"
                                            onclick="decrementCoral('L1')"
                                        >
                                            -
                                        </button>
                                        <input
                                            type="hidden"
                                            name="l1_coral"
                                            id="coralL1Input"
                                            value="0"
                                        />
                                    </div>
                                    <div class="flex flex-col items-center">
                                        <span class="label-text mb-1 text-2xl">L2</span>
                                        <button
                                            type="button"
                                            class="btn btn-sm btn-soft mb-1 outline-none"
                                            onclick="incrementCoral('L2')"
                                        >
                                            +
                                        </button>
                                        <input
                                            id="coralL2Count"
                                            class="input input-bordered w-16 text-center py-1 text-xl transparent-num mr-3"
                                            type="number"
                                            value="0"
                                            min="0"
                                            max="12"
                                            onchange="updateCoralInput('L2')"
                                        />
                                        <button
                                            type="button"
                                            class="btn btn-sm btn-soft mt-1 outline-none"
                                            onclick="decrementCoral('L2')"
                                        >
                                            -
                                        </button>
                                        <input
                                            type="hidden"
                                            name="l2_coral"
                                            id="coralL2Input"
                                            value="0"
                                        />
                                    </div>
                                    <div class="flex flex-col items-center">
                                        <span class="label-text mb-1 text-2xl">L3</span>
                                        <button
                                            type="button"
                                            class="btn btn-sm btn-soft mb-1 outline-none"
                                            onclick="incrementCoral('L3')"
                                        >
                                            +
                                        </button>
                                        <input
                                            id="coralL3Count"
                                            class="input input-bordered w-16 text-center py-1 text-xl transparent-num mr-3"
                                            type="number"
                                            value="0"
                                            min="0"
                                            max="12"
                                            onchange="updateCoralInput('L3')"
                                        />
                                        <button
                                            type="button"
                                            class="btn btn-sm btn-soft mt-1 outline-none"
                                            onclick="decrementCoral('L3')"
                                        >
                                            -
                                        </button>
                                        <input
                                            type="hidden"
                                            name="l3_coral"
                                            id="coralL3Input"
                                            value="0"
                                        />
                                    </div>
                                    <div class="flex flex-col items-center">
                                        <span class="label-text mb-1 text-2xl">L4</span>
                                        <button
                                            type="button"
                                            class="btn btn-sm btn-soft mb-1 outline-none"
                                            onclick="incrementCoral('L4')"
                                        >
                                            +
                                        </button>
                                        <input
                                            id="coralL4Count"
                                            class="input input-bordered w-16 text-center py-1 text-xl transparent-num mr-3"
                                            type="number"
                                            value="0"
                                            min="0"
                                            max="12"
                                            onchange="updateCoralInput('L4')"
                                        />
                                        <button
                                            type="button"
                                            class="btn btn-sm btn-soft mt-1 outline-none"
                                            onclick="decrementCoral('L4')"
                                        >
                                            -
                                        </button>
                                        <input
                                            type="hidden"
                                            name="l4_coral"
                                            id="coralL4Input"
                                            value="0"
                                        />
                                    </div>
                                    <div class="flex flex-col items-center">
                                        <span class="label-text mb-1 text-2xl">Dropped</span>
                                        <button
                                            type="button"
                                            class="btn btn-sm btn-soft mb-1 outline-none"
                                            onclick="incrementDroppedCoral()"
                                        >
                                            +
                                        </button>
                                        <input
                                            id="coralDroppedCount"
                                            class="input input-bordered w-16 text-center py-1 text-xl transparent-num mr-3"
                                            type="number"
                                            value="0"
                                            min="0"
                                            onchange="updateDroppedCoralInput()"
                                        />
                                        <button
                                            type="button"
                                            class="btn btn-sm btn-soft mt-1 outline-none"
                                            onclick="decrementDroppedCoral()"
                                        >
                                            -
                                        </button>
                                        <input
                                            type="hidden"
                                            name="dropped_coral"
                                            id="coralDroppedInput"
                                            value="0"
                                        />
                                    </div>
                                </div>
                            </div>

                            <div class="form-control w-full mb-8">
                                <label class="label pb-2">
                                    <span class="label-text text-lg font-medium">Teleop Algae</span>
                                </label>
                                <div class="grid grid-cols-2 gap-4 ml-4 mt-3">
                                    <div class="flex flex-col items-center">
                                        <span class="label-text mb-1 text-2xl">Floor Hole</span>
                                        <button
                                            type="button"
                                            class="btn btn-sm btn-soft mb-1 outline-none"
                                            onclick="incrementAlgae('FloorHole')"
                                        >
                                            +
                                        </button>
                                        <input
                                            id="algaeFloorHoleCount"
                                            class="input input-bordered w-16 text-center py-1 text-xl transparent-num mr-3"
                                            type="number"
                                            value="0"
                                            min="0"
                                            onchange="updateAlgaeInput('FloorHole')"
                                        />
                                        <button
                                            type="button"
                                            class="btn btn-sm btn-soft mt-1 outline-none"
                                            onclick="decrementAlgae('FloorHole')"
                                        >
                                            -
                                        </button>
                                        <input
                                            type="hidden"
                                            name="algae_floor_hole"
                                            id="algaeFloorHoleInput"
                                            value="0"
                                        />
                                    </div>
                                    <div class="flex flex-col items-center">
                                        <span class="label-text mb-1 text-2xl">Barge</span>
                                        <button
                                            type="button"
                                            class="btn btn-sm btn-soft mb-1 outline-none"
                                            onclick="incrementAlgae('Barge')"
                                        >
                                            +
                                        </button>
                                        <input
                                            id="algaeBargeCount"
                                            class="input input-bordered w-16 text-center py-1 text-xl transparent-num mr-3"
                                            type="number"
                                            value="0"
                                            min="0"
                                            onchange="updateAlgaeInput('Barge')"
                                        />
                                        <button
                                            type="button"
                                            class="btn btn-sm btn-soft mt-1 outline-none"
                                            onclick="decrementAlgae('Barge')"
                                        >
                                            -
                                        </button>
                                        <input
                                            type="hidden"
                                            name="algae_barge"
                                            id="algaeBargeInput"
                                            value="0"
                                        />
                                    </div>
                                </div>
                            </div>

                            <div class="form-control w-full">
                                <label class="label pb-2">
                                    <span class="label-text text-lg font-medium">Climb?</span>
                                </label>
                                <div class="ml-4 space-y-3 mt-3">
                                    <label class="label cursor-pointer justify-start gap-3 py-2 mr-5">
                                        <input
                                            type="radio"
                                            name="climb"
                                            value="Shallow"
                                            class="radio radio-primary"
                                            required
                                        />
                                        <span class="label-text">Shallow</span>
                                    </label>
                                    <label class="label cursor-pointer justify-start gap-3 py-2 mr-5">
                                        <input
                                            type="radio"
                                            name="climb"
                                            value="Deep"
                                            class="radio radio-primary"
                                            required
                                        />
                                        <span class="label-text">Deep</span>
                                    </label>
                                    <label class="label cursor-pointer justify-start gap-3 py-2 mr-5">
                                        <input
                                            type="radio"
                                            name="climb"
                                            value="Park"
                                            class="radio radio-primary"
                                            required
                                        />
                                        <span class="label-text">Park</span>
                                    </label>
                                    <label class="label cursor-pointer justify-start gap-3 py-2 mr-5">
                                        <input
                                            type="radio"
                                            name="climb"
                                            value="Not Attempted"
                                            class="radio radio-primary"
                                            required
                                        />
                                        <span class="label-text">Not Attempted</span>
                                    </label>
                                </div>
                            </div>

                            <div class="form-control w-full mb-4">
                                <label class="label cursor-pointer py-2">
                                    <span class="label-text text-lg font-medium">Defense Bot</span>
                                    <input
                                        type="checkbox"
                                        class="checkbox checkbox-primary"
                                        name="defense_bot"
                                    />
                                </label>
                            </div>

                            <div class="form-control w-full mb-8">
                                <label class="label pb-2">
                                    <span class="label-text text-lg font-medium">Notes</span>
                                </label>
                                <textarea
                                    class="textarea textarea-bordered w-full h-32"
                                    placeholder="Additional notes"
                                    name="notes"
                                ></textarea>
                            </div>

                            <div class="flex justify-center gap-6 mt-10">
                                <button type="submit" class="btn btn-primary btn-lg">
                                    Submit
                                </button>
                                <button
                                    type="reset"
                                    class="btn btn-lg"
                                    onclick="resetCoralCounters()"
                                >
                                    Reset
                                </button>
                            </div>
                        </ActionForm>
                    </div>
                </div>
            </div>
        </PageWrapper>
    }
}
