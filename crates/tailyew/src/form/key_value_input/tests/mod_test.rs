use super::super::key_value_input_preset;
use crate::form::form_helpers::json_field_support::normalize_object_initial;
use crate::form::static_values_input::static_value_types;
use crate::form::{
    JsonBackedValidationReport, JsonInputPath, JsonInputPathPolicy, JsonValueType, KeyValueInput,
};
use serde_json::json;
use yew::{Callback, html};

#[test]
fn key_value_input_public_props_accept_string_literals_for_ui_text() {
    let _ = html! {
        <KeyValueInput
            id="metadata_editor"
            name="metadata"
            label="Metadata"
            key_placeholder="Key"
            value_placeholder="Value"
            add_label="Add property"
            require_values={true}
            empty_value_message="Value is required."
            on_validation_report_change={Some(Callback::from(|_: JsonBackedValidationReport| {}))}
        />
    };
}

#[test]
fn key_value_input_normalizes_initial_values() {
    let (object, warning) = normalize_object_initial(Some(json!({"owner": "platform"})));
    assert_eq!(object, json!({"owner": "platform"}));
    assert!(warning.is_none());

    let (object, warning) = normalize_object_initial(None);
    assert_eq!(object, json!({}));
    assert!(warning.is_none());

    let (object, warning) = normalize_object_initial(Some(json!(["not", "object"])));
    assert_eq!(object, json!({}));
    assert!(warning.is_some());
}

#[test]
fn key_value_string_only_preset_hides_type_selector_globally() {
    let preset = key_value_input_preset(
        json!({"owner": "platform"}),
        vec![JsonValueType::String],
        JsonValueType::String,
        true,
    );

    assert_eq!(preset.initial_value, Some(json!({"owner": "platform"})));
    assert_eq!(preset.allowed_types, Some(vec![JsonValueType::String]));
    assert_eq!(preset.default_new_type, JsonValueType::String);
}

#[test]
fn key_value_mixed_scalar_preset_shows_type_selector() {
    let preset = key_value_input_preset(
        json!({"enabled": true, "retry_count": 2}),
        static_value_types(),
        JsonValueType::String,
        true,
    );

    assert_eq!(preset.allowed_types, Some(static_value_types()));
}

#[test]
fn key_value_allow_remove_false_maps_to_root_policy() {
    let preset = key_value_input_preset(
        json!({"owner": "platform"}),
        vec![JsonValueType::String],
        JsonValueType::String,
        false,
    );
    let policies = preset.path_policies.expect("root policies");

    assert_eq!(
        policies,
        vec![
            JsonInputPathPolicy::for_path(JsonInputPath::root())
                .allowed_types(vec![JsonValueType::Object])
                .default_new_type(JsonValueType::Object)
                .type_editable(false)
                .allow_remove_children(false),
        ]
    );
}
