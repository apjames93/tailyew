use yew::prelude::*;

use crate::icons::icon_base::IconBase;

#[derive(Properties, PartialEq, Clone)]
pub struct PolylineProps {
    #[prop_or_default]
    pub class: Classes,

    #[prop_or(24)]
    pub size: u32,

    #[prop_or(1.5)]
    pub stroke_width: f32,

    #[prop_or_default]
    pub label: Option<String>,

    #[prop_or(false)]
    pub decorative: bool,

    #[prop_or_default]
    pub color: Option<String>,
}

#[component(PolylineIcon)]
pub fn polyline_icon(props: &PolylineProps) -> Html {
    let stroke_color = props
        .color
        .clone()
        .unwrap_or_else(|| "currentColor".to_string());

    let label = if props.label.is_none() && !props.decorative {
        Some("Molecules".to_string())
    } else {
        props.label.clone()
    };

    html! {
        <IconBase
            class={props.class.clone()}
            size={props.size}
            stroke_width={props.stroke_width}
            label={label}
            decorative={props.decorative}
        >
            <circle cx="6" cy="12" r="2.5" stroke={stroke_color.clone()} />
            <circle cx="18" cy="6" r="2.5" stroke={stroke_color.clone()} />
            <circle cx="18" cy="18" r="2.5" stroke={stroke_color.clone()} />
            <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke={stroke_color}
                d="M7.7 13.5l6.6 3.2M7.7 10.5l6.6-3.2"
            />
        </IconBase>
    }
}
