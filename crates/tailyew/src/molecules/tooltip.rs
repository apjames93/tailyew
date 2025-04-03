use yew::prelude::*;

/// Enum for positioning the tooltip relative to its trigger
#[derive(PartialEq, Clone, Default)]
pub enum TooltipPosition {
    Top,
    Right,
    #[default]
    Bottom,
    Left,
}

#[derive(Properties, PartialEq, Clone)]
pub struct TooltipProps {
    pub trigger: Html,
    pub content: Html,
    #[prop_or_default]
    pub position: TooltipPosition,
}

#[function_component(Tooltip)]
pub fn tooltip(props: &TooltipProps) -> Html {
    let TooltipProps {
        trigger,
        content,
        position,
    } = props.clone();
    let show = use_state(|| false);

    let on_mouse_enter = {
        let show = show.clone();
        Callback::from(move |_| show.set(true))
    };

    let on_mouse_leave = {
        let show = show.clone();
        Callback::from(move |_| show.set(false))
    };

    let position_class = match position {
        TooltipPosition::Top => vec![
            "bottom-full",
            "left-1/2",
            "transform",
            "-translate-x-1/2",
            "mb-2",
        ],
        TooltipPosition::Right => vec![
            "left-full",
            "top-1/2",
            "transform",
            "-translate-y-1/2",
            "ml-2",
        ],
        TooltipPosition::Bottom => vec![
            "top-full",
            "left-1/2",
            "transform",
            "-translate-x-1/2",
            "mt-2",
        ],
        TooltipPosition::Left => vec![
            "right-full",
            "top-1/2",
            "transform",
            "-translate-y-1/2",
            "mr-2",
        ],
    };

    html! {
        <div
            class="relative inline-block"
            onmouseenter={on_mouse_enter}
            onmouseleave={on_mouse_leave}
        >
            <div>{ trigger }</div>

            if *show {
                <div
                    class={classes!(
                        "absolute", "z-50", "text-xs", "px-3", "py-2", "rounded",
                        "shadow-lg", "whitespace-nowrap",
                        "bg-gray-900", "dark:bg-gray-800", "text-white", "dark:text-gray-100",
                        "transition-all", "duration-200", "ease-in-out", "transform",
                        "scale-100", "opacity-100",
                        position_class
                    )}
                    role="tooltip"
                    aria-hidden={(!*show).to_string()}
                >
                    { content.clone() }
                </div>
            }
        </div>
    }
}
