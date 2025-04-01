use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FileInputProps {
    pub id: String,    // Unique identifier for the file input
    pub label: String, // Label for the file input field
    #[prop_or_default]
    pub default_value: String, // Optional initial value
    #[prop_or_default]
    pub accept: Option<String>, // Optional file types to accept (e.g., "image/*")
    #[prop_or_default]
    pub class: Option<String>, // Optional custom classes for additional styling
    #[prop_or_default]
    pub onchange: Option<Callback<String>>, // Optional callback to emit the selected file name
}

#[function_component(FileInput)]
pub fn file_input(props: &FileInputProps) -> Html {
    let FileInputProps {
        id,
        label,
        default_value,
        accept,
        class,
        onchange,
    } = props;

    // Internal state for managing the selected file name
    let file_name = use_state(|| default_value.clone());

    // Callback to handle file input change
    let onchange_internal = {
        let file_name = file_name.clone();
        let onchange = onchange.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Some(file) = input.files().and_then(|files| files.get(0)) {
                let selected_file_name = file.name();
                file_name.set(selected_file_name.clone());

                // Emit the file name to the parent component if an `onchange` callback is provided
                if let Some(onchange) = onchange.clone() {
                    onchange.emit(selected_file_name);
                }
            } else {
                file_name.set(String::new()); // Clear the name if no file is selected
            }
        })
    };

    // Combine default styles with optional custom class prop
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

    let label_classes = classes!("font-semibold", "text-gray-700", "dark:text-gray-300",);

    let file_name_classes = classes!("text-sm", "text-gray-600", "italic", "dark:text-gray-400",);

    html! {
        <div class="flex flex-col space-y-2">
            <label for={id.clone()} class={label_classes}>
                {label.clone()}
            </label>
            <div class="flex items-center space-x-4">
                <input
                    type="file"
                    id={id.clone()}
                    class={input_classes}  // Apply dynamic styles
                    accept={accept.clone()} // Apply file type filtering if provided
                    onchange={onchange_internal} // Internal change handler
                />
            </div>
            <p class={file_name_classes}>
                { if !(*file_name).is_empty() { format!("Selected file: {}", *file_name) } else { "No file selected".to_string() } }
            </p>
        </div>
    }
}
