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
        name="brand_color_hex"
        label="Brand Color"
        value={(*color).clone()}
        helper_text="Submitted as the selected hex color."
        on_change={Some(on_change)}
    />
}
"#;

#[component(ColorInputDemoSection)]
pub fn color_input_demo_section() -> Html {
    let example = html! { <ColorInputUsage /> };

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "id".into(),
                "name".into(),
                "label".into(),
                "value".into(),
                "helper_text".into(),
                "error".into(),
                "visually_hidden_label".into(),
                "aria_invalid".into(),
                "aria_describedby".into(),
                "required".into(),
                "disabled".into(),
                "on_change".into(),
                "on_blur".into(),
                "class".into(),
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
                "Option<AttrValue>".into(),
                "bool".into(),
                "Option<bool>".into(),
                "Option<AttrValue>".into(),
                "bool".into(),
                "bool".into(),
                "Option<Callback<String>>".into(),
                "Option<Callback<FocusEvent>>".into(),
                "Classes".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "DOM/accessibility ID.".into(),
                "Submitted form field name. Defaults to id.".into(),
                "Label text displayed above the color input.".into(),
                "Initial hex value of the color (e.g., \"#000000\").".into(),
                "Optional helper copy below the color input.".into(),
                "External error message below the input.".into(),
                "Hides the label visually while preserving it for screen readers.".into(),
                "Overrides computed aria-invalid state.".into(),
                "Additional aria-describedby IDs.".into(),
                "Marks the color input as required.".into(),
                "Disables the color input.".into(),
                "Fires when the color changes, passing the new hex string.".into(),
                "Called when the color input loses focus.".into(),
                "Optional Tailwind classes for the input element.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            github_demo_path="form/color_input_demo_section.rs"
            github_source_path="form/color_input.rs"
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

#[component(ColorInputUsage)]
fn color_input_usage() -> Html {
    let color = use_state(|| "#3b82f6".to_string());
    let on_change = {
        let color = color.clone();
        Callback::from(move |val: String| color.set(val))
    };

    html! {
        <ColorInput
            id="brand_color"
            name="brand_color_hex"
            label="Brand Color"
            value={(*color).clone()}
            helper_text="Submitted as the selected hex color."
            on_change={Some(on_change)}
        />
    }
}
