use yew::prelude::*;

/// Properties for the `CopyIcon` component.
#[derive(Properties, PartialEq, Clone)]
pub struct CopyIconProps {
    #[prop_or(24)]
    pub size: u32,
    #[prop_or_default]
    pub color: Option<String>,
}

/// Copy Icon component
#[function_component(CopyIcon)]
pub fn copy_icon(props: &CopyIconProps) -> Html {
    let CopyIconProps { size, color } = props.clone();

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
            aria-label="Copy to clipboard"
            xmlns="http://www.w3.org/2000/svg"
        >
            <path stroke-linecap="round" stroke-linejoin="round" d="M8 16h8m-8-4h8m-4-8h8v16H8V4z"/>
        </svg>
    }
}
