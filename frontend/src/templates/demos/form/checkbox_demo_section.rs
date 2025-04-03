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
        label="Subscribe to newsletter"
        description="We'll never send spam."
        checked={*state}
        on_change={Some(on_change)}
    />
}
"#;

#[function_component(CheckboxDemoSection)]
pub fn checkbox_demo_section() -> Html {
    let example = html! { <CheckboxUsage /> };

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "id".into(),
                "label".into(),
                "checked".into(),
                "description".into(),
                "disabled".into(),
                "on_change".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "String".into(),
                "String".into(),
                "bool".into(),
                "Option<String>".into(),
                "bool".into(),
                "Option<Callback<bool>>".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "The ID and name for the checkbox input.".into(),
                "Visible label text.".into(),
                "Whether the checkbox is checked.".into(),
                "Optional helper text below the label.".into(),
                "Disables the checkbox if true.".into(),
                "Called when the checkbox value changes.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
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

#[function_component(CheckboxUsage)]
fn checkbox_usage() -> Html {
    let state = use_state(|| false);
    let on_change = {
        let state = state.clone();
        Callback::from(move |val: bool| state.set(val))
    };

    html! {
        <Checkbox
            id="subscribe"
            label="Subscribe to newsletter"
            description="We'll never send spam."
            checked={*state}
            on_change={Some(on_change)}
        />
    }
}
