use crate::templates::demos::DemoComponent;
use tailyew::form::PhoneInput;
use tailyew::organisms::table::Column;
use yew::prelude::*;

const USAGE_CODE: &str = r#"
html! {
    <PhoneInput
        id="phone"
        name="contact_phone"
        label="Phone Number"
        placeholder="123-456-7890"
        default_value=""
        helper_text="Use the format 123-456-7890."
        required={true}
        pattern={Some(r"^\d{3}-\d{3}-\d{4}$".to_string())}
    />
}
"#;

#[component(PhoneInputDemoSection)]
pub fn phone_input_demo_section() -> Html {
    let example = html! {
        <PhoneInput
            id="phone"
            name="contact_phone"
            label="Phone Number"
            placeholder="123-456-7890"
            default_value=""
            helper_text="Use the format 123-456-7890."
            required={true}
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
                "name".into(),
                "default_value".into(),
                "value".into(),
                "pattern".into(),
                "helper_text".into(),
                "error".into(),
                "visually_hidden_label".into(),
                "aria_invalid".into(),
                "aria_describedby".into(),
                "required".into(),
                "disabled".into(),
                "class".into(),
                "on_change".into(),
                "on_blur".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "AttrValue".into(),
                "AttrValue".into(),
                "AttrValue".into(),
                "Option<AttrValue>".into(),
                "AttrValue".into(),
                "Option<AttrValue>".into(),
                "Option<String>".into(),
                "Option<AttrValue>".into(),
                "Option<AttrValue>".into(),
                "bool".into(),
                "Option<bool>".into(),
                "Option<AttrValue>".into(),
                "bool".into(),
                "bool".into(),
                "Classes".into(),
                "Option<Callback<String>>".into(),
                "Option<Callback<FocusEvent>>".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "Placeholder text for the phone input.".into(),
                "Label displayed above the input.".into(),
                "DOM/accessibility ID.".into(),
                "Submitted form field name. Defaults to id.".into(),
                "Initial value of the input.".into(),
                "Controlled phone value for parent-owned state.".into(),
                "Optional regex pattern (defaults to xxx-xxx-xxxx).".into(),
                "Optional helper copy below the input.".into(),
                "External error message shown below the input.".into(),
                "Hides the label visually while preserving it for screen readers.".into(),
                "Overrides computed aria-invalid state.".into(),
                "Additional aria-describedby IDs.".into(),
                "Marks the phone input as required.".into(),
                "Disables the phone input.".into(),
                "Optional Tailwind classes for styling.".into(),
                "Callback fired when the phone value changes.".into(),
                "Called when the input loses focus.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            github_demo_path="form/phone_input_demo_section.rs"
            github_source_path="form/phone_input.rs"
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
