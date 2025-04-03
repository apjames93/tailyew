use crate::templates::demos::DemoComponent;
use tailyew::form::{Input, InputType};
use tailyew::organisms::table::Column;
use yew::prelude::*;

const USAGE_CODE: &str = r#"
let value = use_state(|| "".to_string());
let on_change = {
    let value = value.clone();
    Callback::from(move |val: String| value.set(val))
};

html! {
    <Input
        id="username"
        label="Username"
        placeholder="Enter your username"
        input_type={InputType::Text}
        default_value=""
        required=true
        on_change={Some(on_change)}
    />
}
"#;

#[function_component(InputDemoSection)]
pub fn input_demo_section() -> Html {
    let example = html! { <InputUsage /> };

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "placeholder".into(),
                "label".into(),
                "id".into(),
                "input_type".into(),
                "default_value".into(),
                "min".into(),
                "max".into(),
                "pattern".into(),
                "required".into(),
                "class".into(),
                "on_change".into(),
                "disabled".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "String".into(),
                "String".into(),
                "String".into(),
                "InputType".into(),
                "String".into(),
                "Option<String>".into(),
                "Option<String>".into(),
                "Option<String>".into(),
                "bool".into(),
                "Classes".into(),
                "Option<Callback<String>>".into(),
                "bool".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "Placeholder text shown inside the input.".into(),
                "Label shown above the input field.".into(),
                "ID and name for the input element.".into(),
                "HTML input type (e.g., text, email, number).".into(),
                "Initial value shown in the input.".into(),
                "Optional minimum value (number/date).".into(),
                "Optional maximum value (number/date).".into(),
                "Regex pattern for validation.".into(),
                "Marks input as required.".into(),
                "Additional Tailwind classes.".into(),
                "Called when the input value changes.".into(),
                "Disables the input field if true.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            title="Input Component"
            description={Some(html! {
                <p>{"The `Input` component provides a reusable text-like input field with support for types like `text`, `email`, `date`, and `password`. Includes optional validation, styling, and state management."}</p>
            })}
            example={example}
            usage_code={USAGE_CODE}
            props_table={Some(props_table)}
        />
    }
}

#[function_component(InputUsage)]
fn input_usage() -> Html {
    let value = use_state(|| "".to_string());
    let on_change = {
        let value = value.clone();
        Callback::from(move |val: String| value.set(val))
    };

    html! {
        <Input
            id="username"
            label="Username"
            placeholder="Enter your username"
            input_type={InputType::Text}
            default_value=""
            required=true
            on_change={Some(on_change)}
        />
    }
}
