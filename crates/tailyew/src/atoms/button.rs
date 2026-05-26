use yew::prelude::*;

/// Define the possible HTML tag types for the Button component
#[derive(Clone, Debug, Default, PartialEq)]
pub enum ButtonType {
    #[default]
    Primary,
    Secondary,
    Danger,
    DangerGhost,
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

#[derive(Clone, Debug, Default, PartialEq)]
pub enum ButtonSize {
    IconSmall,
    IconMedium,
    Small,
    #[default]
    Medium,
    Large,
}

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or(ButtonType::Button)]
    pub button_type: ButtonType,

    #[prop_or_default]
    pub on_click: Option<Callback<MouseEvent>>,

    #[prop_or_default]
    pub on_mouse_down: Option<Callback<MouseEvent>>,

    #[prop_or(false)]
    pub disabled: bool,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub size: ButtonSize,

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

    #[prop_or_default]
    pub title: Option<AttrValue>,
}

#[component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let ButtonProps {
        button_type,
        on_click,
        on_mouse_down,
        disabled,
        class,
        size,
        form,
        children,
        aria_label,
        role,
        aria_expanded,
        aria_controls,
        title,
    } = props;

    // Shared focus ring for non-disabled buttons
    let shared_focus = "focus:outline-none focus:ring-2 focus:ring-accent focus:ring-offset-2 dark:focus:ring-accent-dark";

    // Determine base styles for each button type
    let button_style = match button_type {
        ButtonType::Primary => {
            "bg-primary hover:bg-primary-dark text-white font-bold dark:bg-primary-dark dark:hover:bg-primary"
        }
        ButtonType::Secondary => {
            "bg-secondary hover:bg-secondary-dark text-white font-bold dark:bg-secondary-dark dark:hover:bg-secondary"
        }
        ButtonType::Danger => {
            "bg-danger hover:bg-danger-dark text-white font-bold dark:bg-danger-dark dark:hover:bg-danger"
        }
        ButtonType::DangerGhost => {
            "bg-transparent text-red-600 dark:text-red-300 hover:bg-red-50 dark:hover:bg-red-950 border border-transparent hover:border-red-200 dark:hover:border-red-900 font-medium"
        }
        ButtonType::Submit => {
            "bg-success hover:bg-success-dark text-white font-bold dark:bg-success-dark dark:hover:bg-success"
        }
        ButtonType::Ghost => {
            "bg-transparent text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-800 border border-transparent hover:border-gray-300 dark:hover:border-gray-600"
        }
        ButtonType::Icon => {
            "text-gray-500 dark:text-gray-300 hover:text-gray-700 dark:hover:text-gray-500 transition duration-150 border-none"
        }
        ButtonType::Button => {
            "bg-accent hover:bg-accent-dark text-white font-bold dark:bg-accent-dark dark:hover:bg-accent"
        }
    };

    // Disabled state overrides
    let disabled_style = if *disabled {
        "bg-neutral text-gray-200 cursor-not-allowed dark:bg-neutral-dark dark:text-gray-400"
    } else {
        shared_focus
    };

    let size_style = match size {
        ButtonSize::IconSmall => "h-9 w-9 p-0 text-sm rounded-md",
        ButtonSize::IconMedium => "h-10 w-10 p-0 text-sm rounded-md",
        ButtonSize::Small => "py-1.5 px-3 text-sm rounded-md",
        ButtonSize::Medium => "py-2 px-4 rounded-lg",
        ButtonSize::Large => "py-3 px-5 text-base rounded-lg",
    };

    // Combine final class list
    let button_classes = classes!(
        "shadow-sm",
        "transition",
        "duration-150",
        size_style,
        button_style,
        disabled_style,
        class.clone()
    );

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
            onmousedown={on_mouse_down.clone()}
            disabled={*disabled}
            aria-disabled={Some(disabled.to_string())}
            role={role.clone()}
            aria-label={aria_label.clone()}
            aria-expanded={aria_expanded.clone()}
            aria-controls={aria_controls.clone()}
            title={title.clone()}
            // only emit `form="…"`, if Some
            form={form.clone()}
    >
            { for children.iter() }
        </button>
    }
}
