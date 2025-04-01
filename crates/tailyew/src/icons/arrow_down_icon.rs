// src/atoms/arrow_down_icon.rs

use yew::prelude::*;

/// Properties for the `ArrowDownIcon` component.
#[derive(Properties, PartialEq, Clone)]
pub struct ArrowDownIconProps {
    #[prop_or(24)]
    pub size: u32,
    #[prop_or_default]
    pub color: Option<String>,
}

/// Arrow Down Icon component
#[function_component(ArrowDownIcon)]
pub fn arrow_down_icon(props: &ArrowDownIconProps) -> Html {
    let ArrowDownIconProps { size, color } = props.clone();

    // Set the SVG `fill` and `stroke` to the provided color or default to "currentColor"
    let fill_color = color.unwrap_or("currentColor".to_string());

    html! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width={size.to_string()}
            height={size.to_string()}
            viewBox="0 0 24 24"
            fill={fill_color.clone()}
            stroke={fill_color}
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            class="arrow-down-icon"
        >
            <path d="M12 5v14M19 12l-7 7-7-7" />
        </svg>
    }
}
