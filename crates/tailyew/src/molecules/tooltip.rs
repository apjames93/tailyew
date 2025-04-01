// src/molecules/tooltip.rs

use yew::prelude::*;

/// Properties for the Tooltip component
#[derive(Properties, PartialEq, Clone)]
pub struct TooltipProps {
    pub trigger: Html, // The element that will trigger the tooltip on hover
    pub content: Html, // The content of the tooltip
    #[prop_or("bottom".to_string())] // Default to bottom position if not provided
    pub position: String, // Position of the tooltip: top, right, bottom, left
}

#[function_component(Tooltip)]
pub fn tooltip(props: &TooltipProps) -> Html {
    let TooltipProps {
        trigger,
        content,
        position,
    } = props.clone();

    let show_tooltip = use_state(|| false);

    // Callback to show tooltip on mouse enter
    let on_mouse_enter = {
        let show_tooltip = show_tooltip.clone();
        Callback::from(move |_| show_tooltip.set(true))
    };

    // Callback to hide tooltip on mouse leave
    let on_mouse_leave = {
        let show_tooltip = show_tooltip.clone();
        Callback::from(move |_| show_tooltip.set(false))
    };

    // Calculate tooltip position styles based on the `position` prop
    let tooltip_position_class = match position.as_str() {
        "top" => "bottom-full left-1/2 transform -translate-x-1/2 mb-2",
        "right" => "left-full top-1/2 transform -translate-y-1/2 ml-2",
        "bottom" => "top-full left-1/2 transform -translate-x-1/2 mt-2",
        "left" => "right-full top-1/2 transform -translate-y-1/2 mr-2",
        _ => "top-full left-1/2 transform -translate-x-1/2 mt-2", // Default to bottom
    };

    html! {
        <div class="relative inline-block"
            onmouseenter={on_mouse_enter}
            onmouseleave={on_mouse_leave}
        >
            // Trigger element
            <div>
                { trigger.clone() }
            </div>

            // Tooltip content with theme colors and transitions
            { if *show_tooltip {
                html! {
                    <div class={format!("absolute p-2 bg-gray-900 dark:bg-gray-800 text-white dark:text-gray-100 rounded shadow-lg z-20 transition-opacity duration-200 ease-in-out transform scale-95 {}", tooltip_position_class)}>
                        { content.clone() }
                    </div>
                }
            } else {
                html! { <></> }
            }}
        </div>
    }
}
