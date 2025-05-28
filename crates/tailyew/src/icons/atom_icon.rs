use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct AtomIconProps {
    #[prop_or(24)]
    pub size: u32,
    #[prop_or_default]
    pub color: Option<String>,
}

#[function_component(AtomIcon)]
pub fn atom_icon(props: &AtomIconProps) -> Html {
    let AtomIconProps { size, color } = props.clone();

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
            aria-label="Atoms"
            xmlns="http://www.w3.org/2000/svg"
        >
            <circle cx="12" cy="12" r="3.2" />
            <ellipse cx="12" cy="12" rx="8" ry="3.5" />
            <ellipse cx="12" cy="12" rx="3.5" ry="8" transform="rotate(45 12 12)" />
        </svg>
    }
}
