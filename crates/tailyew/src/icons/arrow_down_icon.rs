// src/atoms/arrow_down_icon.rs

use crate::system::use_themed_classes;
use yew::prelude::*;

use crate::icons::icon_base::IconBase;

#[derive(Properties, PartialEq, Clone)]
pub struct ArrowDownIconProps {
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

#[component(ArrowDownIcon)]
pub fn arrow_down_icon(props: &ArrowDownIconProps) -> Html {
    let icon_class = use_themed_classes(
        "ArrowDownIcon",
        "root",
        Classes::default(),
        props.class.clone(),
    );
    let stroke_color = props
        .color
        .clone()
        .unwrap_or_else(|| AttrValue::from("currentColor"));

    let label = if props.label.is_none() && !props.decorative {
        Some(AttrValue::from("Arrow down"))
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
            <path
                d="M12 5v14M19 12l-7 7-7-7"
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke={stroke_color}
            />
        </IconBase>
    }
}
