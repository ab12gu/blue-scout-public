use leptos::prelude::*;
use leptos_meta::*;

use crate::components::PageWrapper;

#[inline]
#[allow(dead_code)]
pub fn extract_checkbox(value: Option<String>) -> bool {
    value.map(|x| x == "on").unwrap_or(false)
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct InsertDataArgs {
    name: String,
    match_number: u16,
    team_number: u32,
    auto_coral: u16,
    auto_algae: u16,
    auto_leave: Option<String>,
    algae_clear: Option<String>,
    l1_coral: u16,
    l2_coral: u16,
    l3_coral: u16,
    l4_coral: u16,
    dropped_coral: u16,
    algae_barge: u16,
    algae_floor_hole: u16,
    climb: String,
    defense_bot: Option<String>,
    notes: String,
}

#[server]
#[allow(clippy::too_many_arguments)]
pub async fn insert_data(args: InsertDataArgs) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::{db::insert_form_data, ClimbType, DataPoint};
        use std::str::FromStr;
        let data_point = DataPoint {
            name: args.name,
            match_number: args.match_number,
            team_number: args.team_number,
            auto_coral: args.auto_coral,
            auto_algae: args.auto_algae,
            auto_leave: extract_checkbox(args.auto_leave),
            algae_clear: extract_checkbox(args.algae_clear),
            l1_coral: args.l1_coral,
            l2_coral: args.l2_coral,
            l3_coral: args.l3_coral,
            l4_coral: args.l4_coral,
            dropped_coral: args.dropped_coral,
            algae_barge: args.algae_barge,
            algae_floor_hole: args.algae_floor_hole,
            climb: match ClimbType::from_str(&args.climb) {
                Ok(climb_type) => climb_type,
                Err(_) => {
                    tracing::error!("Unknown Climb Type: {}", &args.climb);
                    ClimbType::Unknown
                }
            },
            defense_bot: extract_checkbox(args.defense_bot),
            notes: args.notes,
        };

        insert_form_data(data_point).await?;

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
                                    name="args[name]"
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
                                    name="args[match_number]"
                                    required
                                    onkeydown="preventMinusSign(event)"
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
                                    name="args[team_number]"
                                    required
                                    onkeydown="preventMinusSign(event)"
                                />
                            </div>

                            <div class="form-control w-full mb-8">
                                <label class="label pb-2">
                                    <span class="label-text text-lg font-medium">
                                        Auto Coral Number
                                    </span>
                                </label>
                                <input
                                    id="autoCoral"
                                    class="input input-bordered w-full"
                                    type="number"
                                    placeholder="Enter auto coral number"
                                    name="args[auto_coral]"
                                    value="0"
                                    onchange="updateAutoInput('Coral')"
                                    onkeydown="preventMinusSign(event)"
                                />
                            </div>

                            <div class="form-control w-full mb-8">
                                <label class="label pb-2">
                                    <span class="label-text text-lg font-medium">
                                        Auto Algae Number
                                    </span>
                                </label>
                                <input
                                    id="autoAlgae"
                                    class="input input-bordered w-full"
                                    type="number"
                                    placeholder="Enter auto algae number"
                                    name="args[auto_algae]"
                                    value="0"
                                    onchange="updateAutoInput('Algae')"
                                    onkeydown="preventMinusSign(event)"
                                />
                            </div>

                            <div class="form-control w-full mb-8">
                                <label class="label cursor-pointer py-2">
                                    <span class="label-text text-lg font-medium">Auto Leave</span>
                                    <input
                                        type="checkbox"
                                        class="checkbox checkbox-primary"
                                        name="args[auto_leave]"
                                    />
                                </label>
                            </div>

                            <div class="form-control w-full mb-8">
                                <label class="label cursor-pointer py-2">
                                    <span class="label-text text-lg font-medium">Algae Clear</span>
                                    <input
                                        type="checkbox"
                                        class="checkbox checkbox-primary"
                                        name="args[algae_clear]"
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
                                            onkeydown="preventMinusSign(event)"
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
                                            name="args[l1_coral]"
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
                                            onkeydown="preventMinusSign(event)"
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
                                            name="args[l2_coral]"
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
                                            onkeydown="preventMinusSign(event)"
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
                                            name="args[l3_coral]"
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
                                            onkeydown="preventMinusSign(event)"
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
                                            name="args[l4_coral]"
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
                                            onkeydown="preventMinusSign(event)"
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
                                            name="args[dropped_coral]"
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
                                            onkeydown="preventMinusSign(event)"
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
                                            name="args[algae_floor_hole]"
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
                                            onkeydown="preventMinusSign(event)"
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
                                            name="args[algae_barge]"
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
                                            name="args[climb]"
                                            value="Shallow"
                                            class="radio radio-primary"
                                            required
                                        />
                                        <span class="label-text">Shallow</span>
                                    </label>
                                    <label class="label cursor-pointer justify-start gap-3 py-2 mr-5">
                                        <input
                                            type="radio"
                                            name="args[climb]"
                                            value="Deep"
                                            class="radio radio-primary"
                                            required
                                        />
                                        <span class="label-text">Deep</span>
                                    </label>
                                    <label class="label cursor-pointer justify-start gap-3 py-2 mr-5">
                                        <input
                                            type="radio"
                                            name="args[climb]"
                                            value="Park"
                                            class="radio radio-primary"
                                            required
                                        />
                                        <span class="label-text">Park</span>
                                    </label>
                                    <label class="label cursor-pointer justify-start gap-3 py-2 mr-5">
                                        <input
                                            type="radio"
                                            name="args[climb]"
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
                                        name="args[defense_bot]"
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
                                    name="args[notes]"
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
