use super::key_value_input::KeyValueInput;
use crate::form::{JsonBackedValidationReport, JsonInputValidity, JsonValueType};
use serde_json::Value;
use yew::prelude::*;

#[cfg(test)]
mod tests;

/// Props for [`StaticValuesInput`], a convenience wrapper for static JSON maps.
#[derive(Properties, PartialEq, Clone)]
pub struct StaticValuesInputProps {
    pub id: AttrValue,

    #[prop_or_default]
    pub name: Option<AttrValue>,

    #[prop_or_else(|| AttrValue::from("Static values"))]
    pub label: AttrValue,

    #[prop_or_default]
    pub helper_text: Option<AttrValue>,

    #[prop_or_default]
    pub initial_value: Option<Value>,

    #[prop_or_else(|| AttrValue::from("Variable name"))]
    pub key_placeholder: AttrValue,

    #[prop_or_else(|| AttrValue::from("Value"))]
    pub value_placeholder: AttrValue,

    #[prop_or_else(|| AttrValue::from("Add variable"))]
    pub add_label: AttrValue,

    #[prop_or_default]
    pub allowed_value_types: Option<Vec<JsonValueType>>,

    #[prop_or(JsonValueType::String)]
    pub default_value_type: JsonValueType,

    #[prop_or_default]
    pub require_at_least_one: bool,

    #[prop_or(false)]
    pub require_values: bool,

    #[prop_or_else(|| AttrValue::from("Enter a value."))]
    pub empty_value_message: AttrValue,

    #[prop_or_default]
    pub show_json_preview: bool,

    #[prop_or(true)]
    pub block_form_submit_when_invalid: bool,

    #[prop_or_default]
    pub on_json_change: Option<Callback<Value>>,

    #[prop_or_default]
    pub on_validity_change: Option<Callback<JsonInputValidity>>,

    #[prop_or_default]
    pub on_validation_report_change: Option<Callback<JsonBackedValidationReport>>,
}

/// Edits static/template/config values as a JSON object.
#[component(StaticValuesInput)]
pub fn static_values_input(props: &StaticValuesInputProps) -> Html {
    let allowed_value_types = effective_static_value_types(&props.allowed_value_types);
    let default_value_type =
        effective_default_value_type(props.default_value_type, &allowed_value_types);

    html! {
        <KeyValueInput
            id={props.id.clone()}
            name={props.name.clone()}
            label={props.label.clone()}
            helper_text={props.helper_text.clone()}
            initial_value={props.initial_value.clone()}
            value_type={default_value_type}
            allowed_value_types={Some(allowed_value_types)}
            key_placeholder={props.key_placeholder.clone()}
            value_placeholder={props.value_placeholder.clone()}
            add_label={props.add_label.clone()}
            require_at_least_one={props.require_at_least_one}
            require_values={props.require_values}
            empty_value_message={props.empty_value_message.clone()}
            allow_remove={true}
            show_json_preview={props.show_json_preview}
            block_form_submit_when_invalid={props.block_form_submit_when_invalid}
            on_json_change={props.on_json_change.clone()}
            on_validity_change={props.on_validity_change.clone()}
            on_validation_report_change={props.on_validation_report_change.clone()}
        />
    }
}

pub(crate) fn static_value_types() -> Vec<JsonValueType> {
    vec![
        JsonValueType::String,
        JsonValueType::Number,
        JsonValueType::Boolean,
    ]
}

pub(crate) fn effective_static_value_types(
    allowed_value_types: &Option<Vec<JsonValueType>>,
) -> Vec<JsonValueType> {
    allowed_value_types
        .clone()
        .filter(|types| !types.is_empty())
        .unwrap_or_else(static_value_types)
}

pub(crate) fn effective_default_value_type(
    default_value_type: JsonValueType,
    allowed_types: &[JsonValueType],
) -> JsonValueType {
    if allowed_types.contains(&default_value_type) {
        default_value_type
    } else {
        allowed_types
            .first()
            .copied()
            .unwrap_or(JsonValueType::String)
    }
}
