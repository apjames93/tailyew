// src/atoms/x_icon.rs

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
    pub label: Option<String>,

    #[prop_or(false)]
    pub decorative: bool,

    #[prop_or_default]
    pub color: Option<String>,
}

/// X Icon component
#[component(XIcon)]
pub fn x_icon(props: &XIconProps) -> Html {
    let stroke_color = props
        .color
        .clone()
        .unwrap_or_else(|| "currentColor".to_string());

    let label = if props.label.is_none() && !props.decorative {
        Some("Close".to_string())
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
