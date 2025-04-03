use crate::templates::demos::DemoComponent;
use tailyew::form::ColorInput;
use tailyew::organisms::table::Column;
use yew::prelude::*;

const USAGE_CODE: &str = r#"
let color = use_state(|| "blue".to_string());
let on_change = {
    let color = color.clone();
    Callback::from(move |val: String| color.set(val))
};

html! {
    <ColorInput
        id="brand_color"
        label="Brand Color"
        value={(*color).clone()}
        on_change={Some(on_change)}
    />
}
"#;

#[function_component(ColorInputDemoSection)]
pub fn color_input_demo_section() -> Html {
    let example = html! { <ColorInputUsage /> };

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "id".into(),
                "label".into(),
                "value".into(),
                "on_change".into(),
                "class".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "String".into(),
                "String".into(),
                "String".into(),
                "Option<Callback<String>>".into(),
                "Classes".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "ID and name of the input element.".into(),
                "Label text displayed above the color input.".into(),
                "Initial hex value of the color (e.g., \"#000000\").".into(),
                "Fires when the color changes, passing the new hex string.".into(),
                "Optional Tailwind classes for the input element.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            title="ColorInput Component"
            description={Some(html! {
                <p>{"The `ColorInput` component renders a color picker input with a preview and optional callback. Great for theme or brand color customization."}</p>
            })}
            example={example}
            usage_code={USAGE_CODE}
            props_table={Some(props_table)}
        />
    }
}

#[function_component(ColorInputUsage)]
fn color_input_usage() -> Html {
    let color = use_state(|| "#3b82f6".to_string());
    let on_change = {
        let color = color.clone();
        Callback::from(move |val: String| color.set(val))
    };

    html! {
        <ColorInput
            id="brand_color"
            label="Brand Color"
            value={(*color).clone()}
            on_change={Some(on_change)}
        />
    }
}
