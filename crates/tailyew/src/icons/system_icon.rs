use yew::prelude::*;

/// Properties for the `SystemIcon` component.
#[derive(Properties, PartialEq, Clone)]
pub struct SystemIconProps {
    #[prop_or(24)]
    pub size: u32,
    #[prop_or_default]
    pub color: Option<String>,
}

/// System Icon component for TailYew
#[function_component(SystemIcon)]
pub fn system_icon(props: &SystemIconProps) -> Html {
    let SystemIconProps { size, color } = props.clone();

    let stroke_color = color.unwrap_or_else(|| "currentColor".to_string());

    html! {
        <svg
            width={size.to_string()}
            height={size.to_string()}
            fill="none"
            stroke={stroke_color}
            stroke-width="2"
            viewBox="0 0 24 24"
            aria-hidden="true"
            role="img"
            aria-label="System"
            xmlns="http://www.w3.org/2000/svg"
        >
            <circle cx="12" cy="12" r="3.5" />
            <path stroke-linecap="round" stroke-linejoin="round"
                d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 1 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 1 1-4 0v-.09A1.65 1.65 0 0 0 7.6 19a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 1 1-2.83-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 1 1 0-4h.09A1.65 1.65 0 0 0 5 7.6a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 1 1 2.83-2.83l.06.06a1.65 1.65 0 0 0 1.82.33h.08a1.65 1.65 0 0 0 1-1.51V3a2 2 0 1 1 4 0v.09a1.65 1.65 0 0 0 1 1.51h.08a1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 1 1 2.83 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82v.08a1.65 1.65 0 0 0 1.51 1H21a2 2 0 1 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"
            />
        </svg>
    }
}
