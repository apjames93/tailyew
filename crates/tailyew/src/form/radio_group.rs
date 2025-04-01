use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct RadioGroupProps {
    pub id: String,                     // Unique ID for the radio group
    pub label: String,                  // Label for the radio group
    pub options: Vec<(String, String)>, // Vec of (value, label) pairs for the radio options
    #[prop_or_default]
    pub default_value: String, // Initial selected value
    #[prop_or_default]
    pub class: Option<String>, // Optional custom class for additional styling
}

#[function_component(RadioGroup)]
pub fn radio_group(props: &RadioGroupProps) -> Html {
    let RadioGroupProps {
        id,
        label,
        options,
        default_value,
        class,
    } = props;

    // Internal state to manage the selected value
    let selected_value = use_state(|| default_value.clone());

    // Callback for handling radio button change
    let onchange = {
        let selected_value = selected_value.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let new_value = input.value();
            selected_value.set(new_value.clone()); // Update internal state

            // Log the selected value
            web_sys::console::log_1(&format!("Selected radio value: {}", new_value).into());
        })
    };

    // Base classes for the radio group container
    let container_classes = classes!(
        "flex",
        "flex-col",
        "space-y-4",
        class.clone() // Include any custom class prop if provided
    );

    // Base classes for the label text
    let label_classes = classes!(
        "text-lg",
        "font-semibold",
        "text-gray-700",
        "dark:text-gray-300",
    );

    // Base classes for the individual radio button container
    let radio_item_classes = classes!("flex", "items-center", "space-x-2",);

    // Base classes for the radio button input
    let radio_input_classes = classes!(
        "h-4",
        "w-4",
        "text-primary", // Custom primary color for the radio button
        "border-gray-300",
        "focus:ring-2",
        "focus:ring-primary",
        "dark:text-primary-dark", // Dark mode styles
        "dark:border-gray-600",
        "dark:focus:ring-primary-dark", // Focus ring colors for dark mode
    );

    // Base classes for the option labels
    let option_label_classes = classes!("text-gray-700", "dark:text-gray-400",);

    html! {
        <div class={container_classes}>
            <label class={label_classes}>{label}</label>
            <div class="flex flex-col space-y-2">
                { for options.iter().map(|(value, option_label)| {
                    let onchange = onchange.clone();
                    html! {
                        <div class={radio_item_classes.clone()}>
                            <input
                                type="radio"
                                id={format!("{}-{}", id, value)}
                                name={id.clone()}
                                value={value.clone()}
                                checked={*selected_value == *value}
                                aria-checked={(*selected_value == *value).to_string()}  // Convert boolean to string
                                onchange={onchange.clone()}
                                class={radio_input_classes.clone()} // Apply dynamic styles
                            />
                            <label for={format!("{}-{}", id, value)} class={option_label_classes.clone()}>
                                { option_label }
                            </label>
                        </div>
                    }
                }) }
            </div>
        </div>
    }
}
