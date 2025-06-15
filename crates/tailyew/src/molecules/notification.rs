use crate::{
    atoms::{Button, ButtonType, TagType, Typo},
    XIcon,
};
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
    /// Controls visible state
    pub visible: bool,
    /// Whether to show the close (X) button
    #[prop_or(true)]
    pub show_close: bool,
    /// Callback when close button is clicked
    #[prop_or_default]
    pub on_close: Option<Callback<()>>,
    /// Max-width Tailwind classes (e.g. "max-w-sm md:max-w-md" or "max-w-full")
    #[prop_or("max-w-full".into())]
    pub max_width: String,
    /// Fixed vs relative positioning
    #[prop_or(true)]
    pub fixed: bool,
}

#[function_component(Notification)]
pub fn notification(props: &NotificationProps) -> Html {
    let NotificationProps {
        message,
        notification_type,
        visible,
        show_close,
        on_close,
        max_width,
        fixed,
    } = props.clone();

    let is_visible = use_state(|| visible);
    let last_visible = use_state(|| visible);

    // Sync internal state when parent `visible` changes
    {
        let is_visible = is_visible.clone();
        let last_visible = last_visible.clone();
        use_effect_with(visible, move |new_visible| {
            if *new_visible != *last_visible {
                is_visible.set(*new_visible);
                last_visible.set(*new_visible);
            }
            || ()
        });
    }

    if !*is_visible {
        return html! {};
    }

    let (bg_color, border_color, text_color, icon) = match notification_type {
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
                position_class,
                "relative", "overflow-hidden", "p-4", "w-full",
                max_width,
                bg_color, border_color,
                "rounded-lg", "shadow-lg"
            )}
        >
            // Centered content
            <div class="flex items-center justify-center space-x-2">
                { icon.map(|sym| html! { <Typo tag={TagType::Span} class={text_color}>{ sym }</Typo> }) }
                <Typo tag={TagType::Span} class={classes!(text_color, "text-base", "font-medium", "text-center")}>
                    { message }
                </Typo>
            </div>

            // Close button floats top-right
            {
                if show_close {
                    html! {
                        <Button
                            onclick={on_close_click}
                            button_type={ButtonType::Icon}
                            class={classes!("absolute", "top-2", "right-2")}
                        >
                            <XIcon />
                        </Button>
                    }
                } else { html! {} }
            }
        </div>
    }
}
