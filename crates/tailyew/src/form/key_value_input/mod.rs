use crate::form::form_helpers::json_field_support::{
    helper_with_warning, normalize_object_initial,
};
use crate::form::json_input::presets::{
    JsonInputPreset, default_type_for_allowed, non_empty_types,
};
use crate::form::{
    JsonBackedValidationReport, JsonInput, JsonInputPath, JsonInputPathPolicy, JsonInputValidity,
    JsonValueType,
};
use serde_json::Value;
use yew::prelude::*;

#[cfg(test)]
mod tests;

/// Props for [`KeyValueInput`], a JSON object-map editor.
#[derive(Properties, PartialEq, Clone)]
pub struct KeyValueInputProps {
    pub id: AttrValue,
    #[prop_or_default]
    pub name: Option<AttrValue>,
    pub label: AttrValue,

    #[prop_or_default]
    pub helper_text: Option<AttrValue>,

    #[prop_or_default]
    pub initial_value: Option<Value>,

    #[prop_or(JsonValueType::String)]
    pub value_type: JsonValueType,

    #[prop_or_default]
    pub allowed_value_types: Option<Vec<JsonValueType>>,

    #[prop_or_else(|| AttrValue::from("Property name"))]
    pub key_placeholder: AttrValue,

    #[prop_or_else(|| AttrValue::from("Value"))]
    pub value_placeholder: AttrValue,

    #[prop_or_else(|| AttrValue::from("Add property"))]
    pub add_label: AttrValue,

    #[prop_or(false)]
    pub require_at_least_one: bool,

    #[prop_or(false)]
    pub require_values: bool,

    #[prop_or_else(|| AttrValue::from("Enter a value."))]
    pub empty_value_message: AttrValue,

    #[prop_or(true)]
    pub allow_remove: bool,

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

/// Edits a JSON object as key/value rows and submits the object under `name`.
#[component(KeyValueInput)]
pub fn key_value_input(props: &KeyValueInputProps) -> Html {
    let (normalized_initial, warning) = normalize_object_initial(props.initial_value.clone());
    let allowed_value_types = resolved_value_types(props);
    let default_value_type = default_type_for_allowed(&allowed_value_types, props.value_type);
    let preset = key_value_input_preset(
        normalized_initial,
        allowed_value_types,
        default_value_type,
        props.allow_remove,
    )
    .with_placeholders(
        Some(props.key_placeholder.clone()),
        Some(props.value_placeholder.clone()),
    );

    html! {
        <JsonInput
            id={props.id.clone()}
            name={props.name.clone()}
            label={props.label.clone()}
            helper_text={helper_with_warning(&props.helper_text, warning)}
            initial_value={preset.initial_value}
            allowed_types={preset.allowed_types}
            default_new_type={preset.default_new_type}
            path_policies={preset.path_policies}
            placeholder_key={preset.placeholder_key}
            placeholder_value={preset.placeholder_value}
            add_property_label={props.add_label.clone()}
            display_buttons={true}
            require_at_least_one={props.require_at_least_one}
            require_string_values={props.require_values}
            empty_string_value_message={props.empty_value_message.clone()}
            show_json_preview={props.show_json_preview}
            block_form_submit_when_invalid={props.block_form_submit_when_invalid}
            on_json_change={props.on_json_change.clone()}
            on_validity_change={props.on_validity_change.clone()}
            on_validation_report_change={props.on_validation_report_change.clone()}
        />
    }
}

pub(crate) fn key_value_input_preset(
    initial_value: Value,
    allowed_value_types: Vec<JsonValueType>,
    default_value_type: JsonValueType,
    allow_remove: bool,
) -> JsonInputPreset {
    let allowed_value_types = non_empty_types(allowed_value_types, vec![JsonValueType::String]);
    let default_value_type = default_type_for_allowed(&allowed_value_types, default_value_type);

    let base_preset = if allowed_value_types == vec![JsonValueType::String] {
        JsonInputPreset::string_map()
    } else {
        JsonInputPreset::scalar_map(allowed_value_types, default_value_type)
    };

    base_preset
        .with_initial_value(initial_value)
        .with_path_policies(vec![
            JsonInputPathPolicy::for_path(JsonInputPath::root())
                .allowed_types(vec![JsonValueType::Object])
                .default_new_type(JsonValueType::Object)
                .type_editable(false)
                .allow_remove_children(allow_remove),
        ])
}

pub(crate) fn resolved_value_types(props: &KeyValueInputProps) -> Vec<JsonValueType> {
    props
        .allowed_value_types
        .clone()
        .filter(|types| !types.is_empty())
        .unwrap_or_else(|| vec![props.value_type])
}
