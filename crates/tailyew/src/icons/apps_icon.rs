use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct AppsIconProps {
    #[prop_or(24)]
    pub size: u32,
    #[prop_or_default]
    pub color: Option<String>,
}

#[function_component(AppsIcon)]
pub fn apps_icon(props: &AppsIconProps) -> Html {
    let AppsIconProps { size, color } = props.clone();

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
            aria-label="Organisms"
            xmlns="http://www.w3.org/2000/svg"
        >
            <rect x="3" y="3" width="7" height="7" rx="2"/>
            <rect x="14" y="3" width="7" height="7" rx="2"/>
            <rect x="3" y="14" width="7" height="7" rx="2"/>
            <rect x="14" y="14" width="7" height="7" rx="2"/>
        </svg>
    }
}
