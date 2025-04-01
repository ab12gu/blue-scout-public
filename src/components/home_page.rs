use leptos::{ev, prelude::*};
use leptos_meta::*;

use crate::{components::PageWrapper, data::InsertDataArgs};

#[server]
pub async fn insert_data(args: InsertDataArgs) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::db::insert_form_data;
        insert_form_data(args.map_insert_data_args()).await?;

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
    let l1_coral_count = RwSignal::new(0_usize);
    let l2_coral_count = RwSignal::new(0_usize);
    let l3_coral_count = RwSignal::new(0_usize);
    let l4_coral_count = RwSignal::new(0_usize);
    let dropped_coral_count = RwSignal::new(0_usize);
    let barge_algae_count = RwSignal::new(0_usize);
    let floor_hole_algae_count = RwSignal::new(0_usize);
    let increment_coral_closure = |x: &mut usize| {
        if *x < 12 {
            *x += 1;
        }
    };
    let increment_closure = |x: &mut usize| {
        *x += 1;
    };
    let decrement_closure = |x: &mut usize| {
        if *x > 0 {
            *x -= 1;
        }
    };

    fn create_handle_input_change(value: RwSignal<usize>) -> impl Fn(ev::Event) {
        move |ev: ev::Event| {
            let input_string = event_target_value(&ev);
            let new_value = input_string.parse::<usize>().unwrap_or(0);
            value.set(new_value.max(0));
        }
    }

    let reset_counters = move |_: ev::MouseEvent| {
        l1_coral_count.set(0);
        l2_coral_count.set(0);
        l3_coral_count.set(0);
        l4_coral_count.set(0);
        dropped_coral_count.set(0);
        floor_hole_algae_count.set(0);
        barge_algae_count.set(0);
    };

    let prevent_minus_sign = |ev: ev::KeyboardEvent| {
        if ev.key() == "-" {
            ev.prevent_default();
        }
    };
    
    

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
                                    on:keydown=prevent_minus_sign
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
                                    on:keydown=prevent_minus_sign
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
                                    on:keydown=prevent_minus_sign
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
                                    on:keydown=prevent_minus_sign
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
                                            on:click=move |_| {
                                                l1_coral_count.update(increment_closure);
                                            }
                                        >
                                            +
                                        </button>
                                        <input
                                            id="coralL1Count"
                                            class="input input-bordered w-16 text-center py-1 text-xl transparent-num"
                                            type="number"
                                            min="0"
                                            prop:value=move || l1_coral_count.get().to_string()
                                            on:change=create_handle_input_change(l1_coral_count)
                                            on:keydown=prevent_minus_sign
                                        />
                                        <button
                                            type="button"
                                            class="btn btn-sm btn-soft mt-1 outline-none"
                                            on:click=move |_| {
                                                l1_coral_count.update(decrement_closure);
                                            }
                                        >
                                            -
                                        </button>
                                        <input
                                            type="hidden"
                                            name="args[l1_coral]"
                                            id="coralL1Input"
                                            prop:value=move || l1_coral_count.get().to_string()
                                        />
                                    </div>
                                    <div class="flex flex-col items-center">
                                        <span class="label-text mb-1 text-2xl">L2</span>
                                        <button
                                            type="button"
                                            class="btn btn-sm btn-soft mb-1 outline-none"
                                            on:click=move |_| {
                                                l2_coral_count.update(increment_coral_closure);
                                            }
                                        >
                                            +
                                        </button>
                                        <input
                                            id="coralL2Count"
                                            class="input input-bordered w-16 text-center py-1 text-xl transparent-num"
                                            type="number"
                                            prop:value=move || l2_coral_count.get().to_string()
                                            min="0"
                                            max="12"
                                            on:change=create_handle_input_change(l2_coral_count)
                                            on:keydown=prevent_minus_sign
                                        />
                                        <button
                                            type="button"
                                            class="btn btn-sm btn-soft mt-1 outline-none"
                                            on:click=move |_| {
                                                l2_coral_count.update(decrement_closure);
                                            }
                                        >
                                            -
                                        </button>
                                        <input
                                            type="hidden"
                                            name="args[l2_coral]"
                                            id="coralL2Input"
                                            prop:value=move || l2_coral_count.get().to_string()
                                        />
                                    </div>
                                    <div class="flex flex-col items-center">
                                        <span class="label-text mb-1 text-2xl">L3</span>
                                        <button
                                            type="button"
                                            class="btn btn-sm btn-soft mb-1 outline-none"
                                            on:click=move |_| {
                                                l3_coral_count.update(increment_coral_closure);
                                            }
                                        >
                                            +
                                        </button>
                                        <input
                                            id="coralL3Count"
                                            class="input input-bordered w-16 text-center py-1 text-xl transparent-num"
                                            type="number"
                                            prop:value=move || l3_coral_count.get().to_string()
                                            min="0"
                                            max="12"
                                            on:change=create_handle_input_change(l3_coral_count)
                                            on:keydown=prevent_minus_sign
                                        />
                                        <button
                                            type="button"
                                            class="btn btn-sm btn-soft mt-1 outline-none"
                                            on:click=move |_| {
                                                l3_coral_count.update(decrement_closure);
                                            }
                                        >
                                            -
                                        </button>
                                        <input
                                            type="hidden"
                                            name="args[l3_coral]"
                                            id="coralL3Input"
                                            prop:value=move || l3_coral_count.get().to_string()
                                        />
                                    </div>
                                    <div class="flex flex-col items-center">
                                        <span class="label-text mb-1 text-2xl">L4</span>
                                        <button
                                            type="button"
                                            class="btn btn-sm btn-soft mb-1 outline-none"
                                            on:click=move |_| {
                                                l4_coral_count.update(increment_coral_closure);
                                            }
                                        >
                                            +
                                        </button>
                                        <input
                                            id="coralL4Count"
                                            class="input input-bordered w-16 text-center py-1 text-xl transparent-num"
                                            type="number"
                                            prop:value=move || l4_coral_count.get().to_string()
                                            min="0"
                                            max="12"
                                            on:change=create_handle_input_change(l4_coral_count)
                                            on:keydown=prevent_minus_sign
                                        />
                                        <button
                                            type="button"
                                            class="btn btn-sm btn-soft mt-1 outline-none"
                                            on:click=move |_| {
                                                l4_coral_count.update(decrement_closure);
                                            }
                                        >
                                            -
                                        </button>
                                        <input
                                            type="hidden"
                                            name="args[l4_coral]"
                                            id="coralL4Input"
                                            prop:value=move || l4_coral_count.get().to_string()
                                        />
                                    </div>
                                    <div class="flex flex-col items-center">
                                        <span class="label-text mb-1 text-2xl">Dropped</span>
                                        <button
                                            type="button"
                                            class="btn btn-sm btn-soft mb-1 outline-none"
                                            on:click=move |_| {
                                                dropped_coral_count.update(increment_closure);
                                            }
                                        >
                                            +
                                        </button>
                                        <input
                                            id="coralDroppedCount"
                                            class="input input-bordered w-16 text-center py-1 text-xl transparent-num"
                                            type="number"
                                            prop:value=move || dropped_coral_count.get().to_string()
                                            min="0"
                                            on:change=create_handle_input_change(dropped_coral_count)
                                            on:keydown=prevent_minus_sign
                                        />
                                        <button
                                            type="button"
                                            class="btn btn-sm btn-soft mt-1 outline-none"
                                            on:click=move |_| {
                                                dropped_coral_count.update(decrement_closure);
                                            }
                                        >
                                            -
                                        </button>
                                        <input
                                            type="hidden"
                                            name="args[dropped_coral]"
                                            id="coralDroppedInput"
                                            prop:value=move || dropped_coral_count.get().to_string()
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
                                            on:click=move |_| {
                                                floor_hole_algae_count.update(increment_closure);
                                            }
                                        >
                                            +
                                        </button>
                                        <input
                                            id="algaeFloorHoleCount"
                                            class="input input-bordered w-16 text-center py-1 text-xl transparent-num"
                                            type="number"
                                            min="0"
                                            prop:value=move || floor_hole_algae_count.get().to_string()
                                            on:change=create_handle_input_change(floor_hole_algae_count)
                                            on:keydown=prevent_minus_sign
                                        />
                                        <button
                                            type="button"
                                            class="btn btn-sm btn-soft mt-1 outline-none"
                                            on:click=move |_| {
                                                floor_hole_algae_count.update(decrement_closure);
                                            }
                                        >
                                            -
                                        </button>
                                        <input
                                            type="hidden"
                                            name="args[algae_floor_hole]"
                                            id="algaeFloorHoleInput"
                                            prop:value=move || floor_hole_algae_count.get().to_string()
                                        />
                                    </div>
                                    <div class="flex flex-col items-center">
                                        <span class="label-text mb-1 text-2xl">Barge</span>
                                        <button
                                            type="button"
                                            class="btn btn-sm btn-soft mb-1 outline-none"
                                            on:click=move |_| {
                                                barge_algae_count.update(increment_closure);
                                            }
                                        >
                                            +
                                        </button>
                                        <input
                                            id="algaeBargeCount"
                                            class="input input-bordered w-16 text-center py-1 text-xl transparent-num"
                                            type="number"
                                            prop:value=move || barge_algae_count.get().to_string()
                                            min="0"
                                            on:change=create_handle_input_change(barge_algae_count)
                                            on:keydown=prevent_minus_sign
                                        />
                                        <button
                                            type="button"
                                            class="btn btn-sm btn-soft mt-1 outline-none"
                                            on:click=move |_| {
                                                barge_algae_count.update(decrement_closure);
                                            }
                                        >
                                            -
                                        </button>
                                        <input
                                            type="hidden"
                                            name="args[algae_barge]"
                                            id="algaeBargeInput"
                                            prop:value=move || barge_algae_count.get().to_string()
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
                                <button type="reset" class="btn btn-lg" on:click=reset_counters>
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
