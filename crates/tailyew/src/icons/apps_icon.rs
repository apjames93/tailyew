use crate::system::use_themed_classes;
use yew::prelude::*;

use crate::icons::icon_base::IconBase;

#[derive(Properties, PartialEq, Clone)]
pub struct AppsIconProps {
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

#[component(AppsIcon)]
pub fn apps_icon(props: &AppsIconProps) -> Html {
    let icon_class =
        use_themed_classes("AppsIcon", "root", Classes::default(), props.class.clone());
    let fill_or_stroke = props
        .color
        .clone()
        .unwrap_or_else(|| AttrValue::from("currentColor"));

    let label = if props.label.is_none() && !props.decorative {
        Some(AttrValue::from("Apps"))
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
            <rect x="3" y="3" width="7" height="7" rx="2" fill={fill_or_stroke.clone()} />
            <rect x="14" y="3" width="7" height="7" rx="2" fill={fill_or_stroke.clone()} />
            <rect x="3" y="14" width="7" height="7" rx="2" fill={fill_or_stroke.clone()} />
            <rect x="14" y="14" width="7" height="7" rx="2" fill={fill_or_stroke} />
        </IconBase>
    }
}
