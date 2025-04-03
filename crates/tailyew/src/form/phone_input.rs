use regex::Regex;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct PhoneInputProps {
    pub placeholder: String,
    pub label: String,
    pub id: String,

    #[prop_or_default]
    pub default_value: String,

    #[prop_or(Some(r"^\d{3}-\d{3}-\d{4}$".to_string()))]
    pub pattern: Option<String>,

    #[prop_or_default]
    pub class: Classes,
}

#[function_component(PhoneInput)]
pub fn phone_input(props: &PhoneInputProps) -> Html {
    let PhoneInputProps {
        placeholder,
        id,
        default_value,
        label,
        pattern,
        class,
    } = props.clone();

    let phone_number = use_state(|| default_value.clone());
    let error_message = use_state(String::new);

    // Memoize regex
    let regex = use_memo(pattern.clone(), |pattern| {
        Regex::new(pattern.as_deref().unwrap_or(r"^\d{3}-\d{3}-\d{4}$")).ok()
    });

    let oninput = {
        let phone_number = phone_number.clone();
        let error_message = error_message.clone();
        let regex = regex.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            phone_number.set(value.clone());

            if let Some(re) = &*regex {
                if re.is_match(&value) {
                    error_message.set(String::new());
                } else {
                    error_message.set("Invalid format. Expected: xxx-xxx-xxxx".to_string());
                }
            }
        })
    };

    let input_classes = classes!(
        "w-full",
        "px-4",
        "py-2",
        "border",
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
        if error_message.is_empty() {
            "border-gray-300"
        } else {
            "border-red-500 focus:ring-red-500 focus:border-red-500"
        },
        class
    );

    let label_classes = classes!(
        "text-lg",
        "font-semibold",
        "text-gray-700",
        "dark:text-gray-300"
    );

    let error_classes = classes!("text-sm", "text-red-500", "dark:text-red-400");

    html! {
        <div class="flex flex-col space-y-2">
            <label for={id.clone()} class={label_classes}>{ label }</label>
            <input
                id={id.clone()}
                name={id.clone()}
                type="tel"
                placeholder={placeholder}
                value={(*phone_number).clone()}
                pattern={pattern.unwrap_or_default()}
                class={input_classes}
                oninput={oninput}
            />
            {
                if !error_message.is_empty() {
                    html! { <p class={error_classes}>{ (*error_message).clone() }</p> }
                } else {
                    html! {}
                }
            }
        </div>
    }
}
