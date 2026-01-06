// frontend/src/templates/demos/switch_demo/switch_demo_section.rs

use crate::templates::demos::DemoComponent;
use tailyew::form::Switch;
use tailyew::organisms::table::Column;
use yew::prelude::*;

const USAGE_CODE: &str = r#"
use tailyew::form::Switch;
use yew::prelude::*;

#[component(SwitchUsage)]
fn switch_usage() -> Html {
    let state = use_state(|| false);
    let on_change = {
        let state = state.clone();
        Callback::from(move |val: bool| state.set(val))
    };

    html! {
        <Switch
            id="notifications"
            label="Enable notifications"
            description="Receive important updates by email."
            checked={*state}
            on_change={Some(on_change)}
        />
    }
}
"#;

#[component(SwitchDemoSection)]
pub fn switch_demo_section() -> Html {
    let example = html! { <SwitchUsage /> };

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "id".into(),
                "label".into(),
                "checked".into(),
                "required".into(),
                "description".into(),
                "disabled".into(),
                "aria-describedby".into(),
                "aria-label".into(),
                "aria-labelledby".into(),
                "on_change".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "String".into(),
                "String".into(),
                "bool".into(),
                "bool".into(),
                "Option<String>".into(),
                "bool".into(),
                "Option<String>".into(),
                "Option<String>".into(),
                "Option<String>".into(),
                "Option<Callback<bool>>".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "The HTML id/name for the underlying input.".into(),
                "Text label displayed next to the switch.".into(),
                "Initial on/off state (acts like a default value, can be used in controlled mode).".into(),
                "Whether this field is required for form submission.".into(),
                "Optional helper text shown below the control.".into(),
                "Disables the switch when true.".into(),
                "ID of an element that provides additional description; defaults to `${id}-description` when `description` is provided.".into(),
                "Accessible label for the switch; falls back to the visible `label` text.".into(),
                "ID of an element that labels the switch; defaults to `${id}-label`.".into(),
                "Called when the switch value changes, receiving the new `bool` state.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            github_demo_path="form/switch_demo_section.rs"
            github_source_path="form/switch.rs"
            title="Switch Component"
            description={Some(html! {
              <>
                <p>{ "The `Switch` component is a stylized boolean control built on top of a checkbox input." }</p>
                <p>{ "Use it when you want a modern toggle UI while preserving standard HTML form behavior, validation, and strong accessibility via ARIA attributes." }</p>
              </>
            })}
            example={example}
            usage_code={USAGE_CODE}
            props_table={Some(props_table)}
        />
    }
}

#[component(SwitchUsage)]
fn switch_usage() -> Html {
    let state = use_state(|| false);
    let on_change = {
        let state = state.clone();
        Callback::from(move |val: bool| state.set(val))
    };

    html! {
        <Switch
            id="notifications"
            label="Enable notifications"
            description="Receive important updates by email."
            checked={*state}
            on_change={Some(on_change)}
        />
    }
}
