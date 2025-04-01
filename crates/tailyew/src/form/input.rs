use std::fmt;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Debug, PartialEq, Clone, Default)]
pub enum InputType {
    #[default]
    Text,
    Number,
    Password,
    Email,
    Date,
    Time,
    Search,
    Hidden,
}

impl fmt::Display for InputType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            InputType::Text => "text",
            InputType::Number => "number",
            InputType::Password => "password",
            InputType::Email => "email",
            InputType::Date => "date",
            InputType::Time => "time",
            InputType::Search => "search",
            InputType::Hidden => "hidden",
        };
        write!(f, "{}", value)
    }
}

// Properties for the Input component
#[derive(Properties, PartialEq, Clone)]
pub struct InputProps {
    pub placeholder: String,
    pub label: String,
    pub id: String,
    #[prop_or_default]
    pub input_type: InputType,
    #[prop_or_default]
    pub default_value: String,
    #[prop_or_default]
    pub min: Option<String>,
    #[prop_or_default]
    pub max: Option<String>,
    #[prop_or(Some(".*".to_string()))]
    pub pattern: Option<String>, // Regex pattern for input validation
    #[prop_or(false)]
    pub required: bool,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub on_change: Option<Callback<String>>,
    #[prop_or(false)]
    pub disabled: bool,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let InputProps {
        class,
        default_value,
        id,
        input_type,
        label,
        max,
        min,
        pattern,
        placeholder,
        required,
        on_change,
        disabled,
    } = props.clone();

    let value = use_state(|| default_value.clone());

    let oninput = {
        let value = value.clone();
        let on_change = on_change.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let new_value = input.value();

            value.set(new_value.clone());

            if let Some(on_change_callback) = &on_change {
                on_change_callback.emit(new_value);
            }
        })
    };

    let input_classes = classes!(
        "w-full",
        "px-4",
        "py-2",
        "border",
        "border-gray-300",
        "rounded-lg",
        "shadow-sm",
        "transition",
        "duration-150",
        "focus:ring-2",
        "focus:ring-primary",
        "focus:border-primary",
        "dark:bg-gray-800",
        "dark:text-gray-200",
        "dark:border-gray-600",
        "dark:focus:ring-primary-dark",
        "dark:focus:border-primary-dark",
        class.clone(), // Include custom class if provided
    );

    let label_classes = classes!(
        "mb-2",
        "text-lg",
        "font-semibold",
        "text-gray-700",
        "dark:text-gray-300"
    );

    let input_section = html! {
        <input
            class={input_classes}
            id={id.clone()}
            key={id.clone()}
            oninput={oninput}
            placeholder={placeholder.clone()}
            type={input_type.to_string()}
            value={(*value).clone()}
            min={min.clone()}
            max={max.clone()}
            pattern={pattern.clone().unwrap_or_default()}
            required={required}
            disabled={disabled}
        />
    };

    if input_type == InputType::Hidden {
        html! { input_section }
    } else {
        html! {
            <div class="flex flex-col mb-4">
                <label for={id.clone()} class={label_classes}>
                    { label }
                </label>
                { input_section }
            </div>
        }
    }
}
