use crate::form::{Label, join_aria_ids, submitted_name};
use crate::form_deserializer::{de_attr, de_classes, de_option_attr};
use serde::Deserialize;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone, Default, Deserialize)]
pub struct ColorInputProps {
    /// JSON string → AttrValue
    #[prop_or_default]
    #[serde(default, deserialize_with = "de_attr")]
    pub id: AttrValue,

    /// Submitted form field name; defaults to id.
    #[prop_or_default]
    #[serde(default, deserialize_with = "de_option_attr")]
    pub name: Option<AttrValue>,

    /// JSON string → AttrValue
    #[prop_or_default]
    #[serde(default, deserialize_with = "de_attr")]
    pub label: AttrValue,

    /// JSON string → AttrValue (defaults to "#000000")
    #[prop_or("#000000".into())]
    #[serde(default, deserialize_with = "de_attr")]
    pub value: AttrValue,

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

    /// skipped during deserialization
    #[prop_or_default]
    #[serde(skip)]
    pub on_change: Option<Callback<String>>,

    #[prop_or_default]
    #[serde(skip)]
    pub on_blur: Option<Callback<FocusEvent>>,

    /// can be either `"foo bar"` or `["foo","bar"]`
    #[prop_or_default]
    #[serde(default, deserialize_with = "de_classes")]
    pub class: Classes,
}

#[component(ColorInput)]
pub fn color_input(props: &ColorInputProps) -> Html {
    let ColorInputProps {
        id,
        name,
        label,
        value,
        helper_text,
        error,
        visually_hidden_label,
        aria_invalid,
        aria_describedby,
        required,
        disabled,
        on_change,
        on_blur,
        class,
    } = props.clone();

    let color = use_state(|| value.clone());

    let handle_input = {
        let color = color.clone();
        let on_change = on_change.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let new_color = input.value();
            color.set(new_color.clone().into());
            if let Some(cb) = &on_change {
                cb.emit(new_color);
            }
        })
    };

    let effective_error = error
        .clone()
        .map(|error| error.to_string())
        .filter(|error| !error.is_empty());
    let helper_id = helper_text
        .as_ref()
        .filter(|_| !id.is_empty())
        .map(|_| AttrValue::from(format!("{id}-helper")));
    let error_id = effective_error
        .as_ref()
        .filter(|_| !id.is_empty())
        .map(|_| AttrValue::from(format!("{id}-error")));
    let describedby = join_aria_ids(vec![aria_describedby, helper_id.clone(), error_id.clone()]);
    let effective_aria_invalid = aria_invalid.unwrap_or(effective_error.is_some());
    let name_attr = submitted_name(&id, &name);

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
        "dark:bg-gray-800",
        "dark:border-gray-600",
        "dark:text-gray-200",
        "disabled:cursor-not-allowed",
        "disabled:opacity-60",
        effective_error.is_some().then_some("border-red-500"),
        class.clone(),
    );

    let preview_classes = classes!(
        "w-10",
        "h-10",
        "rounded-full",
        "border",
        "shadow-sm",
        "transition",
        "duration-150",
        "border-gray-300",
        "dark:border-gray-600",
    );

    let description_classes = classes!("text-gray-600", "dark:text-gray-400");

    html! {
        <div class="flex flex-col space-y-2">
            <Label
                for_id={id.clone()}
                text={label.clone()}
                required={required}
                class={classes!(visually_hidden_label.then_some("sr-only"))}
            />
            <div class="flex items-center space-x-4">
                <input
                    id={id.clone()}
                    name={name_attr}
                    type="color"
                    value={(*color).clone().to_string()}
                    class={input_classes}
                    oninput={handle_input}
                    onblur={on_blur}
                    required={required}
                    disabled={disabled}
                    aria-label={label.clone()}
                    aria-invalid={AttrValue::from(effective_aria_invalid.to_string())}
                    aria-describedby={describedby}
                />
                <span
                    class={preview_classes}
                    style={format!("background-color: {};", *color)}
                />
                <p class={description_classes}>
                    { format!("Selected color: {}", *color) }
                </p>
            </div>
            if let Some(helper_text) = helper_text {
                <p id={helper_id} class="text-sm text-gray-500 dark:text-gray-400">
                    { helper_text }
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
