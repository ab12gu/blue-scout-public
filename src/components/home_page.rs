use leptos::prelude::*;

use crate::components::PageWrapper;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <PageWrapper>
            <div class="container mx-auto max-w-3xl">
                <h1 class="text-3xl font-bold text-center mb-8">4682 Scouting Form</h1>

                <div class="card bg-base-200 shadow-xl">
                    <div class="card-body p-8">
                        <form id="scoutingForm">
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
                                    name="matchNumber"
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
                                    name="teamNumber"
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
                                    name="autoAlgae"
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
                                    name="autoCoralNumber"
                                />
                            </div>

                            <div class="form-control w-full mb-8">
                                <label class="label cursor-pointer py-2">
                                    <span class="label-text text-lg font-medium">Auto Leave</span>
                                    <input
                                        type="checkbox"
                                        class="checkbox checkbox-primary"
                                        name="autoLeave"
                                    />
                                </label>
                            </div>

                            <div class="form-control w-full mb-8">
                                <label class="label cursor-pointer py-2">
                                    <span class="label-text text-lg font-medium">Algae Clear</span>
                                    <input
                                        type="checkbox"
                                        class="checkbox checkbox-primary"
                                        name="algaeClear"
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
                                            class="btn btn-sm btn-soft mb-1"
                                            onclick="incrementCoral('L1')"
                                        >
                                            +
                                        </button>
                                        <input
                                            id="coralL1Count"
                                            class="input input-bordered w-16 text-center py-1 text-xl transparent-num"
                                            type="number"
                                            value="0"
                                            min="0"
                                            onchange="updateCoralInput('L1')"
                                        />
                                        <button
                                            type="button"
                                            class="btn btn-sm btn-soft mt-1"
                                            onclick="decrementCoral('L1')"
                                        >
                                            -
                                        </button>
                                        <input
                                            type="hidden"
                                            name="coralL1"
                                            id="coralL1Input"
                                            value="0"
                                        />
                                    </div>
                                    <div class="flex flex-col items-center">
                                        <span class="label-text mb-1 text-2xl">L2</span>
                                        <button
                                            type="button"
                                            class="btn btn-sm btn-soft mb-1"
                                            onclick="incrementCoral('L2')"
                                        >
                                            +
                                        </button>
                                        <input
                                            id="coralL2Count"
                                            class="input input-bordered w-16 text-center py-1 text-xl transparent-num"
                                            type="number"
                                            value="0"
                                            min="0"
                                            max="12"
                                            onchange="updateCoralInput('L2')"
                                        />
                                        <button
                                            type="button"
                                            class="btn btn-sm btn-soft mt-1"
                                            onclick="decrementCoral('L2')"
                                        >
                                            -
                                        </button>
                                        <input
                                            type="hidden"
                                            name="coralL2"
                                            id="coralL2Input"
                                            value="0"
                                        />
                                    </div>
                                    <div class="flex flex-col items-center">
                                        <span class="label-text mb-1 text-2xl">L3</span>
                                        <button
                                            type="button"
                                            class="btn btn-sm btn-soft mb-1"
                                            onclick="incrementCoral('L3')"
                                        >
                                            +
                                        </button>
                                        <input
                                            id="coralL3Count"
                                            class="input input-bordered w-16 text-center py-1 text-xl transparent-num"
                                            type="number"
                                            value="0"
                                            min="0"
                                            max="12"
                                            onchange="updateCoralInput('L3')"
                                        />
                                        <button
                                            type="button"
                                            class="btn btn-sm btn-soft mt-1"
                                            onclick="decrementCoral('L3')"
                                        >
                                            -
                                        </button>
                                        <input
                                            type="hidden"
                                            name="coralL3"
                                            id="coralL3Input"
                                            value="0"
                                        />
                                    </div>
                                    <div class="flex flex-col items-center">
                                        <span class="label-text mb-1 text-2xl">L4</span>
                                        <button
                                            type="button"
                                            class="btn btn-sm btn-soft mb-1"
                                            onclick="incrementCoral('L4')"
                                        >
                                            +
                                        </button>
                                        <input
                                            id="coralL4Count"
                                            class="input input-bordered w-16 text-center py-1 text-xl transparent-num"
                                            type="number"
                                            value="0"
                                            min="0"
                                            max="12"
                                            onchange="updateCoralInput('L4')"
                                        />
                                        <button
                                            type="button"
                                            class="btn btn-sm btn-soft mt-1"
                                            onclick="decrementCoral('L4')"
                                        >
                                            -
                                        </button>
                                        <input
                                            type="hidden"
                                            name="coralL4"
                                            id="coralL4Input"
                                            value="0"
                                        />
                                    </div>
                                    <div class="flex flex-col items-center">
                                        <span class="label-text mb-1 text-2xl">Dropped</span>
                                        <button
                                            type="button"
                                            class="btn btn-sm btn-soft mb-1"
                                            onclick="incrementDroppedCoral()"
                                        >
                                            +
                                        </button>
                                        <input
                                            id="coralDroppedCount"
                                            class="input input-bordered w-16 text-center py-1 text-xl transparent-num"
                                            type="number"
                                            value="0"
                                            min="0"
                                            onchange="updateDroppedCoralInput()"
                                        />
                                        <button
                                            type="button"
                                            class="btn btn-sm btn-soft mt-1"
                                            onclick="decrementDroppedCoral()"
                                        >
                                            -
                                        </button>
                                        <input
                                            type="hidden"
                                            name="coralDropped"
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
                                            class="btn btn-sm btn-soft mb-1"
                                            onclick="incrementAlgae('FloorHole')"
                                        >
                                            +
                                        </button>
                                        <input
                                            id="algaeFloorHoleCount"
                                            class="input input-bordered w-16 text-center py-1 text-xl transparent-num"
                                            type="number"
                                            value="0"
                                            min="0"
                                            onchange="updateAlgaeInput('FloorHole')"
                                        />
                                        <button
                                            type="button"
                                            class="btn btn-sm btn-soft mt-1"
                                            onclick="decrementAlgae('FloorHole')"
                                        >
                                            -
                                        </button>
                                        <input
                                            type="hidden"
                                            name="algaeFloorHole"
                                            id="algaeFloorHoleInput"
                                            value="0"
                                        />
                                    </div>
                                    <div class="flex flex-col items-center">
                                        <span class="label-text mb-1 text-2xl">Barge</span>
                                        <button
                                            type="button"
                                            class="btn btn-sm btn-soft mb-1"
                                            onclick="incrementAlgae('Barge')"
                                        >
                                            +
                                        </button>
                                        <input
                                            id="algaeBargeCount"
                                            class="input input-bordered w-16 text-center py-1 text-xl transparent-num"
                                            type="number"
                                            value="0"
                                            min="0"
                                            onchange="updateAlgaeInput('Barge')"
                                        />
                                        <button
                                            type="button"
                                            class="btn btn-sm btn-soft mt-1"
                                            onclick="decrementAlgae('Barge')"
                                        >
                                            -
                                        </button>
                                        <input
                                            type="hidden"
                                            name="algaeBarge"
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
                                        />
                                        <span class="label-text">Shallow</span>
                                    </label>
                                    <label class="label cursor-pointer justify-start gap-3 py-2 mr-5">
                                        <input
                                            type="radio"
                                            name="climb"
                                            value="Deep"
                                            class="radio radio-primary"
                                        />
                                        <span class="label-text">Deep</span>
                                    </label>
                                    <label class="label cursor-pointer justify-start gap-3 py-2 mr-5">
                                        <input
                                            type="radio"
                                            name="climb"
                                            value="Park"
                                            class="radio radio-primary"
                                        />
                                        <span class="label-text">Park</span>
                                    </label>
                                    <label class="label cursor-pointer justify-start gap-3 py-2 mr-5">
                                        <input
                                            type="radio"
                                            name="climb"
                                            value="Not Attempted"
                                            class="radio radio-primary"
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
                                        name="defenseBot"
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
                        </form>
                    </div>
                </div>
            </div>
        </PageWrapper>
    }
}
