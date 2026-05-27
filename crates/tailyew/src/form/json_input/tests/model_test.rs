use super::super::model::{
    convert_node_type, model_from_value, restore_replaced_kind, value_from_model,
};
use super::super::types::{JsonNodeKind, JsonValueType};
use super::first_property_mut;
use serde_json::json;

#[test]
fn initial_flat_object_serializes_without_changing_data() {
    let value = json!({
        "name": "buddy",
        "role": "admin"
    });
    let model = model_from_value(&value);

    assert_eq!(value_from_model(&model).unwrap(), value);
}

#[test]
fn initial_nested_object_preserves_structure() {
    let value = json!({
        "config": {
            "headers": {
                "Accept": "application/json"
            }
        }
    });
    let model = model_from_value(&value);

    assert_eq!(value_from_model(&model).unwrap(), value);
}

#[test]
fn preserves_all_json_value_types() {
    let value = json!({
        "string": "value",
        "number": 42.5,
        "boolean": true,
        "null": null,
        "object": { "nested": "yes" },
        "array": [1, false, null]
    });
    let model = model_from_value(&value);

    assert_eq!(value_from_model(&model).unwrap(), value);
}

#[test]
fn root_array_serializes_without_changing_data() {
    let value = json!([
        "bootstrap",
        3,
        {
            "name": "compile",
            "enabled": true
        }
    ]);
    let model = model_from_value(&value);

    assert_eq!(value_from_model(&model).unwrap(), value);
}

#[test]
fn destructive_type_change_keeps_undo_state_for_non_empty_composite() {
    let mut model = model_from_value(&json!({
        "transport": {
            "timeout_ms": 5000
        }
    }));
    let property = first_property_mut(&mut model, "transport");

    convert_node_type(&mut property.value, JsonValueType::String);

    assert!(matches!(property.value.kind, JsonNodeKind::String(_)));
    assert!(property.value.replaced_kind.is_some());

    restore_replaced_kind(&mut property.value);

    assert!(matches!(property.value.kind, JsonNodeKind::Object(_)));
    assert_eq!(
        value_from_model(&model).unwrap(),
        json!({
            "transport": {
                "timeout_ms": 5000
            }
        })
    );
}
