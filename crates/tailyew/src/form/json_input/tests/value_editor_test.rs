use super::super::model::{model_from_value, value_from_model};
use super::super::types::JsonNodeKind;
use super::super::value_editor::boolean_display_text;
use super::first_property_mut;
use serde_json::json;

#[test]
fn editing_string_updates_serialized_json() {
    let mut model = model_from_value(&json!({ "name": "old" }));
    first_property_mut(&mut model, "name").value.kind = JsonNodeKind::String("new".into());

    assert_eq!(value_from_model(&model).unwrap(), json!({ "name": "new" }));
}

#[test]
fn editing_number_preserves_number_type_when_valid() {
    let mut model = model_from_value(&json!({ "limit": 1 }));
    first_property_mut(&mut model, "limit").value.kind = JsonNodeKind::Number { raw: "25".into() };

    assert_eq!(value_from_model(&model).unwrap(), json!({ "limit": 25 }));
}

#[test]
fn boolean_display_text_is_compact() {
    assert_eq!(boolean_display_text(true), "true");
    assert_eq!(boolean_display_text(false), "false");
}
