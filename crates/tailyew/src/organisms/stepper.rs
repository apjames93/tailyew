// src/organisms/stepper.rs

use crate::atoms::{Button, ButtonType, TagType, Typo};
use yew::prelude::*;

/// Properties for the Stepper component
#[derive(Properties, PartialEq, Clone)]
pub struct StepperProps {
    pub steps: Vec<String>, // List of step labels
    #[prop_or(Some(0))]
    pub current_step: Option<usize>, // Optional externally controlled current step
    #[prop_or_default]
    pub on_step_change: Option<Callback<usize>>, // Optional callback for handling step changes (e.g., notifying parent)
    #[prop_or(true)] // Show navigation buttons by default
    pub show_navigation_buttons: bool,
}

#[function_component(Stepper)]
pub fn stepper(props: &StepperProps) -> Html {
    let StepperProps {
        steps,
        current_step,
        on_step_change,
        show_navigation_buttons,
    } = props.clone();

    // Internal state to track the current active step
    let internal_step = use_state(|| current_step.unwrap_or(0));

    // Sync internal state with current_step prop when it changes
    {
        let internal_step = internal_step.clone();
        use_effect_with(current_step, move |current_step| {
            if let Some(step) = current_step {
                internal_step.set(*step);
            }
            || ()
        });
    }

    // Handler for moving to the next step
    let go_to_next_step = {
        let internal_step = internal_step.clone();
        let steps_len = steps.len();
        let on_step_change = on_step_change.clone();
        Callback::from(move |_| {
            if *internal_step < steps_len - 1 {
                let new_step = *internal_step + 1;
                internal_step.set(new_step);
                if let Some(on_step_change) = on_step_change.clone() {
                    on_step_change.emit(new_step);
                }
            }
        })
    };

    // Handler for moving to the previous step
    let go_to_previous_step = {
        let internal_step = internal_step.clone();
        let on_step_change = on_step_change.clone();
        Callback::from(move |_| {
            if *internal_step > 0 {
                let new_step = *internal_step - 1;
                internal_step.set(new_step);
                if let Some(on_step_change) = on_step_change.clone() {
                    on_step_change.emit(new_step);
                }
            }
        })
    };

    // Callback to handle when a step is clicked (optional, if clickable)
    let on_step_click = {
        let internal_step = internal_step.clone();
        let on_step_change = on_step_change.clone();
        Callback::from(move |index: usize| {
            internal_step.set(index);
            if let Some(on_step_change) = on_step_change.clone() {
                on_step_change.emit(index);
            }
        })
    };

    html! {
        <div class="flex flex-col items-center space-y-4">
            // Stepper Header with Steps
            <div class="w-full flex items-center justify-between mb-4 space-x-2">
                { for steps.iter().enumerate().map(|(index, step)| {
                    let is_current = index == *internal_step;
                    let is_completed = index < *internal_step;

                    // Determine step styling based on state
                    let step_class = if is_current {
                        "bg-blue-600 text-white"
                    } else if is_completed {
                        "bg-green-500 text-white"
                    } else {
                        "bg-gray-300 dark:bg-gray-600 text-gray-800 dark:text-gray-200"
                    };

                    // Clone callback for use in the closure
                    let on_step_click = on_step_click.clone();

                    html! {
                        <div class="flex-1 flex flex-col items-center">
                            // Step Circle with Number
                            <div
                                class={classes!(
                                   "w-12", "h-12", "flex", "items-center", "justify-center", "rounded-full", "transition-all", "duration-300", "ease-in-out",
                                    step_class
                                )}
                                onclick={Callback::from(move |_| on_step_click.emit(index))}
                            >
                                { index + 1 }
                            </div>

                            // Step Label
                            <Typo tag={TagType::P} class="text-center mt-2">
                                { step.to_string() }
                            </Typo>

                            // Line Connector (if not the last step, show a connecting line)
                            if index < steps.len() - 1 {
                                <div class="flex-1 h-1 bg-gray-300 dark:bg-gray-600 mx-2 mt-4 w-full"></div>
                            }
                        </div>
                    }
                })}
            </div>

            // Stepper Navigation Buttons (conditionally rendered based on show_navigation_buttons prop)
            if show_navigation_buttons {
                <div class="flex space-x-4">
                    <Button
                        button_type={ButtonType::Secondary}
                        disabled={*internal_step == 0}
                        onclick={go_to_previous_step}
                    >
                        { "Previous" }
                    </Button>

                    <Button
                        button_type={ButtonType::Primary}
                        disabled={*internal_step == steps.len() - 1}
                        onclick={go_to_next_step}
                    >
                        { "Next" }
                    </Button>
                </div>
            }
        </div>
    }
}
