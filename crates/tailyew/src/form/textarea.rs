use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct TextareaProps {
    pub id: String,
    pub label: String,
    #[prop_or_default]
    pub default_value: String,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub container_class: Option<String>,
    #[prop_or(false)]
    pub required: bool,
    #[prop_or_default]
    pub on_change: Option<Callback<String>>,
}

#[function_component(Textarea)]
pub fn textarea(props: &TextareaProps) -> Html {
    let TextareaProps {
        class,
        container_class,
        default_value,
        id,
        label,
        placeholder,
        required,
        on_change,
    } = props.clone();

    // Internal state for managing the textarea value
    let value = use_state(|| default_value.clone());

    // Callback to handle textarea change
    let oninput = {
        let value = value.clone();
        Callback::from(move |e: InputEvent| {
            let textarea: HtmlTextAreaElement = e.target_unchecked_into();
            value.set(textarea.value());

            if let Some(on_change_callback) = &on_change {
                on_change_callback.emit(textarea.value());
            }
        })
    };

    let div_class = if let Some(klass) = container_class {
        classes!(klass.clone())
    } else {
        classes!("flex", "flex-col", "space-y-2")
    };

    let textarea_classes = classes!(
        "px-4",
        "py-2",
        "border",
        "rounded-lg",
        "shadow-sm",
        "transition",
        "duration-150",
        "focus:outline-none",
        "focus:ring-2",
        "focus:ring-green-500",
        "focus:border-green-500",
        "dark:bg-gray-800",          // Background color for dark mode
        "dark:border-gray-600",      // Border color for dark mode
        "dark:text-gray-200",        // Text color for dark mode
        "dark:focus:ring-green-400", // Focus ring color for dark mode
        class.clone()                // Apply custom class if provided
    );

    // Base classes for the label
    let label_classes = classes!(
        "text-lg",
        "font-semibold",
        "text-gray-700",
        "dark:text-gray-200" // Label text color for dark mode
    );

    html! {
        <div class={div_class}>
            <label for={id.clone()} class={label_classes}>{label.clone()}</label>
            <textarea
                id={id.clone()}
                placeholder={placeholder.clone()}
                oninput={oninput}
                value={(*value).clone()}
                class={textarea_classes}
                rows=5
                required={required}
            />
        </div>
    }
}
