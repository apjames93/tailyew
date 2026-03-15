// src/atoms/delete_icon.rs

use crate::system::use_themed_classes;
use yew::prelude::*;

use crate::icons::icon_base::IconBase;

#[derive(Properties, PartialEq, Clone)]
pub struct DeleteIconProps {
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

#[component(DeleteIcon)]
pub fn delete_icon(props: &DeleteIconProps) -> Html {
    let icon_class = use_themed_classes(
        "DeleteIcon",
        "root",
        Classes::default(),
        props.class.clone(),
    );
    let stroke_color = props
        .color
        .clone()
        .unwrap_or_else(|| AttrValue::from("currentColor"));

    // default label if not decorative
    let label = if props.label.is_none() && !props.decorative {
        Some(AttrValue::from("Delete"))
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
            <path d="M3 6h18" stroke={stroke_color.clone()} stroke-linecap="round" stroke-linejoin="round" />
            <path d="M8 6v-2a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" stroke={stroke_color.clone()} stroke-linecap="round" stroke-linejoin="round" />
            <rect x="5" y="6" width="14" height="14" rx="2" ry="2" stroke={stroke_color.clone()} />
            <line x1="10" y1="11" x2="10" y2="17" stroke={stroke_color.clone()} stroke-linecap="round" stroke-linejoin="round" />
            <line x1="14" y1="11" x2="14" y2="17" stroke={stroke_color} stroke-linecap="round" stroke-linejoin="round" />
        </IconBase>
    }
}
