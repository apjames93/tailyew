use crate::form::{Label, join_aria_ids, submitted_name};
use crate::form_deserializer::*;
use serde::Deserialize;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone, Default, Deserialize)]
pub struct RangeInputProps {
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
    #[prop_or(String::from("0"))]
    pub min: String,
    #[prop_or(String::from("100"))]
    pub max: String,
    #[prop_or(String::from("1"))]
    pub step: String,
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

#[component(RangeInput)]
pub fn range_input(props: &RangeInputProps) -> Html {
    let RangeInputProps {
        id,
        name,
        label,
        default_value,
        value: controlled_value,
        min,
        max,
        step,
        helper_text,
        error,
        aria_invalid,
        aria_describedby,
        disabled,
        class,
        on_change,
        on_blur,
    } = props.clone();

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
            let input: HtmlInputElement = e.target_unchecked_into();
            let new_value = input.value();
            value.set(new_value.clone().into());
            if let Some(cb) = &on_change {
                cb.emit(new_value);
            }
        })
    };

    let range_input_classes = classes!(
        "w-full",
        "h-2",
        "rounded-lg",
        "appearance-none",
        "cursor-pointer",
        "transition",
        "duration-150",
        "bg-gray-200",
        "focus:outline-none",
        "focus:ring-2",
        "focus:ring-green-500",
        "dark:bg-gray-700",
        "dark:focus:ring-green-400",
        "disabled:cursor-not-allowed",
        "disabled:opacity-60",
        class
    );

    let value_classes = classes!("text-gray-700", "font-medium", "dark:text-gray-200");
    let effective_error = error
        .clone()
        .map(|error| error.to_string())
        .filter(|error| !error.is_empty());
    let value_id = (!id.is_empty()).then(|| AttrValue::from(format!("{id}-value")));
    let helper_id = helper_text
        .as_ref()
        .filter(|_| !id.is_empty())
        .map(|_| AttrValue::from(format!("{id}-helper")));
    let error_id = effective_error
        .as_ref()
        .filter(|_| !id.is_empty())
        .map(|_| AttrValue::from(format!("{id}-error")));
    let describedby = join_aria_ids(vec![
        aria_describedby,
        value_id.clone(),
        helper_id.clone(),
        error_id.clone(),
    ]);
    let effective_aria_invalid = aria_invalid.unwrap_or(effective_error.is_some());
    let name_attr = submitted_name(&id, &name);

    html! {
        <div class="flex flex-col space-y-2">
            <Label for_id={id.clone()} text={label.clone()} />
            <div class="flex items-center space-x-4">
                <input
                    id={id.clone()}
                    name={name_attr}
                    type="range"
                    value={controlled_value.unwrap_or_else(|| (*value).clone())}
                    min={min}
                    max={max}
                    step={step}
                    class={range_input_classes}
                    oninput={oninput}
                    onblur={on_blur}
                    disabled={disabled}
                    aria-invalid={AttrValue::from(effective_aria_invalid.to_string())}
                    aria-describedby={describedby}
                />
                <span id={value_id} class={value_classes}>{ (*value).clone() }</span>
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
