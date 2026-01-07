use crate::atoms::{TagType, Typo};
use crate::icons::ArrowDownIcon;
use wasm_bindgen::JsCast;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct AccordionProps {
    pub title: Html,
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub trigger_classes: Classes,

    #[prop_or_default]
    pub content_class: Classes,

    #[prop_or(TagType::Span)]
    pub heading_tag: TagType,

    #[prop_or(false)]
    pub default_open: bool,

    #[prop_or(false)]
    pub compact: bool,

    #[prop_or_default]
    pub arrow: Option<Html>,
}

#[component(Accordion)]
pub fn accordion(props: &AccordionProps) -> Html {
    let AccordionProps {
        title,
        class,
        trigger_classes,
        content_class,
        heading_tag,
        default_open,
        compact,
        arrow,
        children,
    } = props.clone();

    let is_open = use_state(move || default_open);

    let toggle_open = {
        let is_open = is_open.clone();
        Callback::from(move |e: MouseEvent| {
            let target = e.target();
            if let Some(el) = target {
                let tag = el
                    .dyn_ref::<web_sys::Element>()
                    .map(|el| el.tag_name().to_ascii_lowercase());
                if matches!(
                    tag.as_deref(),
                    Some("input" | "textarea" | "button" | "select")
                ) {
                    // Click was inside an interactive field, don't toggle
                    return;
                }
            }
            is_open.set(!*is_open);
        })
    };

    let on_keypress = {
        let is_open = is_open.clone();
        Callback::from(move |e: KeyboardEvent| {
            let tag = e.target().and_then(|t| {
                t.dyn_ref::<web_sys::Element>()
                    .map(|el| el.tag_name().to_ascii_lowercase())
            });
            if matches!(
                tag.as_deref(),
                Some("input" | "textarea" | "button" | "select")
            ) {
                // Ignore keypresses in interactive elements
                return;
            }

            if e.key() == "Enter" || e.key() == " " {
                e.prevent_default();
                is_open.set(!*is_open);
            }
        })
    };

    let wrapper_classes = if compact {
        classes!("rounded-lg", class.clone())
    } else {
        classes!(
            "border",
            "border-gray-300",
            "dark:border-gray-700",
            "rounded-lg",
            "overflow-hidden",
            "shadow-md",
            class.clone()
        )
    };

    let base_trigger_classes = if compact {
        classes!(
            "w-full",
            "bg-gray-200",
            "dark:bg-gray-800",
            "hover:bg-gray-300",
            "dark:hover:bg-gray-700",
            "px-4",
            "py-3",
            "flex",
            "items-center",
            "justify-between",
            "rounded-lg",
            "shadow-md",
            "transition",
            trigger_classes.clone()
        )
    } else {
        classes!(
            "cursor-pointer",
            "bg-gray-200",
            "dark:bg-gray-800",
            "hover:bg-gray-300",
            "dark:hover:bg-gray-700",
            "px-3",
            "py-2",
            "flex",
            "justify-between",
            "items-center",
            "transition",
            "duration-200",
            trigger_classes.clone()
        )
    };

    let arrow_classes = {
        let mut c = Classes::from(vec!["transform", "transition-transform", "duration-200"]);
        if *is_open {
            c.push("rotate-180");
            c.push("text-primary");
        } else {
            c.push("text-gray-500");
        }
        c
    };

    let base_content_classes = if compact {
        classes!("space-y-1", content_class.clone())
    } else {
        classes!(
            "px-3",
            "py-2",
            "border-t",
            "dark:border-gray-700",
            "bg-white",
            "dark:bg-gray-900",
            "transition-all",
            "duration-300",
            "ease-in-out",
            content_class.clone()
        )
    };

    // Tailwind transition class handling
    let visibility_classes = if compact {
        classes!(
            "overflow-hidden",
            "transition-[max-height]",
            "duration-300",
            "ease-in-out",
            if *is_open {
                "max-h-[1000px]"
            } else {
                "max-h-0"
            }
        )
    } else {
        let state_classes = if *is_open {
            vec!["opacity-100", "scale-y-100"]
        } else {
            vec!["opacity-0", "scale-y-0", "pointer-events-none", "hidden"]
        };

        classes!(
            "transition-all",
            "duration-300",
            "ease-in-out",
            "transform",
            "origin-top",
            state_classes
        )
    };

    html! {
        <div class={wrapper_classes}>
            <div
                role="button"
                tabindex="0"
                class={base_trigger_classes}
                onclick={toggle_open}
                onkeypress={on_keypress}
                aria-expanded={(*is_open).to_string()}
                aria-controls="accordion-panel"
            >
                <Typo class={"w-full text-left"} tag={heading_tag}>{ title }</Typo>
                {
                    if let Some(icon) = arrow {
                        icon
                    } else {
                        html! { <Typo class={arrow_classes}><ArrowDownIcon /></Typo> }
                    }
                }
            </div>

            <div
                id="accordion-panel"
                class={classes!(base_content_classes, visibility_classes)}
            >
                { for children.iter() }
            </div>
        </div>
    }
}
