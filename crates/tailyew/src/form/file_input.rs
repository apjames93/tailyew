use crate::form_deserializer::*;
use serde::Deserialize;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone, Default, Deserialize)]
pub struct FileInputProps {
    #[prop_or_default]
    #[serde(default, deserialize_with = "de_attr")]
    pub id: AttrValue,

    #[prop_or_default]
    #[serde(default, deserialize_with = "de_attr")]
    pub label: AttrValue,

    #[prop_or_default]
    #[serde(default, deserialize_with = "de_attr")]
    pub initial_file_name: AttrValue,

    #[prop_or_default]
    #[serde(default, deserialize_with = "de_attr")]
    pub accept: AttrValue,

    #[prop_or_default]
    #[serde(default, deserialize_with = "de_classes")]
    pub class: Classes,

    #[prop_or_default]
    #[serde(skip)]
    pub on_change: Option<Callback<String>>,
}

#[component(FileInput)]
pub fn file_input(props: &FileInputProps) -> Html {
    let FileInputProps {
        id,
        label,
        initial_file_name,
        accept,
        class,
        on_change,
    } = props;

    let file_name = use_state(|| initial_file_name.clone());

    let on_change_internal = {
        let file_name = file_name.clone();
        let on_change = on_change.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Some(file) = input.files().and_then(|files| files.get(0)) {
                let name = file.name();
                file_name.set(name.clone().into());
                if let Some(cb) = on_change.clone() {
                    cb.emit(name);
                }
            } else {
                file_name.set(String::new().into());
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
                onchange={on_change_internal}
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
