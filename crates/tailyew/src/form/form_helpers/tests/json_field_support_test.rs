use super::super::json_field_support::{normalize_array_initial, normalize_object_initial};
use serde_json::json;

#[test]
fn json_field_support_array_initial_normalization_preserves_arrays() {
    let (array, warning) = normalize_array_initial(Some(json!(["read:users"])));
    assert_eq!(array, json!(["read:users"]));
    assert!(warning.is_none());
}

#[test]
fn json_field_support_array_initial_normalization_defaults_missing_value_to_empty_array() {
    let (array, warning) = normalize_array_initial(None);
    assert_eq!(array, json!([]));
    assert!(warning.is_none());
}

#[test]
fn json_field_support_array_initial_normalization_reports_non_array_values() {
    let (array, warning) = normalize_array_initial(Some(json!({"not": "array"})));
    assert_eq!(array, json!([]));
    assert!(warning.is_some());
}

#[test]
fn json_field_support_object_initial_normalization_preserves_objects() {
    let (object, warning) = normalize_object_initial(Some(json!({"owner": "platform"})));
    assert_eq!(object, json!({"owner": "platform"}));
    assert!(warning.is_none());
}

#[test]
fn json_field_support_object_initial_normalization_defaults_missing_value_to_empty_object() {
    let (object, warning) = normalize_object_initial(None);
    assert_eq!(object, json!({}));
    assert!(warning.is_none());
}

#[test]
fn json_field_support_object_initial_normalization_reports_non_object_values() {
    let (object, warning) = normalize_object_initial(Some(json!(["not", "object"])));
    assert_eq!(object, json!({}));
    assert!(warning.is_some());
}
