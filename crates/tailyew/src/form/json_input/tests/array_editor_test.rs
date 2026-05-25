use super::super::model::{model_from_value, node_kind_mut_at_path, value_from_model};
use super::super::types::{JsonArrayItemNode, JsonModel, JsonNodeKind, JsonPathSegment};
use super::first_property_mut;
use serde_json::json;
use uuid::Uuid;

#[test]
fn adding_array_item_updates_correct_path() {
    let mut model = model_from_value(&json!({ "tags": [] }));
    let tags_id = first_property_mut(&mut model, "tags").id;
    let path = vec![JsonPathSegment::Property(tags_id)];

    if let Some(JsonNodeKind::Array(items)) = node_kind_mut_at_path(&mut model, &path) {
        items.push(JsonArrayItemNode {
            id: Uuid::new_v4(),
            value: JsonModel {
                id: Uuid::new_v4(),
                kind: JsonNodeKind::String("frontend".into()),
                touched: true,
                expanded: false,
                replaced_kind: None,
            },
        });
    }

    assert_eq!(
        value_from_model(&model).unwrap(),
        json!({ "tags": ["frontend"] })
    );
}
