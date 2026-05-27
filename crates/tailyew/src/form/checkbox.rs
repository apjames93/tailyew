use crate::form::{Label, join_aria_ids};
use crate::form_deserializer::{de_attr, de_option_attr};
use serde::Deserialize;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone, Default, Deserialize)]
pub struct CheckboxProps {
    /// the field’s ID/name
    #[prop_or_default]
    #[serde(default, deserialize_with = "de_attr")]
    pub id: AttrValue,

    /// submitted form field name; defaults to id
    #[prop_or_default]
    #[serde(default, deserialize_with = "de_option_attr")]
    pub name: Option<AttrValue>,

    /// the visible label
    #[prop_or_default]
    #[serde(default, deserialize_with = "de_attr")]
    pub label: AttrValue,

    /// initial checked state
    #[prop_or(false)]
    #[serde(default)]
    pub checked: bool,

    /// mark as required
    #[prop_or(false)]
    #[serde(default)]
    pub required: bool,

    /// optional helper text under the checkbox
    #[prop_or_default]
    #[serde(default, deserialize_with = "de_option_attr")]
    pub description: Option<AttrValue>,

    /// preferred helper text alias; falls back to description when omitted
    #[prop_or_default]
    #[serde(default, deserialize_with = "de_option_attr")]
    pub helper_text: Option<AttrValue>,

    /// external error message under the checkbox
    #[prop_or_default]
    #[serde(default, deserialize_with = "de_option_attr")]
    pub error: Option<AttrValue>,

    /// hide the visible label while preserving it for screen readers
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

    /// accessible label for compact contexts where the visible label is short
    #[prop_or_default]
    #[serde(default, rename = "aria-label", deserialize_with = "de_option_attr")]
    pub aria_label: Option<AttrValue>,

    /// disable interaction
    #[prop_or(false)]
    #[serde(default)]
    pub disabled: bool,

    /// programmatic callback (skipped in serde)
    #[prop_or_default]
    #[serde(skip)]
    pub on_change: Option<Callback<bool>>,

    #[prop_or_default]
    #[serde(skip)]
    pub on_blur: Option<Callback<FocusEvent>>,
}

#[component(Checkbox)]
pub fn checkbox(props: &CheckboxProps) -> Html {
    let CheckboxProps {
        id,
        name,
        label,
        checked,
        required,
        description,
        helper_text,
        error,
        visually_hidden_label,
        aria_invalid,
        aria_describedby,
        aria_label,
        disabled,
        on_change,
        on_blur,
    } = props.clone();

    let handle_change = {
        let on_change = on_change.clone();
        Callback::from(move |e: Event| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            if let Some(cb) = &on_change {
                cb.emit(input.checked());
            }
        })
    };

    // build all your classes as before
    let checkbox_classes = classes!(
        "h-4",
        "w-4",
        "border-2",
        "rounded",
        "focus:ring-2",
        "transition",
        "duration-150",
        "cursor-pointer",
        "outline-none",
        if checked {
            "bg-primary border-primary text-white focus:ring-primary \
             dark:bg-primary-dark dark:border-primary-dark dark:focus:ring-primary-dark"
        } else {
            "bg-white border-gray-300 text-gray-900 focus:ring-primary \
             dark:bg-gray-800 dark:border-gray-600 dark:text-gray-400 dark:focus:ring-primary-dark"
        },
        if disabled {
            "opacity-60 cursor-not-allowed"
        } else {
            ""
        },
    );

    let label_classes = classes!(
        "ml-2",
        "cursor-pointer",
        "transition",
        "duration-150",
        visually_hidden_label.then_some("sr-only"),
        if checked {
            "text-gray-900 dark:text-gray-300"
        } else {
            "text-gray-700 dark:text-gray-400"
        },
        if disabled { "opacity-70" } else { "" },
    );

    let name_attr = name.unwrap_or_else(|| id.clone());
    let helper_text = helper_text.or(description);
    let helper_id = helper_text
        .as_ref()
        .filter(|_| !id.is_empty())
        .map(|_| AttrValue::from(format!("{id}-helper")));
    let effective_error = error
        .clone()
        .map(|error| error.to_string())
        .filter(|error| !error.is_empty());
    let error_id = effective_error
        .as_ref()
        .filter(|_| !id.is_empty())
        .map(|_| AttrValue::from(format!("{id}-error")));
    let describedby = join_aria_ids(vec![aria_describedby, helper_id.clone(), error_id.clone()]);
    let effective_aria_invalid = aria_invalid.unwrap_or(effective_error.is_some());

    html! {
        <div class="flex flex-col space-y-1">
            <div class="flex items-center">
                <input
                    id={id.clone()}
                    name={name_attr}
                    type="checkbox"
                    checked={checked}
                    required={required}
                    disabled={disabled}
                    class={checkbox_classes}
                    onchange={handle_change}
                    onblur={on_blur}
                    aria-label={aria_label}
                    aria-invalid={AttrValue::from(effective_aria_invalid.to_string())}
                    aria-describedby={describedby}
                />
                <Label
                    for_id={id.clone()}
                    text={label.clone()}
                    required={required}
                    class={label_classes}
                />
            </div>
            if let Some(helper_text) = &helper_text {
                <p id={helper_id} class="mt-1 ml-6 text-sm text-gray-500 dark:text-gray-400">
                    { helper_text.clone() }
                </p>
            }
            if let Some(error) = effective_error {
                <p id={error_id} class="mt-1 ml-6 text-sm font-medium text-red-600 dark:text-red-300" role="alert">
                    { error }
                </p>
            }
        </div>
    }
}
