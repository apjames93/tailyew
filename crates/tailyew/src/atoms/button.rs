use yew::prelude::*;

/// Define the possible HTML tag types for the Button component
#[derive(Clone, Default, PartialEq)]
pub enum ButtonType {
    Primary,
    Secondary,
    Danger,
    Submit,
    #[default]
    Button,
    Icon,
}

impl ButtonType {
    /// Convert ButtonType to its corresponding HTML string representation
    fn as_str(&self) -> &'static str {
        match self {
            ButtonType::Button => "button",
            ButtonType::Danger => "button",
            ButtonType::Primary => "button",
            ButtonType::Secondary => "button",
            ButtonType::Submit => "submit",
            ButtonType::Icon => "button",
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
    pub class: Option<String>,
    #[prop_or_default]
    pub children: Children, // Use Children to accept any HTML content as children
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let ButtonProps {
        button_type,
        onclick,
        disabled,
        class,
        children,
    } = props;

    // Determine the base classes based on the button_type
    let button_style = match button_type {
        ButtonType::Primary => "bg-primary hover:bg-primary-dark text-white font-bold dark:bg-primary-dark dark:hover:bg-primary",
        ButtonType::Secondary => "bg-secondary hover:bg-secondary-dark text-white font-bold dark:bg-secondary-dark dark:hover:bg-secondary",
        ButtonType::Danger => "bg-danger hover:bg-danger-dark text-white font-bold dark:bg-danger-dark dark:hover:bg-danger",
        ButtonType::Submit => "bg-success hover:bg-success-dark text-white font-bold dark:bg-success-dark dark:hover:bg-success",
        ButtonType::Icon => "text-gray-500 dark:text-gray-300 hover:text-gray-700 dark:hover:text-gray-500 transition duration-150",
        ButtonType::Button => "bg-accent hover:bg-accent-dark text-white font-bold dark:bg-accent-dark dark:hover:bg-accent",
    };

    // Determine the disabled state styles
    let disabled_style = if *disabled {
        "bg-neutral text-gray-200 cursor-not-allowed dark:bg-neutral-dark dark:text-gray-400"
    } else {
        "focus:outline-none focus:ring-2 focus:ring-accent focus:ring-offset-2 dark:focus:ring-accent-dark"
    };

    // Combine base styles with additional styles and optional custom class prop
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

    // Clone the onclick callback only if the button is not of type submit
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
        >
            { for children.iter() }
        </button>
    }
}
