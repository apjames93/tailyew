// src/atoms/li.rs

use yew::prelude::*;

/// Properties for the `Li` component
#[derive(Properties, PartialEq, Clone)]
pub struct LiProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Option<String>, // Optional class for custom styling
    #[prop_or_default]
    pub active: bool, // Prop to indicate active state
    #[prop_or_default]
    pub hover: bool, // Prop to indicate if hover styles should be applied
    #[prop_or_default]
    pub with_icon: bool, // Optional icon marker
    #[prop_or_default]
    pub icon: Option<Html>, // Custom icon if `with_icon` is true
    #[prop_or_default]
    pub bordered: bool, // Prop to add border styling
    #[prop_or_default]
    pub spacing: Option<String>, // Optional spacing between items
    #[prop_or_default]
    pub background: Option<String>, // Background color for list items
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>, // Optional onclick event handler
}

#[function_component(Li)]
pub fn li(props: &LiProps) -> Html {
    let LiProps {
        active,
        background,
        bordered,
        children,
        class,
        hover,
        icon,
        onclick,
        spacing,
        with_icon,
    } = props;

    // Determine whether an `onclick` callback is present
    let onclick_callback = onclick.clone();

    // Default classes for the list item
    let default_classes = classes!(
        "flex",         // Flex layout for icon and text alignment
        "items-center", // Center align content vertically
        "px-4",         // Horizontal padding
        "py-4",         // Vertical padding
        "rounded-lg",   // Rounded corners
        "transition-all",
        "duration-300",
        "ease-in-out",
        spacing.clone().unwrap_or_else(|| "mb-4".to_string()), // Optional spacing between items
        if *active {
            "bg-blue-100 text-blue-600 font-semibold"
        } else {
            ""
        }, // Active state styling
        if *hover {
            "hover:bg-gray-100 dark:hover:bg-gray-700"
        } else {
            ""
        }, // Hover state styling
        if *bordered {
            "border-b border-gray-300 dark:border-gray-600"
        } else {
            ""
        }, // Border styling for each list item
        if onclick.is_some() {
            "cursor-pointer"
        } else {
            ""
        }, // Show pointer cursor if clickable
        background
            .clone()
            .unwrap_or_else(|| "bg-white dark:bg-gray-800".to_string()), // Background color
        class.clone(),                                         // Additional custom classes
    );

    // Create the icon section if `with_icon` is true
    let icon_section = if *with_icon {
        html! {
            <span class="flex-shrink-0 p-2 bg-gray-700 rounded-full text-gray-200">
                { icon.clone().unwrap_or_default() }
            </span>
        }
    } else {
        html! {}
    };

    html! {
        <li
            class={default_classes}
            onclick={onclick_callback}
        >
            { icon_section } // Icon section
            <div class="ml-4 space-y-1"> // Space between icon and text content
                { for children.iter() } // Render children content
            </div>
        </li>
    }
}
