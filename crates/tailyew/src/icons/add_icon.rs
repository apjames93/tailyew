// src/atoms/add_icon.rs

use yew::prelude::*;

use crate::icons::icon_base::IconBase;

#[derive(Properties, PartialEq, Clone)]
pub struct AddIconProps {
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

#[component(AddIcon)]
pub fn add_icon(props: &AddIconProps) -> Html {
    let stroke_color = props
        .color
        .clone()
        .unwrap_or_else(|| "currentColor".to_string());

    let label = if props.label.is_none() && !props.decorative {
        Some("Add".to_string())
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
            <line x1="12" y1="5" x2="12" y2="19" stroke={stroke_color.clone()} stroke-linecap="round" stroke-linejoin="round" />
            <line x1="5" y1="12" x2="19" y2="12" stroke={stroke_color} stroke-linecap="round" stroke-linejoin="round" />
        </IconBase>
    }
}
