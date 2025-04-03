use crate::atoms::{Section, TagType, Typo};
use crate::icons::ArrowDownIcon;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct AccordionProps {
    pub title: String,
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or(TagType::Span)]
    pub heading_tag: TagType,
    #[prop_or(false)]
    pub default_open: bool,
    #[prop_or_default]
    pub arrow: Option<Html>,
}

#[function_component(Accordion)]
pub fn accordion(props: &AccordionProps) -> Html {
    let AccordionProps {
        title,
        class,
        heading_tag,
        default_open,
        arrow,
        children,
    } = props.clone();

    let is_open = use_state(move || default_open);

    let toggle_open_mouse = {
        let is_open = is_open.clone();
        Callback::from(move |_| is_open.set(!*is_open))
    };

    let is_open_clone = is_open.clone();
    let on_keypress = Callback::from(move |e: KeyboardEvent| {
        if e.key() == "Enter" || e.key() == " " {
            e.prevent_default();
            is_open_clone.set(!*is_open_clone);
        }
    });

    let wrapper_classes = classes!(
        "border",
        "border-gray-300",
        "dark:border-gray-700",
        "rounded-lg",
        "overflow-hidden",
        "shadow-md",
        "mb-4",
        class.clone()
    );

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

    html! {
        <div class={wrapper_classes}>
            <div
                role="button"
                tabindex="0"
                class="cursor-pointer bg-gray-200 dark:bg-gray-800 hover:bg-gray-300 dark:hover:bg-gray-700 px-4 py-3 flex justify-between items-center transition duration-200"
                onclick={toggle_open_mouse}
                onkeypress={on_keypress}
                aria-expanded={(*is_open).to_string()}
                aria-controls="accordion-panel"
            >
                <Typo tag={heading_tag} class="text-xl font-medium">{ title }</Typo>
                {
                    if let Some(icon) = arrow {
                        icon
                    } else {
                        html! { <Typo class={arrow_classes}><ArrowDownIcon /></Typo> }
                    }
                }
            </div>

            {
                if *is_open {
                    html! {
                        <Section
                            id="accordion-panel"
                            class="px-4 py-2 border-t dark:border-gray-700 bg-white dark:bg-gray-900 transition-all duration-200 ease-in-out"
                        >
                            { for children.iter() }
                        </Section>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}
