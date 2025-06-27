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
    pub onclick: Option<Callback<MouseEvent>>,

    #[prop_or(false)]
    pub disabled: bool,

    #[prop_or_default]
    pub class: Classes,

    /// Optional `form="..."` attribute so you can submit an external `<form id=…>`
    #[prop_or_default]
    pub form: Option<String>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub aria_label: Option<String>,

    #[prop_or_default]
    pub role: Option<String>,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let ButtonProps {
        button_type,
        onclick,
        disabled,
        class,
        form,
        children,
        aria_label,
        role,
    } = props;

    // Shared focus ring for non-disabled buttons
    let shared_focus = "focus:outline-none focus:ring-2 focus:ring-accent focus:ring-offset-2 dark:focus:ring-accent-dark";

    // Determine base styles for each button type
    let button_style = match button_type {
        ButtonType::Primary => "bg-primary hover:bg-primary-dark text-white font-bold dark:bg-primary-dark dark:hover:bg-primary",
        ButtonType::Secondary => "bg-secondary hover:bg-secondary-dark text-white font-bold dark:bg-secondary-dark dark:hover:bg-secondary",
        ButtonType::Danger => "bg-danger hover:bg-danger-dark text-white font-bold dark:bg-danger-dark dark:hover:bg-danger",
        ButtonType::Submit => "bg-success hover:bg-success-dark text-white font-bold dark:bg-success-dark dark:hover:bg-success",
        ButtonType::Ghost => "bg-transparent text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-800 border border-transparent hover:border-gray-300 dark:hover:border-gray-600",
        ButtonType::Icon => "text-gray-500 dark:text-gray-300 hover:text-gray-700 dark:hover:text-gray-500 transition duration-150 border-none",
        ButtonType::Button => "bg-accent hover:bg-accent-dark text-white font-bold dark:bg-accent-dark dark:hover:bg-accent",
    };

    // Disabled state overrides
    let disabled_style = if *disabled {
        "bg-neutral text-gray-200 cursor-not-allowed dark:bg-neutral-dark dark:text-gray-400"
    } else {
        shared_focus
    };

    // Combine final class list
    let button_classes = classes!(
        "py-2",
        "px-4",
        "rounded-lg",
        "shadow-sm",
        "transition",
        "duration-150",
        button_style,
        disabled_style,
        class.clone()
    );

    // If this is a submit‐type button, we let the form attribute handle submission,
    // and we don't wire up an onclick
    let onclick_callback = if *button_type == ButtonType::Submit {
        None
    } else {
        onclick.clone()
    };

    html! {
        <button
            class={button_classes}
            type={button_type.as_str()}
            onclick={onclick_callback}
            disabled={*disabled}
            aria-disabled={Some(disabled.to_string())}
            role={role.clone()}
            aria-label={aria_label.clone()}
            // only emit `form="…"`, if Some
            form={form.clone()}
        >
            { for children.iter() }
        </button>
    }
}
