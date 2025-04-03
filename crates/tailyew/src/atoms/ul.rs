// crates/tailyew/src/atoms/ul.rs

use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub enum MarkerType {
    Disc,
    Decimal,
    None,
}

impl Default for MarkerType {
    fn default() -> Self {
        Self::None // Sidebar use case: no bullets by default
    }
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

    #[prop_or_else(|| "space-y-1".into())] // Small vertical spacing by default
    pub spacing: Classes,

    #[prop_or_default]
    pub marker_type: MarkerType,
}

#[function_component(Ul)]
pub fn ul(props: &UlProps) -> Html {
    let UlProps {
        children,
        class,
        spacing,
        marker_type,
    } = props;

    let ul_classes = classes!(
        marker_type.as_class(),
        "pl-0",
        spacing.clone(),
        class.clone(),
    );

    html! {
        <ul class={ul_classes}>
            { for children.iter() }
        </ul>
    }
}
