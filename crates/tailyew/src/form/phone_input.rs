use crate::form::{Label, join_aria_ids, submitted_name};
use crate::form_deserializer::*;
use regex::Regex;
use serde::Deserialize;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone, Default, Deserialize)]
pub struct PhoneInputProps {
    #[prop_or_default]
    #[serde(default, deserialize_with = "de_attr")]
    pub id: AttrValue,
    #[prop_or_default]
    #[serde(default, deserialize_with = "de_option_attr")]
    pub name: Option<AttrValue>,
    #[prop_or_default]
    #[serde(default, deserialize_with = "de_attr")]
    pub placeholder: AttrValue,
    #[prop_or_default]
    #[serde(default, deserialize_with = "de_attr")]
    pub label: AttrValue,
    #[prop_or_default]
    #[serde(default, deserialize_with = "de_attr")]
    pub default_value: AttrValue,
    #[prop_or_default]
    #[serde(default, deserialize_with = "de_option_attr")]
    pub value: Option<AttrValue>,

    #[prop_or(Some(r"^\d{3}-\d{3}-\d{4}$".to_string()))]
    pub pattern: Option<String>,

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

#[component(PhoneInput)]
pub fn phone_input(props: &PhoneInputProps) -> Html {
    let PhoneInputProps {
        placeholder,
        id,
        name,
        default_value,
        value: controlled_value,
        label,
        pattern,
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
    } = props.clone();

    let phone_number = use_state(|| default_value.clone());
    let error_message = use_state(String::new);

    {
        let phone_number = phone_number.clone();
        let controlled_value = controlled_value.clone();

        use_effect_with(controlled_value, move |controlled_value| {
            if let Some(controlled_value) = controlled_value
                && *phone_number != *controlled_value
            {
                phone_number.set(controlled_value.clone());
            }
        });
    }

    // Memoize regex
    let regex = use_memo(pattern.clone(), |pattern| {
        Regex::new(pattern.as_deref().unwrap_or(r"^\d{3}-\d{3}-\d{4}$")).ok()
    });

    let oninput = {
        let phone_number = phone_number.clone();
        let error_message = error_message.clone();
        let regex = regex.clone();
        let on_change = on_change.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            phone_number.set(value.clone().into());
            if let Some(cb) = &on_change {
                cb.emit(value.clone());
            }

            if let Some(re) = &*regex {
                if re.is_match(&value) {
                    error_message.set(String::new());
                } else {
                    error_message.set("Invalid format. Expected: xxx-xxx-xxxx".to_string());
                }
            }
        })
    };

    let external_error = error
        .clone()
        .map(|error| error.to_string())
        .filter(|error| !error.is_empty());
    let effective_error =
        external_error.or_else(|| (!error_message.is_empty()).then(|| (*error_message).clone()));
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
        if effective_error.is_none() {
            "border-gray-300"
        } else {
            "border-red-500 focus:ring-red-500 focus:border-red-500"
        },
        "disabled:cursor-not-allowed",
        "disabled:bg-gray-100",
        "disabled:text-gray-500",
        "dark:disabled:bg-gray-700",
        class
    );

    let error_classes = classes!(
        "text-sm",
        "font-medium",
        "text-red-600",
        "dark:text-red-300"
    );

    html! {
        <div class="flex flex-col space-y-2">
            <Label
                for_id={id.clone()}
                text={label.clone()}
                required={required}
                class={classes!(visually_hidden_label.then_some("sr-only"))}
            />
            <input
                id={id.clone()}
                name={name_attr}
                type="tel"
                placeholder={placeholder}
                value={controlled_value.unwrap_or_else(|| (*phone_number).clone())}
                pattern={pattern.clone()}
                class={input_classes}
                oninput={oninput}
                onblur={on_blur}
                required={required}
                disabled={disabled}
                aria-invalid={AttrValue::from(effective_aria_invalid.to_string())}
                aria-describedby={describedby}
            />
            if let Some(helper_text) = helper_text {
                <p id={helper_id} class="text-sm text-gray-500 dark:text-gray-400">
                    { helper_text }
                </p>
            }
            {
                if let Some(error) = effective_error {
                    html! { <p id={error_id} class={error_classes} role="alert">{ error }</p> }
                } else {
                    html! {}
                }
            }
        </div>
    }
}
