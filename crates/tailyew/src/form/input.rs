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
    pub pattern: Option<String>,

    #[prop_or(false)]
    pub required: bool,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub on_change: Option<Callback<String>>,

    #[prop_or(false)]
    pub disabled: bool,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let InputProps {
        placeholder,
        label,
        id,
        input_type,
        default_value,
        min,
        max,
        pattern,
        required,
        class,
        on_change,
        disabled,
    } = props.clone();

    let value = use_state(|| default_value.clone());

    let oninput = {
        let value = value.clone();
        let on_change = on_change.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let new_val = input.value();
            value.set(new_val.clone());
            if let Some(cb) = &on_change {
                cb.emit(new_val);
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
        class.clone()
    );

    let label_classes = classes!(
        "mb-2",
        "text-lg",
        "font-semibold",
        "text-gray-700",
        "dark:text-gray-300"
    );

    let input_element = html! {
        <input
            id={id.clone()}
            name={id.clone()}
            type={input_type.to_string()}
            value={(*value).clone()}
            placeholder={placeholder}
            class={input_classes}
            oninput={oninput}
            min={min}
            max={max}
            pattern={pattern.clone().unwrap_or_default()}
            required={required}
            disabled={disabled}
        />
    };

    if input_type == InputType::Hidden {
        html! { input_element }
    } else {
        html! {
            <div class="flex flex-col mb-4">
                <label for={id.clone()} class={label_classes}>{ label }</label>
                { input_element }
            </div>
        }
    }
}
