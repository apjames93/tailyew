use regex::Regex;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct PhoneInputProps {
    pub placeholder: String, // Placeholder text for the input field
    pub label: String,       // Label for the phone input
    pub id: String,          // Unique identifier for the input and label association
    #[prop_or_default]
    pub default_value: String, // Optional initial value
    #[prop_or(Some(r"^\d{3}-\d{3}-\d{4}$".to_string()))]
    pub pattern: Option<String>, // Optional regex pattern for validation
    #[prop_or_default]
    pub class: Option<String>, // Custom class for additional styling
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
    } = props;

    // State to track the current phone number value and error message
    let phone_number = use_state(|| default_value.clone());
    let error_message: yew::UseStateHandle<String> = use_state(String::new);

    // Use the provided pattern or default to a standard phone format regex
    let phone_regex = Regex::new(pattern.as_ref().unwrap()).unwrap();

    // Callback to handle input changes
    let oninput = {
        let phone_number = phone_number.clone();
        let error_message = error_message.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();

            // Set the phone number value and validate it
            phone_number.set(value.clone());

            // Check if the input matches the provided or default phone number format
            if phone_regex.is_match(&value) {
                error_message.set(String::new()); // Clear error message if valid
            } else {
                error_message
                    .set("Invalid phone number format. Expected: xxx-xxx-xxxx".to_string());
            }
        })
    };

    // CSS classes based on the error state and theme settings
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
        }, // Change border color if there's an error
        class.clone() // Include custom class prop if provided
    );

    let label_classes = classes!(
        "text-lg",
        "font-semibold",
        "text-gray-700",
        "dark:text-gray-300",
    );

    let error_classes = classes!("text-sm", "text-red-500", "dark:text-red-400",);

    html! {
        <div class="flex flex-col space-y-2">
            <label for={id.clone()} class={label_classes}>
                {label.clone()}
            </label>
            <input
                id={id.clone()}
                name={id.clone()}
                type="tel"
                placeholder={placeholder.clone()}
                value={(*phone_number).clone()}
                class={input_classes}  // Apply dynamic styles based on the error state and theme
                pattern={pattern.clone().unwrap_or_default()} // Use the provided or default pattern
                oninput={oninput}
            />

            // Display error message if there is an invalid format
            if !error_message.is_empty() {
                <p class={error_classes}>{(*error_message).clone()}</p>
            }
        </div>
    }
}
