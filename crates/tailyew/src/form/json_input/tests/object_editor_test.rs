use super::super::controls::nested_panel_class;
use super::super::model::{
    count_children, model_from_value, node_kind_mut_at_path, value_from_model,
};
use super::super::types::{JsonModel, JsonNodeKind, JsonPathSegment, JsonPropertyNode};
use super::first_property_mut;
use serde_json::json;
use uuid::Uuid;

#[test]
fn deleting_property_removes_it_from_model() {
    let mut model = model_from_value(&json!({ "keep": true, "remove": false }));
    if let JsonNodeKind::Object(properties) = &mut model.kind {
        properties.retain(|property| property.key != "remove");
    }

    assert_eq!(value_from_model(&model).unwrap(), json!({ "keep": true }));
}

#[test]
fn adding_nested_property_updates_correct_path() {
    let mut model = model_from_value(&json!({ "config": {} }));
    let config_id = first_property_mut(&mut model, "config").id;
    let path = vec![JsonPathSegment::Property(config_id)];

    if let Some(JsonNodeKind::Object(properties)) = node_kind_mut_at_path(&mut model, &path) {
        properties.push(JsonPropertyNode {
            id: Uuid::new_v4(),
            key: "timeout".into(),
            key_touched: true,
            value: JsonModel {
                id: Uuid::new_v4(),
                kind: JsonNodeKind::Number { raw: "30".into() },
                touched: true,
                expanded: false,
                replaced_kind: None,
            },
        });
    }

    assert_eq!(
        value_from_model(&model).unwrap(),
        json!({ "config": { "timeout": 30 } })
    );
}

#[test]
fn empty_object_has_no_property_rows() {
    let model = model_from_value(&json!({}));

    assert_eq!(count_children(&model), 0);
}

#[test]
fn composite_expansion_uses_full_width_nested_panel_classes() {
    let class = nested_panel_class(1);

    assert!(class.contains("ml-4"));
    assert!(class.contains("border-l"));
    assert!(class.contains("pl-4"));
    assert!(!class.contains("rounded"));
    assert!(!class.contains("shadow"));
}
