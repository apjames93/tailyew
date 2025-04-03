// src/organisms/stepper.rs

use crate::atoms::{Button, ButtonType, TagType, Typo};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct StepperProps {
    pub steps: Vec<String>,
    #[prop_or(Some(0))]
    pub current_step: Option<usize>,
    #[prop_or_default]
    pub on_step_change: Option<Callback<usize>>,
    #[prop_or(true)]
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

    let internal_step = use_state(|| current_step.unwrap_or(0));

    {
        let internal_step = internal_step.clone();
        use_effect_with(current_step, move |new_step| {
            if let Some(step) = new_step {
                internal_step.set(*step);
            }
            || ()
        });
    }

    let go_to_next_step = {
        let internal_step = internal_step.clone();
        let steps_len = steps.len();
        let on_step_change = on_step_change.clone();

        Callback::from(move |_| {
            if *internal_step < steps_len - 1 {
                let new_index = *internal_step + 1;
                internal_step.set(new_index);
                if let Some(cb) = on_step_change.clone() {
                    cb.emit(new_index);
                }
            }
        })
    };

    let go_to_previous_step = {
        let internal_step = internal_step.clone();
        let on_step_change = on_step_change.clone();

        Callback::from(move |_| {
            if *internal_step > 0 {
                let new_index = *internal_step - 1;
                internal_step.set(new_index);
                if let Some(cb) = on_step_change.clone() {
                    cb.emit(new_index);
                }
            }
        })
    };

    let on_step_click = {
        let internal_step = internal_step.clone();
        let on_step_change = on_step_change.clone();

        Callback::from(move |index: usize| {
            internal_step.set(index);
            if let Some(cb) = on_step_change.clone() {
                cb.emit(index);
            }
        })
    };

    html! {
        <div class="w-full flex flex-col items-center gap-6">
            // Step indicators
            <div class="flex w-full justify-between items-center relative">
                {
                    for steps.iter().enumerate().map(|(index, step)| {
                        let is_active = index == *internal_step;
                        let is_done = index < *internal_step;

                        let circle_class = match (is_active, is_done) {
                            (true, _) => "bg-blue-600 text-white",
                            (false, true) => "bg-green-500 text-white",
                            _ => "bg-gray-300 dark:bg-gray-600 text-gray-800 dark:text-gray-200"
                        };

                        let on_click = {
                            let on_step_click = on_step_click.clone();
                            Callback::from(move |_| on_step_click.emit(index))
                        };

                        html! {
                            <div class="flex-1 flex flex-col items-center relative group">
                                <div
                                    class={classes!("w-10", "h-10","flex","items-center","justify-center","rounded-full","text-sm","font-medium","transition-all","duration-300", circle_class)}
                                    onclick={on_click}
                                >
                                    { index + 1 }
                                </div>

                                <Typo tag={TagType::P} class="text-xs mt-2 text-center">{ step.clone() }</Typo>

                                // Connector line (skip last step)
                                if index < steps.len() - 1 {
                                    <div class="absolute top-5 left-full w-full h-1 bg-gray-300 dark:bg-gray-600 z-[-1]"></div>
                                }
                            </div>
                        }
                    })
                }
            </div>

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
