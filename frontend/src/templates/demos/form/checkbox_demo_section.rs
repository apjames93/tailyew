// frontend/src/templates/demos/checkbox_demo/checkbox_demo_section.rs

use crate::templates::demos::DemoComponent;
use tailyew::form::Checkbox;
use tailyew::organisms::table::Column;
use yew::prelude::*;

const USAGE_CODE: &str = r#"
let state = use_state(|| false);
let on_change = {
    let state = state.clone();
    Callback::from(move |val: bool| state.set(val))
};

html! {
    <Checkbox
        id="subscribe"
        name="newsletter_subscribed"
        label="Subscribe to newsletter"
        helper_text="We'll never send spam."
        checked={*state}
        on_change={Some(on_change)}
    />
}
"#;

#[component(CheckboxDemoSection)]
pub fn checkbox_demo_section() -> Html {
    let example = html! { <CheckboxUsage /> };

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "id",
                "name",
                "label",
                "checked",
                "required",
                "description",
                "helper_text",
                "error",
                "visually_hidden_label",
                "aria_invalid",
                "aria_describedby",
                "aria_label",
                "disabled",
                "on_change",
                "on_blur",
            ]
            .into_iter()
            .map(Html::from)
            .collect(),
        },
        Column {
            header: "Type".into(),
            values: vec![
                "AttrValue",
                "Option<AttrValue>",
                "AttrValue",
                "bool",
                "bool",
                "Option<AttrValue>",
                "Option<AttrValue>",
                "Option<AttrValue>",
                "bool",
                "Option<bool>",
                "Option<AttrValue>",
                "Option<AttrValue>",
                "bool",
                "Option<Callback<bool>>",
                "Option<Callback<FocusEvent>>",
            ]
            .into_iter()
            .map(Html::from)
            .collect(),
        },
        Column {
            header: "Description".into(),
            values: vec![
                "DOM/accessibility ID.",
                "Submitted form field name. Defaults to id.",
                "Visible label text.",
                "Whether the checkbox is checked.",
                "Marks the checkbox as required.",
                "Backward-compatible helper text alias.",
                "Preferred helper text below the checkbox.",
                "External error message shown below the checkbox.",
                "Hides the label visually while preserving it for screen readers.",
                "Overrides computed aria-invalid state.",
                "Additional aria-describedby IDs.",
                "Accessible label for compact or hidden-label contexts.",
                "Disables the checkbox if true.",
                "Called when the checkbox value changes.",
                "Called when the checkbox loses focus.",
            ]
            .into_iter()
            .map(Html::from)
            .collect(),
        },
    ];

    html! {
        <DemoComponent
            github_demo_path="form/checkbox_demo_section.rs"
            github_source_path="form/checkbox.rs"
            title="Checkbox Component"
            description={Some(html! {
                <p>{"The `Checkbox` component is a stylized toggle input with optional label and description. Supports controlled state and `on_change` callbacks."}</p>
            })}
            example={example}
            usage_code={USAGE_CODE}
            props_table={Some(props_table)}
        />
    }
}

#[component(CheckboxUsage)]
fn checkbox_usage() -> Html {
    let state = use_state(|| false);
    let on_change = {
        let state = state.clone();
        Callback::from(move |val: bool| state.set(val))
    };

    html! {
        <Checkbox
            id="subscribe"
            name="newsletter_subscribed"
            label="Subscribe to newsletter"
            helper_text="We'll never send spam."
            checked={*state}
            on_change={Some(on_change)}
        />
    }
}
