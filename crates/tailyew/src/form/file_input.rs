use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FileInputProps {
    pub id: String,
    pub label: String,
    #[prop_or_default]
    pub initial_file_name: String,
    #[prop_or_default]
    pub accept: Option<String>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub onchange: Option<Callback<String>>,
}

#[function_component(FileInput)]
pub fn file_input(props: &FileInputProps) -> Html {
    let FileInputProps {
        id,
        label,
        initial_file_name,
        accept,
        class,
        onchange,
    } = props;

    let file_name = use_state(|| initial_file_name.clone());

    let onchange_internal = {
        let file_name = file_name.clone();
        let onchange = onchange.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Some(file) = input.files().and_then(|files| files.get(0)) {
                let name = file.name();
                file_name.set(name.clone());
                if let Some(cb) = onchange.clone() {
                    cb.emit(name);
                }
            } else {
                file_name.set(String::new());
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
        class.clone()
    );

    let label_classes = classes!("font-semibold", "text-gray-700", "dark:text-gray-300");

    let file_name_classes = classes!(
        "text-sm",
        "text-gray-600",
        "italic",
        "dark:text-gray-400",
        "truncate"
    );

    html! {
        <div class="flex flex-col space-y-2">
            <label for={id.clone()} class={label_classes}>
                { label }
            </label>
            <input
                id={id.clone()}
                type="file"
                class={input_classes}
                accept={accept.clone()}
                onchange={onchange_internal}
                aria-label={label.to_string()}
                aria-describedby={format!("{id}-file-name")}
            />
            <p id={format!("{id}-file-name")} class={file_name_classes}>
                {
                    if !file_name.is_empty() {
                        format!("Selected file: {}", *file_name)
                    } else {
                        "No file selected".to_string()
                    }
                }
            </p>
        </div>
    }
}
