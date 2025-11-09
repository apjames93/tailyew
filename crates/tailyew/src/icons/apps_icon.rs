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
    pub label: Option<String>,

    #[prop_or(false)]
    pub decorative: bool,

    #[prop_or_default]
    pub color: Option<String>,
}

#[function_component(AppsIcon)]
pub fn apps_icon(props: &AppsIconProps) -> Html {
    let fill_or_stroke = props
        .color
        .clone()
        .unwrap_or_else(|| "currentColor".to_string());

    let label = if props.label.is_none() && !props.decorative {
        Some("Apps".to_string())
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
            <rect x="3" y="3" width="7" height="7" rx="2" fill={fill_or_stroke.clone()} />
            <rect x="14" y="3" width="7" height="7" rx="2" fill={fill_or_stroke.clone()} />
            <rect x="3" y="14" width="7" height="7" rx="2" fill={fill_or_stroke.clone()} />
            <rect x="14" y="14" width="7" height="7" rx="2" fill={fill_or_stroke} />
        </IconBase>
    }
}
