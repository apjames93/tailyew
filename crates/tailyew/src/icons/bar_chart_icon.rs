use yew::prelude::*;

/// Properties for the BarChartIcon component.
#[derive(Properties, PartialEq, Clone)]
pub struct BarChartIconProps {
    #[prop_or(24)]
    pub size: u32,
    #[prop_or_default]
    pub color: Option<String>,
}

/// Bar chart icon for TailYew
#[function_component(BarChartIcon)]
pub fn bar_chart_icon(props: &BarChartIconProps) -> Html {
    let BarChartIconProps { size, color } = props.clone();

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
            aria-label="Bar chart"
            xmlns="http://www.w3.org/2000/svg"
        >
            <rect x="4" y="12" width="3" height="8" rx="1" />
            <rect x="10.5" y="8" width="3" height="12" rx="1" />
            <rect x="17" y="4" width="3" height="16" rx="1" />
        </svg>
    }
}
