use crate::system::use_themed_classes;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct BadgeProps {
    /// Content inside the badge (text, number, dot)
    #[prop_or_default]
    pub badge_content: Option<String>,

    /// Whether the badge should be shown when content is "0"
    #[prop_or(false)]
    pub show_zero: bool,

    /// Show a small dot instead of content
    #[prop_or(false)]
    pub is_dot: bool,

    /// Maximum number before truncation (e.g. "99+")
    #[prop_or(99)]
    pub max: usize,

    /// Tailwind color classes
    #[prop_or_else(|| "bg-red-500 text-white".into())]
    pub color: Classes,

    /// Badge position: top-right, bottom-left, etc.
    #[prop_or_else(|| "top-0 right-0".into())]
    pub position: Classes,

    /// Optional additional styles
    #[prop_or_default]
    pub class: Classes,

    pub children: Children,
}

#[component(Badge)]
pub fn badge(props: &BadgeProps) -> Html {
    let BadgeProps {
        badge_content,
        show_zero,
        is_dot,
        max,
        color,
        position,
        class,
        children,
    } = props.clone();

    let content = badge_content.unwrap_or_default();
    let should_hide = (!show_zero && content == "0") && !is_dot;

    let display_content = if is_dot {
        html! {}
    } else if let Ok(num) = content.parse::<usize>() {
        let display = if num > max {
            format!("{}+", max)
        } else {
            content.clone()
        };
        html! { {display} }
    } else {
        html! { {content.clone()} }
    };

    let wrapper_classes = use_themed_classes(
        "Badge",
        "root",
        classes!("relative", "inline-block"),
        Classes::default(),
    );
    let badge_classes = use_themed_classes(
        "Badge",
        "badge",
        classes!(
            "absolute",
            "flex",
            "items-center",
            "justify-center",
            "rounded-full",
            "text-xs",
            "px-1.5",
            "h-5",
            "min-w-[1.25rem]",
            "transform",
            "translate-x-1/2",
            "-translate-y-1/2",
            if is_dot { "w-2 h-2 p-0" } else { "" },
            color.clone(),
            position.clone(),
        ),
        class.clone(),
    );

    html! {
        <div class={wrapper_classes}>
            { for children.iter() }
            {
                if !should_hide {
                    html! {
                        <span class={badge_classes.clone()}>
                            { if is_dot { html! {} } else { display_content } }
                        </span>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}
