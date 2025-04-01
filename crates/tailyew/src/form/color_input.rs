use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ColorInputProps {
    pub id: String,    // ID for the input element
    pub label: String, // Label text for the color input
    #[prop_or_default]
    pub default_value: String, // Optional initial value
    #[prop_or_default]
    pub class: Option<String>, // Optional custom classes for additional styling
}

#[function_component(ColorInput)]
pub fn color_input(props: &ColorInputProps) -> Html {
    let ColorInputProps {
        id,
        label,
        default_value,
        class,
    } = props;

    // State to manage the current color value
    let color = use_state(|| default_value.clone());

    // Callback for handling input changes
    let oninput = {
        let color = color.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            color.set(input.value());
        })
    };

    // Base classes for the input element
    let input_classes = classes!(
        "w-16",
        "h-10",
        "p-1",
        "border",
        "rounded-lg",
        "shadow-sm",
        "focus:ring-2",
        "focus:ring-green-500",
        "focus:border-green-500",
        "transition",
        "duration-150",
        "bg-white",
        "dark:bg-gray-800",     // Background color for dark mode
        "dark:border-gray-600", // Border color for dark mode
        "dark:text-gray-200",   // Text color for dark mode
        class.clone()           // Custom class if provided
    );

    // Base classes for the color preview box
    let preview_classes = classes!(
        "w-10",
        "h-10",
        "rounded-full",
        "border",
        "shadow-sm",
        "transition",
        "duration-150",
        "border-gray-300",
        "dark:border-gray-600" // Border color for dark mode
    );

    // Base classes for the label text
    let label_classes = classes!(
        "text-lg",
        "font-semibold",
        "text-gray-700",
        "dark:text-gray-200" // Text color for dark mode
    );

    // Base classes for the description text
    let description_classes = classes!(
        "text-gray-600",
        "dark:text-gray-400" // Text color for dark mode
    );

    html! {
        <div class="flex flex-col space-y-2">
            <label for={id.clone()} class={label_classes}>{label.clone()}</label>
            <div class="flex items-center space-x-4">
                <input
                    id={id.clone()}
                    name={id.clone()}
                    type="color"
                    value={(*color).clone()}
                    class={input_classes} // Apply theme and custom styles
                    oninput={oninput.clone()}
                />
                <span
                    class={preview_classes} // Apply theme styles for the preview box
                    style={format!("background-color: {};", *color)}
                ></span>
                <p class={description_classes}>{format!("Selected color: {}", *color)}</p>
            </div>
        </div>
    }
}
