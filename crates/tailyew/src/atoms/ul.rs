// crates/tailyew/src/atoms/ul.rs

use yew::prelude::*;

#[derive(PartialEq, Clone, Default)]
pub enum MarkerType {
    Disc,
    Decimal,
    #[default]
    None,
}

impl MarkerType {
    fn as_class(&self) -> &'static str {
        match self {
            MarkerType::Disc => "list-disc",
            MarkerType::Decimal => "list-decimal",
            MarkerType::None => "list-none",
        }
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct UlProps {
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    // Compact vertical spacing by default
    #[prop_or_else(|| "space-y-1".into())]
    pub spacing: Classes,

    #[prop_or_default]
    pub marker_type: MarkerType,

    #[prop_or_else(|| "marker:text-gray-400 dark:marker:text-gray-500".into())]
    pub marker_color: Classes,

    #[prop_or_default]
    pub dense: bool,
}

#[function_component(Ul)]
pub fn ul(props: &UlProps) -> Html {
    let UlProps {
        children,
        class,
        spacing,
        marker_type,
        marker_color,
        dense,
    } = props;

    let spacing_classes = if *dense {
        classes!("space-y-1")
    } else {
        spacing.clone()
    };

    let has_markers = !matches!(marker_type, MarkerType::None);

    let list_style = match marker_type {
        MarkerType::Disc => "list-style-type: disc;",
        MarkerType::Decimal => "list-style-type: decimal;",
        MarkerType::None => "list-style-type: none;",
    };

    // Always behave like the old `MarkerAlign::Outside`
    let align_class = if has_markers {
        "list-outside"
    } else {
        "list-none"
    };

    let padding_class = if has_markers { "pl-6" } else { "pl-0" };

    let ul_classes = classes!(
        marker_type.as_class(),
        align_class,
        padding_class,
        "text-gray-800",
        "dark:text-gray-100",
        if *dense {
            Some("text-sm leading-5")
        } else {
            Some("text-base leading-6")
        },
        marker_color.clone(),
        spacing_classes,
        class.clone(),
    );

    html! {
        <ul class={ul_classes} style={list_style}>
            { for children.iter() }
        </ul>
    }
}
