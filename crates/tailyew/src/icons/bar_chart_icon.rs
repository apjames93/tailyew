use crate::system::use_themed_classes;
use yew::prelude::*;

use crate::icons::icon_base::IconBase;

/// Properties for the BarChartIcon component.
#[derive(Properties, PartialEq, Clone)]
pub struct BarChartIconProps {
    /// Extra classes for the svg
    #[prop_or_default]
    pub class: Classes,

    /// Icon size in px
    #[prop_or(24)]
    pub size: u32,

    /// Stroke width to match the rest of the icon set
    #[prop_or(1.5)]
    pub stroke_width: f32,

    /// Accessible label
    #[prop_or_default]
    pub label: Option<AttrValue>,

    /// If true, hide from screen readers
    #[prop_or(false)]
    pub decorative: bool,

    /// Optional explicit stroke/fill color for the bars
    #[prop_or_default]
    pub color: Option<AttrValue>,
}

/// Bar chart icon for TailYew
#[component(BarChartIcon)]
pub fn bar_chart_icon(props: &BarChartIconProps) -> Html {
    let icon_class = use_themed_classes(
        "BarChartIcon",
        "root",
        Classes::default(),
        props.class.clone(),
    );
    let stroke_or_fill = props
        .color
        .clone()
        .unwrap_or_else(|| AttrValue::from("currentColor"));

    // give it a default label if user didn't set one and it's not decorative
    let label = if props.label.is_none() && !props.decorative {
        Some(AttrValue::from("Bar chart"))
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
            // these are bars, so it's fine to just set fill explicitly
            <rect x="4" y="12" width="3" height="8" rx="1" fill={stroke_or_fill.clone()} />
            <rect x="10.5" y="8" width="3" height="12" rx="1" fill={stroke_or_fill.clone()} />
            <rect x="17" y="4" width="3" height="16" rx="1" fill={stroke_or_fill} />
        </IconBase>
    }
}
