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
            name="notifications_enabled"
            label="Enable notifications"
            helper_text="Receive important updates by email."
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
                "name".into(),
                "label".into(),
                "checked".into(),
                "required".into(),
                "description".into(),
                "helper_text".into(),
                "error".into(),
                "visually_hidden_label".into(),
                "disabled".into(),
                "aria-describedby".into(),
                "aria_invalid".into(),
                "aria-label".into(),
                "aria-labelledby".into(),
                "on_change".into(),
                "on_blur".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "AttrValue".into(),
                "Option<AttrValue>".into(),
                "AttrValue".into(),
                "bool".into(),
                "bool".into(),
                "Option<AttrValue>".into(),
                "Option<AttrValue>".into(),
                "Option<AttrValue>".into(),
                "bool".into(),
                "bool".into(),
                "Option<AttrValue>".into(),
                "Option<bool>".into(),
                "Option<AttrValue>".into(),
                "Option<AttrValue>".into(),
                "Option<Callback<bool>>".into(),
                "Option<Callback<FocusEvent>>".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "DOM/accessibility ID for the underlying checkbox input.".into(),
                "Submitted form field name. Defaults to id.".into(),
                "Text label displayed next to the switch.".into(),
                "Initial on/off state (acts like a default value, can be used in controlled mode)."
                    .into(),
                "Whether this field is required for form submission.".into(),
                "Backward-compatible helper text alias.".into(),
                "Preferred helper text shown below the control.".into(),
                "External error message shown below the switch.".into(),
                "Hides the label visually while preserving it for screen readers.".into(),
                "Disables the switch when true.".into(),
                "Additional aria-describedby IDs.".into(),
                "Overrides computed aria-invalid state.".into(),
                "Accessible label for the switch; falls back to the visible `label` text.".into(),
                "ID of an element that labels the switch; defaults to `${id}-label`.".into(),
                "Called when the switch value changes, receiving the new `bool` state.".into(),
                "Called when the switch loses focus.".into(),
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
            name="notifications_enabled"
            label="Enable notifications"
            helper_text="Receive important updates by email."
            checked={*state}
            on_change={Some(on_change)}
        />
    }
}
