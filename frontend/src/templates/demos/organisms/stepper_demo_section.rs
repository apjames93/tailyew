use crate::templates::demos::DemoComponent;
use tailyew::atoms::{TagType, Typo};
use tailyew::organisms::{table::Column, Stepper};
use yew::prelude::*;

#[function_component(StepperDemoSection)]
pub fn stepper_demo_section() -> Html {
    let steps = vec![
        "Start".to_string(),
        "Details".to_string(),
        "Confirmation".to_string(),
        "Finish".to_string(),
    ];

    let example = html! {
        <div class="p-6 bg-white dark:bg-gray-800 rounded-lg shadow-lg max-w-3xl mx-auto space-y-6">
            <Typo tag={TagType::H1}>{ "Stepper Component Demo" }</Typo>
            <Stepper steps={steps.clone()} current_step={Some(1)} />
        </div>
    };

    let usage_code = r#"
let steps = vec![
    "Start".to_string(),
    "Details".to_string(),
    "Confirmation".to_string(),
    "Finish".to_string(),
];

<Stepper
    steps={steps}
    current_step={Some(1)}
    show_navigation_buttons={true}
/>
"#;

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "steps".into(),
                "current_step".into(),
                "on_step_change".into(),
                "show_navigation_buttons".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "Vec<String>".into(),
                "Option<usize>".into(),
                "Option<Callback<usize>>".into(),
                "bool".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "List of step labels.".into(),
                "The current step index (0-based).".into(),
                "Callback triggered when step changes.".into(),
                "Whether to show Prev/Next buttons.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            title="Stepper Component"
            description={Some(html! {
                <p>{"The `Stepper` visually guides users through a multi-step process. Supports interactive steps and optional navigation buttons."}</p>
            })}
            example={example}
            usage_code={usage_code}
            props_table={Some(props_table)}
        />
    }
}
