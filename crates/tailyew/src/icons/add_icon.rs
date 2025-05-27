// src/atoms/add_icon.rs

use yew::prelude::*;

/// Properties for the `AddIcon` component.
#[derive(Properties, PartialEq, Clone)]
pub struct AddIconProps {
    /// Size (both width and height) in pixels.
    #[prop_or(24)]
    pub size: u32,
    /// Stroke color (defaults to currentColor).
    #[prop_or_default]
    pub color: Option<String>,
}

#[function_component(AddIcon)]
pub fn add_icon(props: &AddIconProps) -> Html {
    let AddIconProps { size, color } = props.clone();
    let stroke_color = color.unwrap_or_else(|| "currentColor".into());

    html! {
        <svg
            width={size.to_string()}
            height={size.to_string()}
            viewBox="0 0 24 24"
            fill="none"
            stroke={stroke_color.clone()}
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            class="add-icon"
        >
            <line x1="12" y1="5" x2="12" y2="19" />
            <line x1="5" y1="12" x2="19" y2="12" />
        </svg>
    }
}
