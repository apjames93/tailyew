use yew::prelude::*;

/// Properties for the `Polyline` component.
#[derive(Properties, PartialEq, Clone)]
pub struct PolylineProps {
    #[prop_or(24)]
    pub size: u32,
    #[prop_or_default]
    pub color: Option<String>,
}

/// Molecules Icon component for TailYew
#[function_component(PolylineIcon)]
pub fn polyline_icon(props: &PolylineProps) -> Html {
    let PolylineProps { size, color } = props.clone();

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
            aria-label="Molecules"
            xmlns="http://www.w3.org/2000/svg"
        >
            <circle cx="6" cy="12" r="2.5" />
            <circle cx="18" cy="6" r="2.5" />
            <circle cx="18" cy="18" r="2.5" />
            <path stroke-linecap="round" stroke-linejoin="round" d="M7.7 13.5l6.6 3.2M7.7 10.5l6.6-3.2" />
        </svg>
    }
}
