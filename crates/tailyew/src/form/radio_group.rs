use crate::form::{Label, join_aria_ids, submitted_name};
use crate::form_deserializer::{de_attr, de_classes, de_option_attr};
use serde::Deserialize;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone, Default, Deserialize)]
pub struct RadioGroupProps {
    /// the shared name/id for the group
    #[prop_or_default]
    #[serde(default, deserialize_with = "de_attr")]
    pub id: AttrValue,

    /// submitted field name shared by all radio inputs; defaults to id
    #[prop_or_default]
    #[serde(default, deserialize_with = "de_option_attr")]
    pub name: Option<AttrValue>,

    /// optional visual label for the whole group
    #[prop_or_default]
    #[serde(default, deserialize_with = "de_attr")]
    pub label: AttrValue,

    /// list of (value, label) pairs
    #[prop_or_default]
    #[serde(default)]
    pub options: Vec<(String, String)>,

    /// which value is selected by default
    #[prop_or_default]
    #[serde(default, deserialize_with = "de_attr")]
    pub default_value: AttrValue,

    /// controlled selected value
    #[prop_or_default]
    #[serde(default, deserialize_with = "de_option_attr")]
    pub value: Option<AttrValue>,

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
    pub required: bool,

    #[prop_or(false)]
    #[serde(default)]
    pub disabled: bool,

    /// any extra CSS classes to apply to the container
    #[prop_or_default]
    #[serde(default, deserialize_with = "de_classes")]
    pub class: Classes,

    /// programmatic callback on change (not deserialized from JSON)
    #[prop_or_default]
    #[serde(skip)]
    pub on_change: Option<Callback<String>>,

    #[prop_or_default]
    #[serde(skip)]
    pub on_blur: Option<Callback<FocusEvent>>,
}

#[component(RadioGroup)]
pub fn radio_group(props: &RadioGroupProps) -> Html {
    let RadioGroupProps {
        id,
        name,
        label,
        options,
        default_value,
        value: controlled_value,
        helper_text,
        error,
        aria_invalid,
        aria_describedby,
        required,
        disabled,
        class,
        on_change,
        on_blur,
    } = props.clone();

    // state for the selected value
    let selected = use_state(|| default_value.clone());

    {
        let selected = selected.clone();
        let controlled_value = controlled_value.clone();

        use_effect_with(controlled_value, move |controlled_value| {
            if let Some(controlled_value) = controlled_value
                && *selected != *controlled_value
            {
                selected.set(controlled_value.clone());
            }
        });
    }

    // when any radio button changes
    let onchange = {
        let selected = selected.clone();
        let on_change = on_change.clone();
        Callback::from(move |e: Event| {
            let new_val = e.target_unchecked_into::<HtmlInputElement>().value();
            selected.set(new_val.clone().into());
            if let Some(cb) = &on_change {
                cb.emit(new_val);
            }
        })
    };

    // merge your custom classes into the container
    let container_classes = classes!("flex", "flex-col", "space-y-4", class.clone());

    let label_classes = classes!("text-gray-700", "dark:text-gray-300");
    let item_classes = classes!("flex", "items-center", "space-x-2");
    let input_classes = classes!(
        "h-4",
        "w-4",
        "text-primary",
        "border-gray-300",
        "focus:ring-2",
        "focus:ring-primary",
        "dark:text-primary-dark",
        "dark:border-gray-600",
        "dark:focus:ring-primary-dark",
        "disabled:cursor-not-allowed",
        "disabled:opacity-60",
    );
    let text_classes = classes!("text-gray-700", "dark:text-gray-400");
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
    let name_attr = submitted_name(&id, &name);

    html! {
        <div
            class={container_classes}
            role="radiogroup"
            aria-invalid={AttrValue::from(effective_aria_invalid.to_string())}
            aria-describedby={describedby.clone()}
        >
            { // only show a legend/label if non-empty
              if !label.is_empty() {
                html! { <Label text={label.clone()} class={label_classes.clone()} /> }
              } else {
                html!{}
              }
            }

            <div class="flex flex-col space-y-2">
                { for options.iter().map(|(value, text)| {
                    let checked = *selected == *value;
                    let option_id = format!("{}-{}", id, value);
                    html! {
                        <div class={item_classes.clone()}>
                            <input
                                type="radio"
                                id={option_id.clone()}
                                name={name_attr.clone()}
                                value={value.clone()}
                                checked={checked}
                                required={required}
                                disabled={disabled}
                                aria-checked={checked.to_string()}
                                aria-invalid={AttrValue::from(effective_aria_invalid.to_string())}
                                aria-describedby={describedby.clone()}
                                onchange={onchange.clone()}
                                onblur={on_blur.clone()}
                                class={input_classes.clone()}
                            />
                            <Label for_id={option_id} text={text.clone()} class={text_classes.clone()} />
                        </div>
                    }
                }) }
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
