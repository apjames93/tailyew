use crate::form::{Label, join_aria_ids};
use crate::form_deserializer::*;
use serde::Deserialize;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone, Default, Deserialize)]
pub struct TextareaProps {
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
    pub default_value: AttrValue,

    #[prop_or_default]
    #[serde(default, deserialize_with = "de_option_attr")]
    pub value: Option<AttrValue>,

    #[prop_or_default]
    #[serde(default, deserialize_with = "de_attr")]
    pub placeholder: AttrValue,

    #[prop_or_default]
    #[serde(default, deserialize_with = "de_classes")]
    pub class: Classes,

    #[prop_or_default]
    #[serde(default, deserialize_with = "de_classes")]
    pub container_class: Classes,

    #[prop_or_default]
    #[serde(default, deserialize_with = "de_classes")]
    pub label_class: Classes,

    #[prop_or(false)]
    #[serde(default)]
    pub visually_hidden_label: bool,

    #[prop_or_default]
    #[serde(default, deserialize_with = "de_option_attr")]
    pub helper_text: Option<AttrValue>,

    #[prop_or_default]
    #[serde(default, deserialize_with = "de_option_attr")]
    pub error: Option<AttrValue>,

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

    #[prop_or_default]
    #[serde(default, rename = "aria-label", deserialize_with = "de_option_attr")]
    pub aria_label: Option<AttrValue>,

    #[prop_or(false)]
    #[serde(default)]
    pub required: bool,

    #[prop_or(false)]
    #[serde(default)]
    pub disabled: bool,

    #[prop_or_default]
    #[serde(skip)]
    pub on_change: Option<Callback<String>>,

    #[prop_or_default]
    #[serde(skip)]
    pub on_blur: Option<Callback<FocusEvent>>,

    #[prop_or(5)]
    #[serde(default)]
    pub rows: usize,
}

#[component(Textarea)]
pub fn textarea(props: &TextareaProps) -> Html {
    let TextareaProps {
        id,
        name,
        label,
        default_value,
        value: controlled_value,
        placeholder,
        class,
        container_class,
        label_class,
        visually_hidden_label,
        helper_text,
        error,
        aria_invalid,
        aria_describedby,
        aria_label,
        required,
        disabled,
        on_change,
        on_blur,
        rows,
    } = props;

    let value = use_state(|| default_value.clone());

    {
        let value = value.clone();
        let controlled_value = controlled_value.clone();

        use_effect_with(controlled_value, move |controlled_value| {
            if let Some(controlled_value) = controlled_value
                && *value != *controlled_value
            {
                value.set(controlled_value.clone());
            }
        });
    }

    let oninput = {
        let value = value.clone();
        let on_change = on_change.clone();
        Callback::from(move |e: InputEvent| {
            let textarea: HtmlTextAreaElement = e.target_unchecked_into();
            let val = textarea.value();
            value.set(val.clone().into());
            if let Some(cb) = &on_change {
                cb.emit(val.clone());
            }
        })
    };

    let div_class = classes!("flex", "flex-col", "space-y-2", container_class.clone());
    let name_attr = name.clone().unwrap_or_else(|| id.clone());
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
    let describedby = join_aria_ids(vec![
        aria_describedby.clone(),
        helper_id.clone(),
        error_id.clone(),
    ]);
    let effective_aria_invalid = aria_invalid.unwrap_or(effective_error.is_some());

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
        if effective_error.is_some() {
            "border-red-500"
        } else {
            ""
        },
        "disabled:cursor-not-allowed",
        "disabled:bg-gray-100",
        "disabled:text-gray-500",
        "dark:bg-gray-800",
        "dark:border-gray-600",
        "dark:text-gray-200",
        "dark:focus:ring-green-400",
        "dark:disabled:bg-gray-700",
        class.clone()
    );

    html! {
        <div class={div_class}>
            <Label
                for_id={id.clone()}
                text={label.clone()}
                required={*required}
                class={classes!(visually_hidden_label.then_some("sr-only"), label_class.clone())}
            />
            <textarea
                id={id.clone()}
                name={name_attr}
                placeholder={placeholder.clone()}
                oninput={oninput}
                value={controlled_value.clone().unwrap_or_else(|| (*value).clone())}
                class={textarea_classes}
                rows={format!("{}", rows)}
                required={*required}
                disabled={*disabled}
                onblur={on_blur.clone()}
                aria-label={aria_label.clone()}
                aria-invalid={AttrValue::from(effective_aria_invalid.to_string())}
                aria-describedby={describedby}
            />
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
