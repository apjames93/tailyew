use crate::system::use_themed_classes;
use yew::prelude::*;

/// Define the possible HTML tag types for the Button component
#[derive(Clone, Default, PartialEq)]
pub enum ButtonType {
    #[default]
    Primary,
    Secondary,
    Danger,
    Submit,
    Ghost,
    Button,
    Icon,
}

impl ButtonType {
    fn as_str(&self) -> &'static str {
        match self {
            ButtonType::Submit => "submit",
            _ => "button",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or(ButtonType::Button)]
    pub button_type: ButtonType,

    #[prop_or_default]
    pub on_click: Option<Callback<MouseEvent>>,

    #[prop_or(false)]
    pub disabled: bool,

    #[prop_or_default]
    pub class: Classes,

    /// Optional `form="..."` attribute so you can submit an external `<form id=…>`
    #[prop_or_default]
    pub form: Option<AttrValue>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub aria_label: Option<AttrValue>,

    #[prop_or_default]
    pub role: Option<AttrValue>,

    #[prop_or_default]
    pub aria_expanded: Option<AttrValue>,

    #[prop_or_default]
    pub aria_controls: Option<AttrValue>,
}

#[component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let ButtonProps {
        button_type,
        on_click,
        disabled,
        class,
        form,
        children,
        aria_label,
        role,
        aria_expanded,
        aria_controls,
    } = props;

    // Shared focus ring for non-disabled buttons
    let shared_focus = "focus:outline-none focus:ring-2 focus:ring-accent focus:ring-offset-2 dark:focus:ring-accent-dark";

    // Determine base styles for each button type
    let button_style = match button_type {
        ButtonType::Primary => "bg-primary hover:bg-primary-dark text-content-invert font-bold dark:bg-primary-dark dark:hover:bg-primary dark:text-content-invert",
        ButtonType::Secondary => "bg-secondary hover:bg-secondary-dark text-content-invert font-bold dark:bg-secondary-dark dark:hover:bg-secondary dark:text-content-invert",
        ButtonType::Danger => "bg-danger hover:bg-danger-dark text-content-invert font-bold dark:bg-danger-dark dark:hover:bg-danger dark:text-content-invert",
        ButtonType::Submit => "bg-success hover:bg-success-dark text-content-invert font-bold dark:bg-success-dark dark:hover:bg-success dark:text-content-invert",
        ButtonType::Ghost => "bg-transparent text-content dark:text-content-invert hover:bg-surface-muted dark:hover:bg-surface-dark border border-transparent hover:border-border dark:hover:border-border-dark",
        ButtonType::Icon => "text-content-muted dark:text-content-muted-dark hover:text-content dark:hover:text-content-invert transition duration-150 border-none",
        ButtonType::Button => "bg-accent hover:bg-accent-dark text-content-invert font-bold dark:bg-accent-dark dark:hover:bg-accent dark:text-content-invert",
    };

    // Disabled state overrides
    let disabled_style = if *disabled {
        "bg-surface-muted text-content-muted cursor-not-allowed dark:bg-surface-dark dark:text-content-muted-dark"
    } else {
        shared_focus
    };

    // Combine final class list
    let defaults = classes!(
        "py-2",
        "px-4",
        "rounded-lg",
        "shadow-sm",
        "transition",
        "duration-150",
        button_style,
        disabled_style,
    );
    let button_classes = use_themed_classes("Button", "root", defaults, class.clone());

    // If this is a submit‐type button, we let the form attribute handle submission,
    // and we don't wire up an onclick
    let on_click_callback = if *button_type == ButtonType::Submit {
        None
    } else {
        on_click.clone()
    };

    html! {
        <button
            class={button_classes}
            type={button_type.as_str()}
            onclick={on_click_callback}
            disabled={*disabled}
            aria-disabled={Some(disabled.to_string())}
            role={role.clone()}
            aria-label={aria_label.clone()}
            aria-expanded={aria_expanded.clone()}
            aria-controls={aria_controls.clone()}
            // only emit `form="…"`, if Some
            form={form.clone()}
    >
            { for children.iter() }
        </button>
    }
}
