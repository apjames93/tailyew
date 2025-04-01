// src/cp/atoms/a.rs

use yew::prelude::*;

/// Properties for the Anchor (A) component
#[derive(Properties, PartialEq, Clone)]
pub struct AProps {
    pub href: String,
    pub children: Children,
    #[prop_or_default]
    pub target: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
}

#[function_component(A)]
pub fn a(props: &AProps) -> Html {
    let AProps {
        href,
        children,
        target,
        class,
        onclick,
    } = props.clone();

    // Combine the base styles with any additional classes provided as props
    let classes = classes!(
        "text-blue-500",
        "dark:text-blue-300", // Text color for light/dark mode
        "hover:text-blue-700",
        "dark:hover:text-blue-500", // Hover color for light/dark mode
        "focus:outline-none",
        "focus:ring-2",
        "focus:ring-blue-400",
        "dark:focus:ring-blue-600", // Focus outline for accessibility
        "transition-colors",
        "duration-200", // Smooth color transitions
        "underline",    // Underline for all links
        class.clone()
    );

    let on_click_handler = {
        let onclick = onclick.clone();
        Callback::from(move |e: MouseEvent| {
            if let Some(onclick) = &onclick {
                e.prevent_default();
                onclick.emit(e);
            }
        })
    };

    html! {
        <a
            href={href}
            target={target.clone()}
            rel={if target.as_deref() == Some("_blank") { Some("noopener noreferrer") } else { None }}  // Add `rel` attribute for security if target is _blank
            class={classes}
            onclick={on_click_handler}
        >
            { for children.iter() }
        </a>
    }
}
