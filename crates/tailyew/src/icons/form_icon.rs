use yew::prelude::*;

/// Properties for the `FormIcon` component.
#[derive(Properties, PartialEq, Clone)]
pub struct FormIconProps {
    #[prop_or(24)]
    pub size: u32,
    #[prop_or_default]
    pub color: Option<String>,
}

/// Form Icon component for TailYew
#[function_component(FormIcon)]
pub fn form_icon(props: &FormIconProps) -> Html {
    let FormIconProps { size, color } = props.clone();

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
            aria-label="Form"
            xmlns="http://www.w3.org/2000/svg"
        >
            <rect x="4" y="4" width="16" height="16" rx="3" />
            <path stroke-linecap="round" stroke-linejoin="round" d="M8 11h8M8 15h6M9 8h.01" />
        </svg>
    }
}
