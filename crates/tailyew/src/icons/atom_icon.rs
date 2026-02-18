use crate::system::use_themed_classes;
use yew::prelude::*;

use crate::icons::icon_base::IconBase;
#[derive(Properties, PartialEq, Clone)]
pub struct AtomIconProps {
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

#[component(AtomIcon)]
pub fn atom_icon(props: &AtomIconProps) -> Html {
    let icon_class =
        use_themed_classes("AtomIcon", "root", Classes::default(), props.class.clone());
    let stroke_color = props
        .color
        .clone()
        .unwrap_or_else(|| AttrValue::from("currentColor"));

    let label = if props.label.is_none() && !props.decorative {
        Some(AttrValue::from("Atoms"))
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
            // center nucleus
            <circle cx="12" cy="12" r="3.2" stroke={stroke_color.clone()} />

            // horizontal orbit
            <ellipse
                cx="12"
                cy="12"
                rx="8"
                ry="3.5"
                stroke={stroke_color.clone()}
            />

            // tilted orbit
            <ellipse
                cx="12"
                cy="12"
                rx="3.5"
                ry="8"
                transform="rotate(45 12 12)"
                stroke={stroke_color}
            />
        </IconBase>
    }
}
