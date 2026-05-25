use crate::form::{Label, join_aria_ids, submitted_name};
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
    #[serde(default, deserialize_with = "de_option_attr")]
    pub name: Option<AttrValue>,

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
    #[serde(default, deserialize_with = "de_option_attr")]
    pub helper_text: Option<AttrValue>,

    #[prop_or_default]
    #[serde(default, deserialize_with = "de_option_attr")]
    pub error: Option<AttrValue>,

    #[prop_or(false)]
    #[serde(default)]
    pub visually_hidden_label: bool,

    #[prop_or_default]
    #[serde(default)]
    pub aria_invalid: Option<bool>,

    #[prop_or_default]
    #[serde(
        default,
        rename = "aria-describedby",
        deserialize_with = "de_option_attr"
    )]
    pub aria_describedby: Option<AttrValue>,

    #[prop_or(false)]
    #[serde(default)]
    pub required: bool,

    #[prop_or(false)]
    #[serde(default)]
    pub disabled: bool,

    #[prop_or_default]
    #[serde(default, deserialize_with = "de_classes")]
    pub class: Classes,

    #[prop_or_default]
    #[serde(skip)]
    pub on_change: Option<Callback<String>>,

    #[prop_or_default]
    #[serde(skip)]
    pub on_blur: Option<Callback<FocusEvent>>,
}

#[component(FileInput)]
pub fn file_input(props: &FileInputProps) -> Html {
    let FileInputProps {
        id,
        name,
        label,
        initial_file_name,
        accept,
        helper_text,
        error,
        visually_hidden_label,
        aria_invalid,
        aria_describedby,
        required,
        disabled,
        class,
        on_change,
        on_blur,
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

    let effective_error = error
        .clone()
        .map(|error| error.to_string())
        .filter(|error| !error.is_empty());
    let file_name_id = (!id.is_empty()).then(|| AttrValue::from(format!("{id}-file-name")));
    let helper_id = helper_text
        .as_ref()
        .filter(|_| !id.is_empty())
        .map(|_| AttrValue::from(format!("{id}-helper")));
    let error_id = effective_error
        .as_ref()
        .filter(|_| !id.is_empty())
        .map(|_| AttrValue::from(format!("{id}-error")));
    let describedby = join_aria_ids(vec![
        aria_describedby.clone(),
        file_name_id.clone(),
        helper_id.clone(),
        error_id.clone(),
    ]);
    let effective_aria_invalid = aria_invalid.unwrap_or(effective_error.is_some());
    let name_attr = submitted_name(id, name);

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
        "disabled:cursor-not-allowed",
        "disabled:bg-gray-100",
        "disabled:text-gray-500",
        "dark:disabled:bg-gray-700",
        effective_error.is_some().then_some("border-red-500"),
        class.clone()
    );

    let file_name_classes = classes!(
        "text-sm",
        "text-gray-600",
        "italic",
        "dark:text-gray-400",
        "truncate"
    );

    html! {
        <div class="flex flex-col space-y-2">
            <Label
                for_id={id.clone()}
                text={label.clone()}
                required={*required}
                class={classes!(visually_hidden_label.then_some("sr-only"))}
            />
            <input
                id={id.clone()}
                name={name_attr}
                type="file"
                class={input_classes}
                accept={accept.clone()}
                onchange={on_change_internal}
                onblur={on_blur.clone()}
                required={*required}
                disabled={*disabled}
                aria-label={label.to_string()}
                aria-invalid={AttrValue::from(effective_aria_invalid.to_string())}
                aria-describedby={describedby}
            />
            <p id={file_name_id} class={file_name_classes}>
                {
                    if !file_name.is_empty() {
                        format!("Selected file: {}", *file_name)
                    } else {
                        "No file selected".to_string()
                    }
                }
            </p>
            if let Some(helper_text) = helper_text {
                <p id={helper_id} class="text-sm text-gray-500 dark:text-gray-400">
                    { helper_text.clone() }
                </p>
            }
            if let Some(error) = effective_error {
                <p id={error_id} class="text-sm font-medium text-red-600 dark:text-red-300" role="alert">
                    { error }
                </p>
            }
        </div>
    }
}
