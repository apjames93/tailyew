// crates/tailyew/src/atoms/li.rs

use yew::prelude::*;

#[derive(PartialEq, Clone, Copy, Default)]
pub enum IconPosition {
    #[default]
    Left,
    Right,
}

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
    pub icon: Option<Html>,

    #[prop_or_default]
    pub icon_position: IconPosition,

    #[prop_or_default]
    pub bordered: bool,

    #[prop_or_default]
    pub background: Classes,

    #[prop_or_default]
    pub on_click: Option<Callback<MouseEvent>>,
}

#[component(Li)]
pub fn li(props: &LiProps) -> Html {
    let LiProps {
        active,
        background,
        bordered,
        children,
        class,
        hover,
        icon,
        icon_position,
        on_click,
    } = props;

    let icon_section = icon.as_ref().map(|icon| {
        let spacing_class = match icon_position {
            IconPosition::Left => "mr-3",
            IconPosition::Right => "ml-3",
        };

        html! {
            <span class={classes!(
                "flex-shrink-0",
                "inline-flex",
                "items-center",
                "justify-center",
                "text-gray-500",
                "dark:text-gray-300",
                spacing_class,
            )}>
                { icon.clone() }
            </span>
        }
    });

    let li_classes = classes!(
        "transition-colors",
        "duration-200",
        "text-gray-800",
        "dark:text-gray-100",
        "rounded-md",
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
        if on_click.is_some() {
            Some("cursor-pointer")
        } else {
            None
        },
    );

    let row_classes = classes!("flex", "items-center", "gap-2", "w-full");
    let content_classes = classes!("flex-1", "min-w-0", "text-sm");

    let (leading_icon, trailing_icon) = match icon_section {
        Some(icon) if matches!(icon_position, IconPosition::Right) => (Html::default(), icon),
        Some(icon) => (icon, Html::default()),
        None => (Html::default(), Html::default()),
    };

    html! {
        <li class={li_classes} onclick={on_click.clone()}>
            <div class={row_classes}>
                { leading_icon }
                <div class={content_classes}>{ for children.iter() }</div>
                { trailing_icon }
            </div>
        </li>
    }
}
