use super::super::presets::JsonInputPreset;
use crate::form::{JsonInputPath, JsonValueType};

#[test]
fn root_array_preset_locks_item_type_when_single_type_is_allowed() {
    let preset = JsonInputPreset::root_array(JsonValueType::String);
    let policies = preset.path_policies.expect("root array policies");

    assert_eq!(preset.allowed_types, Some(vec![JsonValueType::String]));
    assert_eq!(preset.default_new_type, JsonValueType::String);
    assert!(policies.iter().any(|policy| {
        policy.path == JsonInputPath::root().any_index()
            && policy.type_editable == Some(false)
            && policy.allowed_types == Some(vec![JsonValueType::String])
    }));
}

#[test]
fn string_map_preset_hides_type_selection_and_uses_string() {
    let preset = JsonInputPreset::string_map();

    assert_eq!(preset.allowed_types, Some(vec![JsonValueType::String]));
    assert_eq!(preset.default_new_type, JsonValueType::String);
    assert!(preset.path_policies.is_none());
}

#[test]
fn scalar_map_preset_allows_requested_scalar_types() {
    let preset = JsonInputPreset::scalar_map(
        vec![
            JsonValueType::String,
            JsonValueType::Number,
            JsonValueType::Boolean,
        ],
        JsonValueType::String,
    );

    assert_eq!(
        preset.allowed_types,
        Some(vec![
            JsonValueType::String,
            JsonValueType::Number,
            JsonValueType::Boolean,
        ])
    );
    assert_eq!(preset.default_new_type, JsonValueType::String);
}
