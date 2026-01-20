use yew::prelude::*;

/// Properties for the Anchor (A) component
#[derive(Properties, PartialEq, Clone)]
pub struct AProps {
    pub href: AttrValue,
    pub children: Children,

    #[prop_or_default]
    pub target: Option<AttrValue>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub on_click: Option<Callback<MouseEvent>>,

    #[prop_or_default]
    pub aria_label: Option<AttrValue>,

    #[prop_or_default]
    pub aria_describedby: Option<AttrValue>,

    #[prop_or_default]
    pub role: Option<AttrValue>,

    #[prop_or_default]
    pub tabindex: Option<i16>,

    #[prop_or(true)]
    pub include_referrer: bool,
}

#[component(A)]
pub fn a(props: &AProps) -> Html {
    let AProps {
        href,
        children,
        target,
        class,
        on_click,
        aria_label,
        aria_describedby,
        role,
        tabindex,
        include_referrer,
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
        class.clone(),
    );

    let on_click_handler = on_click.clone().map(|cb| {
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            cb.emit(e);
        })
    });

    html! {
        <a
            href={href.clone()}
            target={target.clone()}
            rel={if target.as_deref() == Some("_blank") && !include_referrer {
                Some("noopener noreferrer")
            } else if target.as_deref() == Some("_blank") {
                Some("noopener")
            } else {
                None
            }}
            class={merged_classes}
            onclick={on_click_handler}
            aria-label={aria_label.clone()}
            aria-describedby={aria_describedby.clone()}
            role={role.clone()}
            tabindex={tabindex.map(|i| i.to_string())}
        >
            { for children.iter() }
        </a>
    }
}
