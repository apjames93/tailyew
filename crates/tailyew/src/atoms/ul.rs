// src/atoms/ul.rs

use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct UlProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Option<String>, // Optional class for custom styling
    #[prop_or_default]
    pub spacing: Option<String>, // Spacing between list items
    #[prop_or_default]
    pub marker_type: Option<String>, // Marker type for list
}

#[function_component(Ul)]
pub fn ul(props: &UlProps) -> Html {
    let UlProps {
        children,
        class,
        spacing,
        marker_type,
    } = props;

    let list_style = match marker_type.as_deref() {
        Some("decimal") => "list-decimal",
        Some("circle") => "list-circle",
        _ => "list-disc", // Default to bullet points
    };

    let ul_classes = classes!(
        list_style,
        spacing.clone().unwrap_or_else(|| "space-y-2".to_string()), // Default spacing if not provided
        "pl-5",                                                     // Padding for list items
        "text-gray-700",
        "dark:text-gray-300",
        class.clone()
    );

    html! {
        <ul class={ul_classes}>
            { for children.iter() }
        </ul>
    }
}
