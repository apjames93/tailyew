use crate::templates::demos::DemoComponent;
use tailyew::form::PhoneInput;
use tailyew::organisms::table::Column;
use yew::prelude::*;

const USAGE_CODE: &str = r#"
html! {
    <PhoneInput
        id="phone"
        label="Phone Number"
        placeholder="123-456-7890"
        default_value=""
        pattern={Some(r"^\d{3}-\d{3}-\d{4}$".to_string())}
    />
}
"#;

#[function_component(PhoneInputDemoSection)]
pub fn phone_input_demo_section() -> Html {
    let example = html! {
        <PhoneInput
            id="phone"
            label="Phone Number"
            placeholder="123-456-7890"
            default_value=""
            pattern={Some(r"^\d{3}-\d{3}-\d{4}$".to_string())}
        />
    };

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "placeholder".into(),
                "label".into(),
                "id".into(),
                "default_value".into(),
                "pattern".into(),
                "class".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "String".into(),
                "String".into(),
                "String".into(),
                "String".into(),
                "Option<String>".into(),
                "Classes".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "Placeholder text for the phone input.".into(),
                "Label displayed above the input.".into(),
                "Unique ID and name for the input.".into(),
                "Initial value of the input.".into(),
                "Optional regex pattern (defaults to xxx-xxx-xxxx).".into(),
                "Optional Tailwind classes for styling.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            title="PhoneInput Component"
            description={Some(html! {
                <p>{"The `PhoneInput` component renders a telephone input with built-in validation based on a regex pattern, and real-time feedback on invalid formats."}</p>
            })}
            example={example}
            usage_code={USAGE_CODE}
            props_table={Some(props_table)}
        />
    }
}
