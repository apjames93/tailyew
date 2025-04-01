// src/atoms/x_icon.rs

use yew::prelude::*;

/// Properties for the `XIcon` component.
#[derive(Properties, PartialEq, Clone)]
pub struct XIconProps {
    #[prop_or(24)]
    pub size: u32,
    #[prop_or_default]
    pub color: Option<String>,
}

/// X Icon component
#[function_component(XIcon)]
pub fn x_icon(props: &XIconProps) -> Html {
    let XIconProps { size, color } = props.clone();

    // Set the SVG `fill` and `stroke` to the provided color or default to "currentColor"
    let fill_color = color.unwrap_or("currentColor".to_string());

    html! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width={size.to_string()}
            height={size.to_string()}
            viewBox="0 0 24 24"
            fill="none"
            stroke={fill_color}
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            class="x-icon"
        >
            <line x1="18" y1="6" x2="6" y2="18" />
            <line x1="6" y1="6" x2="18" y2="18" />
        </svg>
    }
}
