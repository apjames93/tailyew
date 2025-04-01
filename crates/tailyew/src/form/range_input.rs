use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct RangeInputProps {
    pub id: String,    // Unique ID for the range input
    pub label: String, // Label for the range input
    #[prop_or_default]
    pub default_value: String, // Default value for the range input
    #[prop_or(String::from("0"))]
    pub min: String, // Minimum value for the range
    #[prop_or(String::from("100"))]
    pub max: String, // Maximum value for the range
    #[prop_or(String::from("1"))]
    pub step: String, // Step value for the range
    #[prop_or_default]
    pub class: Option<String>, // Optional custom class for styling
}

#[function_component(RangeInput)]
pub fn range_input(props: &RangeInputProps) -> Html {
    let RangeInputProps {
        id,
        label,
        default_value,
        min,
        max,
        step,
        class,
    } = props;

    // State to track the current value of the range input
    let value = use_state(|| default_value.clone());

    // Callback to handle range input changes
    let oninput = {
        let value = value.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            value.set(input.value());
        })
    };

    // Base classes for the range input
    let range_input_classes = classes!(
        "w-full",
        "h-2",
        "rounded-lg",
        "appearance-none",
        "cursor-pointer",
        "transition",
        "duration-150",
        "bg-gray-200",
        "focus:outline-none",
        "focus:ring-2",
        "focus:ring-green-500",
        "dark:bg-gray-700", // Dark mode styles
        "dark:focus:ring-green-400",
        class.clone() // Include custom class if provided
    );

    // Base classes for the value display
    let value_classes = classes!(
        "text-gray-700",
        "font-medium",
        "dark:text-gray-200", // Dark mode styles
    );

    html! {
        <div class="flex flex-col space-y-2">
            <label for={id.clone()} class="text-lg font-semibold text-gray-700 dark:text-gray-200">{label.clone()}</label>
            <div class="flex items-center space-x-4">
                <input
                    id={id.clone()}
                    name={id.clone()}
                    type="range"
                    value={(*value).clone()}
                    min={min.clone()}
                    max={max.clone()}
                    step={step.clone()}
                    class={range_input_classes} // Apply dynamic styles based on state and theme
                    oninput={oninput.clone()}
                />
                <span class={value_classes}>{(*value).clone()}</span>
            </div>
        </div>
    }
}
