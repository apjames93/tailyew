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
    #[serde(default, deserialize_with = "de_attr")]
    pub label: AttrValue,

    #[prop_or_default]
    #[serde(default, deserialize_with = "de_attr")]
    pub placeholder: AttrValue,

    #[prop_or_default]
    #[serde(default, deserialize_with = "de_attr")]
    pub default_value: AttrValue,

    #[prop_or_default]
    #[serde(default)]
    pub input_type: InputType,

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

    // Cannot deserialize these from JSON blobs:
    #[prop_or_default]
    #[serde(skip)]
    pub on_change: Option<Callback<String>>,

    #[prop_or_default]
    #[serde(skip)]
    pub on_focus: Option<Callback<FocusEvent>>,

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
        input_type,
        default_value,
        min,
        max,
        error_title,
        required,
        class,
        on_change,
        on_focus,
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
    let form_pattern = use_state(|| Some(String::from(".*"))); // fallback to valid

    let oninput = {
        let value = value.clone();
        let validation_error = validation_error.clone();
        let on_change = on_change.clone();
        let pattern = pattern.clone();
        let error_title = error_title.clone();
        let form_pattern = form_pattern.clone();
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
                    form_pattern.set(Some("^$a".into())); // always fail
                } else {
                    validation_error.set(None);
                    form_pattern.set(Some(".*".into()));
                }
                return;
            }

            // Otherwise fallback to pattern-based validation
            match pattern.as_ref().map(|p| Regex::new(p.as_str())) {
                Some(Ok(re)) => {
                    if re.is_match(&new_val) {
                        validation_error.set(None);
                        form_pattern.set(Some(".*".into()));
                    } else {
                        validation_error.set(Some(
                            error_title
                                .clone()
                                .unwrap_or_else(|| "Invalid format.".into())
                                .to_string(),
                        ));
                        form_pattern.set(Some("^$a".into()));
                    }
                }
                Some(Err(err)) => {
                    validation_error.set(Some(format!("Invalid regex: {}", err)));
                    form_pattern.set(Some("^$a".into()));
                }
                None => {
                    validation_error.set(None);
                    form_pattern.set(Some(".*".into()));
                }
            }
        })
    };

    let input_classes = classes!(
        "w-full",
        "px-4",
        "py-2",
        "border",
        "border-gray-300",
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

    let label_classes = classes!(
        "mb-2",
        "text-lg",
        "font-semibold",
        "text-gray-700",
        "dark:text-gray-300"
    );

    let title = validation_error
        .as_ref()
        .cloned()
        .or_else(|| error_title.map(|s| s.to_string()))
        .unwrap_or_default();
    let title_attr = (!title.is_empty()).then_some(title.clone());

    // a11y fallbacks
    let effective_aria_label = aria_label.or_else(|| (!label.is_empty()).then_some(label.clone()));
    let effective_aria_labelledby =
        aria_labelledby.or_else(|| (!id.is_empty()).then_some(id.clone()));
    let effective_aria_describedby =
        aria_describedby.or_else(|| (!title.is_empty()).then_some(id.clone()));

    let id_attr = (!id.is_empty()).then_some(id.clone());

    let input_element = html! {
        <input
            id={id_attr.clone()}
            name={id_attr.clone()}
            type={input_type.to_string()}
            value={(*value).clone()}
            placeholder={placeholder}
            class={input_classes}
            oninput={oninput}
            min={min}
            max={max}
            title={title_attr}
            required={required}
            disabled={disabled}
            pattern={form_pattern.as_ref().cloned().unwrap_or_else(|| ".*".to_string())}
            autocomplete={autocomplete}
            aria-invalid={AttrValue::from(validation_error.is_some().to_string())}
            aria-required={AttrValue::from(required.to_string())}
            aria-describedby={effective_aria_describedby}
            aria-label={effective_aria_label}
            aria-labelledby={effective_aria_labelledby}
            aria-expanded={aria_expanded}
            aria-controls={aria_controls}
            aria-haspopup={aria_haspopup}
            onfocus={on_focus}
            ref={node_ref}
        />
    };

    if input_type == InputType::Hidden {
        html! { input_element }
    } else {
        html! {
            <div class="flex flex-col mb-4">
                <label for={id} class={label_classes}>{ label }</label>
                { input_element }
            </div>
        }
    }
}
