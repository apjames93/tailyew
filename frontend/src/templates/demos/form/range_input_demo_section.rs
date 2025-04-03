use crate::templates::demos::DemoComponent;
use tailyew::form::RangeInput;
use tailyew::organisms::table::Column;
use yew::prelude::*;

const USAGE_CODE: &str = r#"
html! {
    <RangeInput
        id="volume"
        label="Volume"
        default_value={"50".to_string()}
        min={"0".to_string()}
        max={"100".to_string()}
        step={"5".to_string()}
        on_change={Some(Callback::from(|val| web_sys::console::log_1(&format!("Volume: {}", val).into())))}
    />
}
"#;

#[function_component(RangeInputDemoSection)]
pub fn range_input_demo_section() -> Html {
    let on_change = Callback::from(|val: String| {
        web_sys::console::log_1(&format!("Range value changed: {}", val).into());
    });

    let example = html! {
        <RangeInput
            id="volume"
            label="Volume"
            default_value={"50".to_string()}
            min={"0".to_string()}
            max={"100".to_string()}
            step={"5".to_string()}
            on_change={Some(on_change)}
        />
    };

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "id".into(),
                "label".into(),
                "default_value".into(),
                "min".into(),
                "max".into(),
                "step".into(),
                "class".into(),
                "on_change".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "String".into(),
                "String".into(),
                "String".into(),
                "String".into(),
                "String".into(),
                "String".into(),
                "Classes".into(),
                "Option<Callback<String>>".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "The unique ID and name for the input.".into(),
                "Label displayed above the slider.".into(),
                "The starting value for the range.".into(),
                "Minimum value allowed.".into(),
                "Maximum value allowed.".into(),
                "Value increment between steps.".into(),
                "Optional Tailwind classes for styling.".into(),
                "Callback fired when the value changes.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
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
