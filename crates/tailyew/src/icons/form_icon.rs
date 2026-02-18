use crate::system::use_themed_classes;
use yew::prelude::*;

use crate::icons::icon_base::IconBase;

#[derive(Properties, PartialEq, Clone)]
pub struct FormIconProps {
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

/// Form Icon component for TailYew
#[component(FormIcon)]
pub fn form_icon(props: &FormIconProps) -> Html {
    let icon_class =
        use_themed_classes("FormIcon", "root", Classes::default(), props.class.clone());
    let stroke_color = props
        .color
        .clone()
        .unwrap_or_else(|| AttrValue::from("currentColor"));

    let label = if props.label.is_none() && !props.decorative {
        Some(AttrValue::from("Form"))
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
            <rect x="4" y="4" width="16" height="16" rx="3" stroke={stroke_color.clone()} />
            <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke={stroke_color}
                d="M8 11h8M8 15h6M9 8h.01"
            />
        </IconBase>
    }
}
