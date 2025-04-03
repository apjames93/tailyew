use yew::prelude::*;

/// Properties for the Anchor (A) component
#[derive(Properties, PartialEq, Clone)]
pub struct AProps {
    pub href: String,
    pub children: Children,
    #[prop_or_default]
    pub target: Option<String>,
    #[prop_or_default]
    pub class: Classes,
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
    } = props;

    let merged_classes = classes!(
        "text-blue-500",
        "dark:text-blue-300",
        "hover:text-blue-700",
        "dark:hover:text-blue-500",
        "focus:outline-none",
        "focus:ring-2",
        "focus:ring-blue-400",
        "dark:focus:ring-blue-600",
        "transition-colors",
        "duration-200",
        "underline",
        class.clone(), // now safely a `Classes`
    );

    let on_click_handler = onclick.clone().map(|cb| {
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            cb.emit(e);
        })
    });

    html! {
        <a
            href={href.clone()}
            target={target.clone()}
            rel={if target.as_deref() == Some("_blank") { Some("noopener noreferrer") } else { None }}
            class={merged_classes}
            onclick={on_click_handler}
        >
            { for children.iter() }
        </a>
    }
}
