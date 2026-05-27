use super::super::scalar_editor::{array_len, field_array_preset};
use crate::form::{JsonInputPath, JsonValueType};
use serde_json::json;

#[test]
fn field_array_preset_uses_configured_item_type_and_limits_add_remove() {
    let preset = field_array_preset(
        json!([100, 250, 500]),
        vec![JsonValueType::Number],
        JsonValueType::Number,
        3,
        Some(1),
        Some(3),
        true,
    );
    let policies = preset.path_policies.expect("field array policies");

    assert_eq!(array_len(preset.initial_value.as_ref().unwrap()), 3);
    assert_eq!(preset.default_new_type, JsonValueType::Number);
    assert!(policies.iter().any(|policy| {
        policy.path == JsonInputPath::root()
            && policy.allow_add_children == Some(false)
            && policy.allow_remove_children == Some(true)
    }));
    assert!(policies.iter().any(|policy| {
        policy.path == JsonInputPath::root().any_index()
            && policy.allowed_types == Some(vec![JsonValueType::Number])
            && policy.type_editable == Some(false)
    }));
}

#[test]
fn field_array_min_items_can_prevent_remove() {
    let preset = field_array_preset(
        json!(["one"]),
        vec![JsonValueType::String],
        JsonValueType::String,
        1,
        Some(1),
        None,
        true,
    );
    let policies = preset.path_policies.expect("field array policies");

    assert!(policies.iter().any(|policy| {
        policy.path == JsonInputPath::root() && policy.allow_remove_children == Some(false)
    }));
}
