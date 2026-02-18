use crate::system::use_themed_classes;
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
    pub label: Option<AttrValue>,

    #[prop_or(false)]
    pub decorative: bool,

    #[prop_or_default]
    pub color: Option<AttrValue>,
}

#[component(PolylineIcon)]
pub fn polyline_icon(props: &PolylineProps) -> Html {
    let icon_class = use_themed_classes(
        "PolylineIcon",
        "root",
        Classes::default(),
        props.class.clone(),
    );
    let stroke_color = props
        .color
        .clone()
        .unwrap_or_else(|| AttrValue::from("currentColor"));

    let label = if props.label.is_none() && !props.decorative {
        Some(AttrValue::from("Molecules"))
    } else {
        props.label.clone()
    };

    html! {
        <IconBase
            class={icon_class}
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
