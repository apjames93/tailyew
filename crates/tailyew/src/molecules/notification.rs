use crate::atoms::{Button, ButtonType, TagType, Typo};
use crate::icons::XIcon;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum NotificationTypes {
    Error,
    Info,
    #[default]
    Primary,
    Success,
    Warning,
}

#[derive(Properties, PartialEq, Clone)]
pub struct NotificationProps {
    pub message: String,
    pub notification_type: NotificationTypes,
    pub is_visible: bool,
    #[prop_or_default]
    pub on_close: Option<Callback<()>>,
    #[prop_or_default]
    pub is_fixed: Option<bool>,
}

#[function_component(Notification)]
pub fn notification(props: &NotificationProps) -> Html {
    let NotificationProps {
        message,
        notification_type,
        is_visible,
        on_close,
        is_fixed,
    } = props.clone();

    let visible_state = use_state(|| is_visible);

    // Update internal state whenever `is_visible` prop changes
    {
        let visible_state = visible_state.clone();
        use_effect_with(is_visible, move |new_is_visible| {
            visible_state.set(*new_is_visible);
            || ()
        });
    }

    if !*visible_state {
        return html! { <></> };
    }

    let (bg_color, border_color, text_color, icon) = match notification_type {
        NotificationTypes::Success => (
            "bg-green-100 dark:bg-green-800",
            "border-green-400",
            "text-green-800 dark:text-green-200",
            "✓",
        ),
        NotificationTypes::Error => (
            "bg-red-100 dark:bg-red-800",
            "border-red-400",
            "text-red-800 dark:text-red-200",
            "✗",
        ),
        NotificationTypes::Warning => (
            "bg-yellow-100 dark:bg-yellow-700",
            "border-yellow-400",
            "text-yellow-800 dark:text-yellow-200",
            "!",
        ),
        NotificationTypes::Info => (
            "bg-blue-100 dark:bg-blue-700",
            "border-blue-400",
            "text-blue-800 dark:text-blue-200",
            "ℹ",
        ),
        NotificationTypes::Primary => (
            "bg-gray-100 dark:bg-gray-800",
            "border-gray-400",
            "text-gray-800 dark:text-gray-200",
            "",
        ),
    };

    let on_close_click = {
        let visible_state = visible_state.clone();
        let on_close = on_close.clone();
        Callback::from(move |_| {
            visible_state.set(false);

            if let Some(close_callback) = on_close.clone() {
                close_callback.emit(());
            }
        })
    };

    let position_class = if is_fixed.unwrap_or(true) {
        "fixed top-4 right-4 z-50"
    } else {
        "relative"
    };

    html! {
        <div
            class={classes!(
                position_class, "p-4", "w-full", "max-w-sm", "md:max-w-md",
                "flex", "items-center", "justify-between", "border-l-4", "rounded-lg", "shadow-lg",
                "transition-transform", "duration-300", "ease-in-out", "transform", "scale-100",
                bg_color, border_color
            )}
        >
            <Typo tag={TagType::Span} class={classes!(text_color)}>{icon}</Typo>

            <Typo tag={TagType::Span} class={classes!(text_color, "text-base", "font-medium", "flex-grow")}>
                { message.clone() }
            </Typo>

            <Button
                onclick={on_close_click}
                button_type={ButtonType::Icon}
            >
                <XIcon />
            </Button>
        </div>
    }
}
