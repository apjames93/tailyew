use crate::templates::demos::DemoComponent;
use tailyew::form::{Input, InputType};
use tailyew::organisms::table::Column;
use yew::prelude::*;

const USAGE_CODE: &str = r#"
let name = use_state(|| "".to_string());
let username = use_state(|| "".to_string());

let on_name_change = {
    let name = name.clone();
    Callback::from(move |val: String| name.set(val))
};

let on_username_change = {
    let username = username.clone();
    Callback::from(move |val: String| username.set(val))
};

html! {
    <>
        <Input
            id="name"
            label="Full Name"
            placeholder="Jane Doe"
            input_type={InputType::Text}
            default_value=""
            required=true
            on_change={Some(on_name_change)}
        />
        <Input
            id="username"
            label="Username"
            placeholder="e.g. buddy_guy"
            input_type={InputType::Text}
            pattern={Some("^[a-z0-9_-]{3,16}$")}
            error_title={Some("Use 3–16 lowercase letters, numbers, underscores, or dashes.")}
            default_value=""
            required=true
            on_change={Some(on_username_change)}
        />
    </>
}
"#;

#[component(InputDemoSection)]
pub fn input_demo_section() -> Html {
    let example = html! { <InputUsage /> };

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "placeholder",
                "label",
                "id",
                "input_type",
                "default_value",
                "min",
                "max",
                "pattern",
                "error_title",
                "required",
                "class",
                "on_change",
                "disabled",
                "autocomplete",
                "aria_label",
                "aria_labelledby",
                "aria_describedby",
                "aria_expanded",
                "aria_controls",
                "aria_haspopup",
                "node_ref",
                "validate",
            ]
            .into_iter()
            .map(Html::from)
            .collect(),
        },
        Column {
            header: "Type".into(),
            values: vec![
                "AttrValue",
                "AttrValue",
                "AttrValue",
                "InputType",
                "AttrValue",
                "Option<AttrValue>",
                "Option<AttrValue>",
                "Option<AttrValue>",
                "Option<AttrValue>",
                "bool",
                "Classes",
                "Option<Callback<String>>",
                "bool",
                "Option<AttrValue>",
                "Option<AttrValue>",
                "Option<AttrValue>",
                "Option<AttrValue>",
                "Option<AttrValue>",
                "Option<AttrValue>",
                "Option<AttrValue>",
                "NodeRef",
                "Option<Callback<String, Option<String>>>",
            ]
            .into_iter()
            .map(Html::from)
            .collect(),
        },
        Column {
            header: "Description".into(),
            values: vec![
                "Placeholder text shown inside the input.",
                "Label shown above the input field.",
                "ID and name for the input element.",
                "HTML input type (e.g., text, email, number).",
                "Initial value shown in the input.",
                "Optional minimum value (number/date).",
                "Optional maximum value (number/date).",
                "Rust regex used for validation.",
                "Tooltip shown on invalid input.",
                "Marks input as required.",
                "Additional Tailwind classes.",
                "Callback on input value change.",
                "Disables the input field.",
                "Autocomplete hint for browsers.",
                "ARIA label (e.g., for screen readers).",
                "ARIA labelledby reference ID.",
                "ARIA describedby reference ID.",
                "Node reference for DOM access.",
                "Callback for custom validation.",
            ]
            .into_iter()
            .map(Html::from)
            .collect(),
        },
    ];

    html! {
        <DemoComponent
            github_demo_path="form/input_demo_section.rs"
            github_source_path="form/input.rs"
            title="Input Component"
            description={Some(html! {
                <p>{"The `Input` component provides accessible, validated text inputs with optional Rust regex validation via the `pattern` prop. Error messages are shown via `title`, and the component integrates cleanly into native HTML5 form validation."}</p>
            })}
            example={example}
            usage_code={USAGE_CODE}
            props_table={Some(props_table)}
        />
    }
}

#[component(InputUsage)]
fn input_usage() -> Html {
    let name = use_state(|| "".to_string());
    let username = use_state(|| "".to_string());

    let on_name_change = {
        let name = name.clone();
        Callback::from(move |val: String| name.set(val))
    };

    let on_username_change = {
        let username = username.clone();
        Callback::from(move |val: String| username.set(val))
    };

    html! {
        <>
            <Input
                id="name"
                label="Full Name"
                placeholder="Jane Doe"
                input_type={InputType::Text}
                default_value=""
                required=true
                on_change={Some(on_name_change)}
            />
            <Input
                id="username"
                label="Username"
                placeholder="e.g. buddy_guy"
                input_type={InputType::Text}
                pattern={Some("^[a-z0-9_-]{3,16}$")}
                error_title={Some("Use 3–16 lowercase letters, numbers, underscores, or dashes.")}
                default_value=""
                required=true
                on_change={Some(on_username_change)}
            />
        </>
    }
}
