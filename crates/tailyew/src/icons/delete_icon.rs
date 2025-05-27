// src/atoms/delete_icon.rs

use yew::prelude::*;

/// Properties for the `DeleteIcon` component.
#[derive(Properties, PartialEq, Clone)]
pub struct DeleteIconProps {
    /// Size (both width and height) in pixels.
    #[prop_or(24)]
    pub size: u32,
    /// Stroke color (defaults to currentColor).
    #[prop_or_default]
    pub color: Option<String>,
}

#[function_component(DeleteIcon)]
pub fn delete_icon(props: &DeleteIconProps) -> Html {
    let DeleteIconProps { size, color } = props.clone();
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
            class="delete-icon"
        >
            // Trash can outline
            <path d="M3 6h18" />
            <path d="M8 6v-2a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" />
            <rect x="5" y="6" width="14" height="14" rx="2" ry="2" />
            // Inner “bin” lines
            <line x1="10" y1="11" x2="10" y2="17" />
            <line x1="14" y1="11" x2="14" y2="17" />
        </svg>
    }
}
