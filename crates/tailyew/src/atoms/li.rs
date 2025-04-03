// crates/tailyew/src/atoms/li.rs

use yew::prelude::*;

/// A single item in a list, often used in navigation or option lists
#[derive(Properties, PartialEq, Clone)]
pub struct LiProps {
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub active: bool,

    #[prop_or_default]
    pub hover: bool,

    #[prop_or_default]
    pub with_icon: bool,

    #[prop_or_default]
    pub icon: Option<Html>,

    #[prop_or_default]
    pub bordered: bool,

    #[prop_or_default]
    pub background: Classes,

    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
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
        with_icon,
    } = props;

    let icon_section = if *with_icon {
        html! {
            <span class="flex-shrink-0 p-2 bg-gray-700 rounded-full text-white">
                { icon.clone().unwrap_or_default() }
            </span>
        }
    } else {
        Html::default()
    };

    let li_classes = classes!(
        "transition-colors",
        "duration-200",
        "text-gray-700",
        "dark:text-gray-200",
        class.clone(),
        background.clone(),
        if *active {
            Some("bg-gray-100 dark:bg-gray-800 font-semibold")
        } else {
            None
        },
        if *hover {
            Some("hover:bg-gray-100 dark:hover:bg-gray-800")
        } else {
            None
        },
        if *bordered {
            Some("border-b border-gray-200 dark:border-gray-600")
        } else {
            None
        },
        if onclick.is_some() {
            Some("cursor-pointer")
        } else {
            None
        },
    );

    html! {
        <li class={li_classes} onclick={onclick.clone()}>
            { icon_section }
            <div class="ml-2">{ for children.iter() }</div>
        </li>
    }
}
