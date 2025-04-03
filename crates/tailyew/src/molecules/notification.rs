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
    pub visible: bool,
    #[prop_or_default]
    pub on_close: Option<Callback<()>>,
    #[prop_or(true)]
    pub fixed: bool,
}

#[function_component(Notification)]
pub fn notification(props: &NotificationProps) -> Html {
    let NotificationProps {
        message,
        notification_type,
        visible,
        on_close,
        fixed,
    } = props.clone();

    let is_visible = use_state(|| visible);

    // Manually sync prop if it changes
    if *is_visible != visible {
        is_visible.set(visible);
    }

    if !*is_visible {
        return html! {};
    }

    let (bg_color, border_color, text_color, icon): (&str, &str, &str, Option<&str>) =
        match notification_type {
            NotificationTypes::Success => (
                "bg-green-100 dark:bg-green-800",
                "border-green-400",
                "text-green-800 dark:text-green-200",
                Some("✓"),
            ),
            NotificationTypes::Error => (
                "bg-red-100 dark:bg-red-800",
                "border-red-400",
                "text-red-800 dark:text-red-200",
                Some("✗"),
            ),
            NotificationTypes::Warning => (
                "bg-yellow-100 dark:bg-yellow-700",
                "border-yellow-400",
                "text-yellow-800 dark:text-yellow-200",
                Some("!"),
            ),
            NotificationTypes::Info => (
                "bg-blue-100 dark:bg-blue-700",
                "border-blue-400",
                "text-blue-800 dark:text-blue-200",
                Some("ℹ"),
            ),
            NotificationTypes::Primary => (
                "bg-gray-100 dark:bg-gray-800",
                "border-gray-400",
                "text-gray-800 dark:text-gray-200",
                None,
            ),
        };

    let on_close_click = {
        let is_visible = is_visible.clone();
        let on_close = on_close.clone();
        Callback::from(move |_| {
            is_visible.set(false);
            if let Some(cb) = on_close.clone() {
                cb.emit(());
            }
        })
    };

    let position_class = if fixed {
        "fixed top-4 right-4 z-50"
    } else {
        "relative"
    };

    html! {
        <div
            role="alert"
            class={classes!(
                position_class, "p-4", "w-full", "max-w-sm", "md:max-w-md",
                "flex", "items-center", "justify-between", "border-l-4", "rounded-lg", "shadow-lg",
                "transition-transform", "duration-300", "ease-in-out", "transform", "scale-100",
                bg_color, border_color
            )}
        >
            if let Some(symbol) = icon {
                <Typo tag={TagType::Span} class={text_color}>{symbol}</Typo>
            }

            <Typo tag={TagType::Span} class={classes!(text_color, "text-base", "font-medium", "flex-grow", "ml-2")}>
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
