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
                "id",
                "name",
                "label",
                "placeholder",
                "default_value",
                "value",
                "input_type",
                "size",
                "min",
                "max",
                "pattern",
                "error_title",
                "required",
                "class",
                "container_class",
                "marginless",
                "label_class",
                "visually_hidden_label",
                "helper_text",
                "error",
                "aria_invalid",
                "on_change",
                "on_focus",
                "on_blur",
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
                "Option<AttrValue>",
                "AttrValue",
                "AttrValue",
                "AttrValue",
                "Option<AttrValue>",
                "InputType",
                "InputSize",
                "Option<AttrValue>",
                "Option<AttrValue>",
                "Option<AttrValue>",
                "Option<AttrValue>",
                "bool",
                "Classes",
                "Classes",
                "bool",
                "Classes",
                "bool",
                "Option<AttrValue>",
                "Option<AttrValue>",
                "Option<bool>",
                "Option<Callback<String>>",
                "Option<Callback<FocusEvent>>",
                "Option<Callback<FocusEvent>>",
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
                "DOM/accessibility ID.",
                "Submitted form field name. Defaults to id.",
                "Label shown above the input field.",
                "Placeholder text shown inside the input.",
                "Initial uncontrolled value.",
                "Controlled value for parent-owned input state.",
                "HTML input type (e.g., text, email, number).",
                "Visual density for the input control.",
                "Optional minimum value (number/date).",
                "Optional maximum value (number/date).",
                "Rust regex used for validation.",
                "Tooltip shown on invalid input.",
                "Marks input as required.",
                "Additional Tailwind classes.",
                "Additional Tailwind classes for the wrapper.",
                "Removes the default bottom margin.",
                "Additional Tailwind classes for the label.",
                "Hides the label visually while preserving it for screen readers.",
                "Optional helper copy below the input.",
                "External error message shown below the input.",
                "Overrides computed aria-invalid state.",
                "Callback on input value change.",
                "Callback when the input receives focus.",
                "Callback when the input loses focus.",
                "Disables the input field.",
                "Autocomplete hint for browsers.",
                "ARIA label (e.g., for screen readers).",
                "ARIA labelledby reference ID.",
                "ARIA describedby reference ID.",
                "ARIA expanded state for composite controls.",
                "ARIA controls relationship target.",
                "ARIA popup type/state for composite controls.",
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
