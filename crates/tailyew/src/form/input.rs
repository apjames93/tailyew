use crate::form::{Label, join_aria_ids};
use crate::form_deserializer::*;
use regex::Regex;
use serde::Deserialize;
use std::fmt;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone, Default, Deserialize)]
pub struct InputProps {
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
    pub placeholder: AttrValue,

    #[prop_or_default]
    #[serde(default, deserialize_with = "de_attr")]
    pub default_value: AttrValue,

    #[prop_or_default]
    #[serde(default, deserialize_with = "de_option_attr")]
    pub value: Option<AttrValue>,

    #[prop_or_default]
    #[serde(default)]
    pub input_type: InputType,

    #[prop_or_default]
    #[serde(default)]
    pub size: InputSize,

    #[prop_or_default]
    #[serde(default, deserialize_with = "de_option_attr")]
    pub min: Option<AttrValue>,

    #[prop_or_default]
    #[serde(default, deserialize_with = "de_option_attr")]
    pub max: Option<AttrValue>,

    #[prop_or_default]
    #[serde(default, deserialize_with = "de_option_attr")]
    pub pattern: Option<AttrValue>,

    #[prop_or_default]
    #[serde(default, deserialize_with = "de_option_attr")]
    pub error_title: Option<AttrValue>,

    #[prop_or(false)]
    #[serde(default)]
    pub required: bool,

    #[prop_or_default]
    #[serde(default, deserialize_with = "de_classes")]
    pub class: Classes,

    #[prop_or_default]
    #[serde(default, deserialize_with = "de_classes")]
    pub container_class: Classes,

    #[prop_or(false)]
    #[serde(default)]
    pub marginless: bool,

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

    // Cannot deserialize these from JSON blobs:
    #[prop_or_default]
    #[serde(skip)]
    pub on_change: Option<Callback<String>>,

    #[prop_or_default]
    #[serde(skip)]
    pub on_focus: Option<Callback<FocusEvent>>,

    #[prop_or_default]
    #[serde(skip)]
    pub on_blur: Option<Callback<FocusEvent>>,

    #[prop_or(false)]
    #[serde(default)]
    pub disabled: bool,

    #[prop_or_default]
    #[serde(default, deserialize_with = "de_option_attr")]
    pub autocomplete: Option<AttrValue>,

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

    #[prop_or_default]
    #[serde(
        default,
        rename = "aria-labelledby",
        deserialize_with = "de_option_attr"
    )]
    pub aria_labelledby: Option<AttrValue>,

    #[prop_or_default]
    #[serde(default, rename = "aria-expanded", deserialize_with = "de_option_attr")]
    pub aria_expanded: Option<AttrValue>,

    #[prop_or_default]
    #[serde(default, rename = "aria-controls", deserialize_with = "de_option_attr")]
    pub aria_controls: Option<AttrValue>,

    #[prop_or_default]
    #[serde(default, rename = "aria-haspopup", deserialize_with = "de_option_attr")]
    pub aria_haspopup: Option<AttrValue>,

    // Also skip NodeRef / validation callbacks:
    #[prop_or_default]
    #[serde(skip)]
    pub node_ref: NodeRef,

    #[prop_or_default]
    #[serde(skip)]
    pub validate: Option<Callback<String, Option<String>>>,
}

#[derive(Debug, PartialEq, Clone, Default, Deserialize)]
pub enum InputType {
    #[default]
    Text,
    Number,
    Password,
    Email,
    Date,
    Time,
    Search,
    Hidden,
}

#[derive(Debug, PartialEq, Clone, Default, Deserialize)]
pub enum InputSize {
    Small,
    #[default]
    Medium,
}

impl fmt::Display for InputType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                InputType::Text => "text",
                InputType::Number => "number",
                InputType::Password => "password",
                InputType::Email => "email",
                InputType::Date => "date",
                InputType::Time => "time",
                InputType::Search => "search",
                InputType::Hidden => "hidden",
            }
        )
    }
}

#[component(Input)]
pub fn input(props: &InputProps) -> Html {
    let InputProps {
        placeholder,
        label,
        id,
        name,
        input_type,
        size,
        default_value,
        value: controlled_value,
        min,
        max,
        error_title,
        required,
        class,
        container_class,
        marginless,
        label_class,
        visually_hidden_label,
        helper_text,
        error,
        aria_invalid,
        on_change,
        on_focus,
        on_blur,
        disabled,
        pattern,
        autocomplete,
        aria_describedby,
        aria_label,
        aria_labelledby,
        aria_expanded,
        aria_controls,
        aria_haspopup,
        node_ref,
        validate,
    } = props.clone();

    let value = use_state(|| default_value.to_string());
    let validation_error = use_state(|| None::<String>);

    {
        let value = value.clone();
        let controlled_value = controlled_value.clone();

        use_effect_with(controlled_value, move |controlled_value| {
            if let Some(controlled_value) = controlled_value {
                let next_value = controlled_value.to_string();
                if *value != next_value {
                    value.set(next_value);
                }
            }
        });
    }

    let oninput = {
        let value = value.clone();
        let validation_error = validation_error.clone();
        let on_change = on_change.clone();
        let pattern = pattern.clone();
        let error_title = error_title.clone();
        let validate = validate.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let new_val = input.value();
            value.set(new_val.clone());

            if let Some(cb) = &on_change {
                cb.emit(new_val.clone());
            }

            // Custom validate callback (takes precedence over pattern)
            if let Some(validate_fn) = &validate {
                if let Some(error) = validate_fn.emit(new_val.clone()) {
                    validation_error.set(Some(error));
                } else {
                    validation_error.set(None);
                }
                return;
            }

            // Otherwise fallback to pattern-based validation
            match pattern.as_ref().map(|p| Regex::new(p.as_str())) {
                Some(Ok(re)) => {
                    if re.is_match(&new_val) {
                        validation_error.set(None);
                    } else {
                        validation_error.set(Some(
                            error_title
                                .clone()
                                .unwrap_or_else(|| "Invalid format.".into())
                                .to_string(),
                        ));
                    }
                }
                Some(Err(err)) => {
                    validation_error.set(Some(format!("Invalid regex: {}", err)));
                }
                None => {
                    validation_error.set(None);
                }
            }
        })
    };

    let input_classes = classes!(
        "w-full",
        "box-border",
        "border",
        "border-gray-300",
        "shadow-sm",
        "transition",
        "duration-150",
        "focus:outline-none",
        "focus:ring-2",
        "focus:ring-primary",
        "focus:border-primary",
        "dark:bg-gray-800",
        "dark:text-gray-200",
        "dark:border-gray-600",
        "dark:focus:ring-primary-dark",
        "dark:focus:border-primary-dark",
        match size {
            InputSize::Small => "h-9 rounded-md px-3 py-0 text-sm leading-5",
            InputSize::Medium => "h-10 rounded-lg px-4 py-0 text-sm leading-5",
        },
        class.clone()
    );

    let external_error = error.map(|s| s.to_string()).filter(|s| !s.is_empty());
    let effective_error = external_error
        .clone()
        .or_else(|| validation_error.as_ref().cloned());
    let title = effective_error
        .clone()
        .or_else(|| error_title.map(|s| s.to_string()))
        .unwrap_or_default();
    let title_attr = (!title.is_empty()).then_some(title.clone());

    let helper_id = helper_text
        .as_ref()
        .filter(|_| !id.is_empty())
        .map(|_| AttrValue::from(format!("{id}-helper")));
    let error_id = effective_error
        .as_ref()
        .filter(|_| !id.is_empty())
        .map(|_| AttrValue::from(format!("{id}-error")));

    let describedby = join_aria_ids(vec![aria_describedby, helper_id.clone(), error_id.clone()]);

    // a11y fallbacks
    let effective_aria_label = aria_label.or_else(|| (!label.is_empty()).then_some(label.clone()));
    let effective_aria_labelledby =
        aria_labelledby.or_else(|| (!id.is_empty()).then_some(id.clone()));
    let effective_aria_invalid = aria_invalid.unwrap_or(effective_error.is_some());

    let id_attr = (!id.is_empty()).then_some(id.clone());
    let name_attr = name.or_else(|| id_attr.clone());

    let input_element = html! {
        <input
            id={id_attr.clone()}
            name={name_attr}
            type={input_type.to_string()}
            value={controlled_value.unwrap_or_else(|| AttrValue::from((*value).clone()))}
            placeholder={placeholder}
            class={input_classes}
            oninput={oninput}
            min={min}
            max={max}
            title={title_attr}
            required={required}
            disabled={disabled}
            pattern={pattern}
            autocomplete={autocomplete}
            aria-invalid={AttrValue::from(effective_aria_invalid.to_string())}
            aria-required={AttrValue::from(required.to_string())}
            aria-describedby={describedby}
            aria-label={effective_aria_label}
            aria-labelledby={effective_aria_labelledby}
            aria-expanded={aria_expanded}
            aria-controls={aria_controls}
            aria-haspopup={aria_haspopup}
            onfocus={on_focus}
            onblur={on_blur}
            ref={node_ref}
        />
    };

    if input_type == InputType::Hidden {
        html! { input_element }
    } else {
        html! {
            <div class={classes!("flex", "flex-col", (!marginless).then_some("mb-4"), container_class)}>
                if !label.is_empty() {
                    <Label
                        for_id={id.clone()}
                        text={label.clone()}
                        required={required}
                        class={classes!(
                            "mb-2",
                            visually_hidden_label.then_some("sr-only"),
                            label_class,
                        )}
                    />
                }
                { input_element }
                if let Some(helper_text) = helper_text {
                    <p id={helper_id} class="mt-1 text-xs text-gray-500 dark:text-gray-400">
                        { helper_text }
                    </p>
                }
                if let Some(error) = effective_error {
                    <p id={error_id} class="mt-1 text-xs font-medium text-red-600 dark:text-red-300">
                        { error }
                    </p>
                }
            </div>
        }
    }
}
