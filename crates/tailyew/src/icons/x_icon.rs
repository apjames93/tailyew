// src/atoms/x_icon.rs

use crate::system::use_themed_classes;
use yew::prelude::*;

use crate::icons::icon_base::IconBase;

#[derive(Properties, PartialEq, Clone)]
pub struct XIconProps {
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

/// X Icon component
#[component(XIcon)]
pub fn x_icon(props: &XIconProps) -> Html {
    let icon_class = use_themed_classes("XIcon", "root", Classes::default(), props.class.clone());
    let stroke_color = props
        .color
        .clone()
        .unwrap_or_else(|| AttrValue::from("currentColor"));

    let label = if props.label.is_none() && !props.decorative {
        Some(AttrValue::from("Close"))
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
            <line
                x1="18"
                y1="6"
                x2="6"
                y2="18"
                stroke={stroke_color.clone()}
                stroke-linecap="round"
                stroke-linejoin="round"
            />
            <line
                x1="6"
                y1="6"
                x2="18"
                y2="18"
                stroke={stroke_color}
                stroke-linecap="round"
                stroke-linejoin="round"
            />
        </IconBase>
    }
}
