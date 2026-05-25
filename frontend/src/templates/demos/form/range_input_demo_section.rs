use crate::templates::demos::DemoComponent;
use tailyew::form::RangeInput;
use tailyew::organisms::table::Column;
use yew::prelude::*;

const USAGE_CODE: &str = r#"
html! {
    <RangeInput
        id="volume"
        name="volume_level"
        label="Volume"
        default_value={"50".to_string()}
        min={"0".to_string()}
        max={"100".to_string()}
        step={"5".to_string()}
        helper_text="Drag to set the submitted volume level."
        on_change={Some(Callback::from(|val| web_sys::console::log_1(&format!("Volume: {}", val).into())))}
    />
}
"#;

#[component(RangeInputDemoSection)]
pub fn range_input_demo_section() -> Html {
    let on_change = Callback::from(|val: String| {
        web_sys::console::log_1(&format!("Range value changed: {}", val).into());
    });

    let example = html! {
        <RangeInput
            id="volume"
            name="volume_level"
            label="Volume"
            default_value={"50".to_string()}
            min={"0".to_string()}
            max={"100".to_string()}
            step={"5".to_string()}
            helper_text="Drag to set the submitted volume level."
            on_change={Some(on_change)}
        />
    };

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "id".into(),
                "name".into(),
                "label".into(),
                "default_value".into(),
                "value".into(),
                "min".into(),
                "max".into(),
                "step".into(),
                "helper_text".into(),
                "error".into(),
                "aria_invalid".into(),
                "aria_describedby".into(),
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
                "Option<AttrValue>".into(),
                "AttrValue".into(),
                "AttrValue".into(),
                "Option<AttrValue>".into(),
                "String".into(),
                "String".into(),
                "String".into(),
                "Option<AttrValue>".into(),
                "Option<AttrValue>".into(),
                "Option<bool>".into(),
                "Option<AttrValue>".into(),
                "bool".into(),
                "Classes".into(),
                "Option<Callback<String>>".into(),
                "Option<Callback<FocusEvent>>".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "DOM/accessibility ID.".into(),
                "Submitted form field name. Defaults to id.".into(),
                "Label displayed above the slider.".into(),
                "The starting value for the range.".into(),
                "Controlled range value for parent-owned state.".into(),
                "Minimum value allowed.".into(),
                "Maximum value allowed.".into(),
                "Value increment between steps.".into(),
                "Optional helper copy below the slider.".into(),
                "External error message shown below the slider.".into(),
                "Overrides computed aria-invalid state.".into(),
                "Additional aria-describedby IDs.".into(),
                "Disables the slider.".into(),
                "Optional Tailwind classes for styling.".into(),
                "Callback fired when the value changes.".into(),
                "Called when the slider loses focus.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            github_demo_path="form/range_input_demo_section.rs"
            github_source_path="form/range_input.rs"
            title="RangeInput Component"
            description={Some(html! {
                <p>{"The `RangeInput` component renders a stylized slider with a live-updating value display. Supports min, max, step, and custom change handling."}</p>
            })}
            example={example}
            usage_code={USAGE_CODE}
            props_table={Some(props_table)}
        />
    }
}
