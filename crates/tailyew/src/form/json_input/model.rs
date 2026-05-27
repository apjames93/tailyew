use super::types::*;
use super::validation::validate_model_report;
use serde_json::{Map, Number, Value};
use uuid::Uuid;
use yew::AttrValue;

pub(super) fn model_from_value(value: &Value) -> JsonModel {
    model_from_value_at_depth(value, 0)
}

fn model_from_value_at_depth(value: &Value, depth: usize) -> JsonModel {
    match value {
        Value::String(value) => JsonModel {
            id: Uuid::new_v4(),
            kind: JsonNodeKind::String(value.clone()),
            touched: false,
            expanded: false,
            replaced_kind: None,
        },
        Value::Number(value) => JsonModel {
            id: Uuid::new_v4(),
            kind: JsonNodeKind::Number {
                raw: value.to_string(),
            },
            touched: false,
            expanded: false,
            replaced_kind: None,
        },
        Value::Bool(value) => JsonModel {
            id: Uuid::new_v4(),
            kind: JsonNodeKind::Boolean(*value),
            touched: false,
            expanded: false,
            replaced_kind: None,
        },
        Value::Null => JsonModel {
            id: Uuid::new_v4(),
            kind: JsonNodeKind::Null,
            touched: false,
            expanded: false,
            replaced_kind: None,
        },
        Value::Object(value) => JsonModel {
            id: Uuid::new_v4(),
            kind: JsonNodeKind::Object(
                value
                    .iter()
                    .map(|(key, value)| JsonPropertyNode {
                        id: Uuid::new_v4(),
                        key: key.clone(),
                        key_touched: false,
                        value: model_from_value_at_depth(value, depth + 1),
                    })
                    .collect(),
            ),
            touched: false,
            expanded: depth <= 1,
            replaced_kind: None,
        },
        Value::Array(value) => JsonModel {
            id: Uuid::new_v4(),
            kind: JsonNodeKind::Array(
                value
                    .iter()
                    .map(|value| JsonArrayItemNode {
                        id: Uuid::new_v4(),
                        value: model_from_value_at_depth(value, depth + 1),
                    })
                    .collect(),
            ),
            touched: false,
            expanded: depth <= 1,
            replaced_kind: None,
        },
    }
}

pub(super) fn value_from_model(model: &JsonModel) -> Result<Value, JsonInputValidity> {
    let config = JsonInputConfig {
        display_buttons: true,
        require_at_least_one: false,
        require_string_values: false,
        empty_string_value_message: AttrValue::from("Enter a value."),
        disable_keys: false,
        disable_values: false,
        placeholder_key: AttrValue::from(DEFAULT_KEY_PLACEHOLDER),
        placeholder_value: AttrValue::from(DEFAULT_VALUE_PLACEHOLDER),
        allowed_types: JsonValueType::all(),
        default_new_type: JsonValueType::String,
        max_depth: None,
        density: JsonInputDensity::Compact,
        path_policies: Vec::new(),
        add_property_label: AttrValue::from("Add property"),
        add_item_label: AttrValue::from("Add item"),
    };
    let report = validate_model_report(model, &config, ValidationVisibility::All, "$".to_owned());
    if !report.validity.is_valid {
        return Err(report.validity);
    }

    Ok(value_from_valid_model(model))
}

fn value_from_valid_model(model: &JsonModel) -> Value {
    match &model.kind {
        JsonNodeKind::String(value) => Value::String(value.clone()),
        JsonNodeKind::Number { raw } => {
            Value::Number(parse_json_number(raw).unwrap_or_else(|| 0.into()))
        }
        JsonNodeKind::Boolean(value) => Value::Bool(*value),
        JsonNodeKind::Null => Value::Null,
        JsonNodeKind::Object(properties) => {
            let mut object = Map::new();
            for property in properties {
                object.insert(
                    property.key.clone(),
                    value_from_valid_model(&property.value),
                );
            }
            Value::Object(object)
        }
        JsonNodeKind::Array(items) => Value::Array(
            items
                .iter()
                .map(|item| value_from_valid_model(&item.value))
                .collect(),
        ),
    }
}

pub(super) fn parse_json_number(raw: &str) -> Option<Number> {
    match serde_json::from_str::<Value>(raw.trim()) {
        Ok(Value::Number(number)) => Some(number),
        _ => None,
    }
}

pub(super) fn node_mut_at_path<'a>(
    root: &'a mut JsonModel,
    path: &[JsonPathSegment],
) -> Option<&'a mut JsonModel> {
    let mut current = root;

    for segment in path {
        match (segment, &mut current.kind) {
            (JsonPathSegment::Property(property_id), JsonNodeKind::Object(properties)) => {
                current = &mut properties
                    .iter_mut()
                    .find(|property| property.id == *property_id)?
                    .value;
            }
            (JsonPathSegment::ArrayItem(item_id), JsonNodeKind::Array(items)) => {
                current = &mut items.iter_mut().find(|item| item.id == *item_id)?.value;
            }
            _ => return None,
        }
    }

    Some(current)
}

pub(super) fn node_kind_mut_at_path<'a>(
    root: &'a mut JsonModel,
    path: &[JsonPathSegment],
) -> Option<&'a mut JsonNodeKind> {
    node_mut_at_path(root, path).map(|node| &mut node.kind)
}

pub(super) fn property_mut_at_path<'a>(
    root: &'a mut JsonModel,
    parent_path: &[JsonPathSegment],
    property_id: Uuid,
) -> Option<&'a mut JsonPropertyNode> {
    match node_kind_mut_at_path(root, parent_path)? {
        JsonNodeKind::Object(properties) => properties
            .iter_mut()
            .find(|property| property.id == property_id),
        _ => None,
    }
}

pub(super) fn convert_node_type(node: &mut JsonModel, next_type: JsonValueType) {
    if value_type_for_node(node) == next_type {
        return;
    }

    let id = node.id;
    let replaced_kind = if !next_type.is_composite() && is_non_empty_composite(&node.kind) {
        Some(node.kind.clone())
    } else {
        None
    };
    let expanded = next_type.is_composite();
    *node = new_model_for_type(next_type, expanded);
    node.id = id;
    node.touched = true;
    node.replaced_kind = replaced_kind;
}

pub(super) fn new_model_for_type(value_type: JsonValueType, expanded: bool) -> JsonModel {
    JsonModel {
        id: Uuid::new_v4(),
        kind: match value_type {
            JsonValueType::String => JsonNodeKind::String(String::new()),
            JsonValueType::Number => JsonNodeKind::Number { raw: "0".into() },
            JsonValueType::Boolean => JsonNodeKind::Boolean(false),
            JsonValueType::Null => JsonNodeKind::Null,
            JsonValueType::Object => JsonNodeKind::Object(Vec::new()),
            JsonValueType::Array => JsonNodeKind::Array(Vec::new()),
        },
        touched: false,
        expanded,
        replaced_kind: None,
    }
}

pub(super) fn restore_replaced_kind(node: &mut JsonModel) {
    if let Some(kind) = node.replaced_kind.take() {
        node.kind = kind;
        node.expanded = true;
        node.touched = true;
    }
}

pub(super) fn value_type_for_node(node: &JsonModel) -> JsonValueType {
    match node.kind {
        JsonNodeKind::String(_) => JsonValueType::String,
        JsonNodeKind::Number { .. } => JsonValueType::Number,
        JsonNodeKind::Boolean(_) => JsonValueType::Boolean,
        JsonNodeKind::Null => JsonValueType::Null,
        JsonNodeKind::Object(_) => JsonValueType::Object,
        JsonNodeKind::Array(_) => JsonValueType::Array,
    }
}

pub(super) fn count_children(node: &JsonModel) -> usize {
    match &node.kind {
        JsonNodeKind::Object(properties) => properties.len(),
        JsonNodeKind::Array(items) => items.len(),
        _ => 0,
    }
}

pub(super) fn summarize_value(node: &JsonModel) -> String {
    match value_type_for_node(node) {
        JsonValueType::Object => format!(
            "Object · {}",
            pluralize(count_children(node), "property", "properties")
        ),
        JsonValueType::Array => {
            format!(
                "Array · {}",
                pluralize(count_children(node), "item", "items")
            )
        }
        JsonValueType::String => "String".to_owned(),
        JsonValueType::Number => "Number".to_owned(),
        JsonValueType::Boolean => "Boolean".to_owned(),
        JsonValueType::Null => "Null".to_owned(),
    }
}

pub(super) fn pluralize(count: usize, singular: &str, plural: &str) -> String {
    if count == 1 {
        format!("1 {singular}")
    } else {
        format!("{count} {plural}")
    }
}

pub(super) fn append_property_path(
    path: &[JsonPathSegment],
    property_id: Uuid,
) -> Vec<JsonPathSegment> {
    let mut next_path = path.to_vec();
    next_path.push(JsonPathSegment::Property(property_id));
    next_path
}

pub(super) fn append_array_path(path: &[JsonPathSegment], item_id: Uuid) -> Vec<JsonPathSegment> {
    let mut next_path = path.to_vec();
    next_path.push(JsonPathSegment::ArrayItem(item_id));
    next_path
}

pub(super) fn property_path(parent: &str, key: &str) -> String {
    let segment = if key.trim().is_empty() {
        "(empty key)"
    } else {
        key
    };

    if parent == "$" {
        segment.to_owned()
    } else {
        format!("{parent}.{segment}")
    }
}

pub(super) fn array_path(parent: &str, index: usize) -> String {
    if parent == "$" {
        format!("[{index}]")
    } else {
        format!("{parent}[{index}]")
    }
}

pub(super) fn display_key(key: &str) -> String {
    if key.trim().is_empty() {
        "unnamed property".to_owned()
    } else {
        key.to_owned()
    }
}

pub(super) fn is_at_max_depth(config: &JsonInputConfig, depth: usize) -> bool {
    config.max_depth.is_some_and(|max_depth| depth >= max_depth)
}

pub(super) fn is_non_empty_composite(kind: &JsonNodeKind) -> bool {
    match kind {
        JsonNodeKind::Object(properties) => !properties.is_empty(),
        JsonNodeKind::Array(items) => !items.is_empty(),
        _ => false,
    }
}

pub(super) fn duplicate_properties(properties: &[JsonPropertyNode]) -> Vec<&JsonPropertyNode> {
    properties
        .iter()
        .filter(|property| {
            !property.key.trim().is_empty()
                && properties
                    .iter()
                    .filter(|candidate| candidate.key == property.key)
                    .count()
                    > 1
        })
        .collect()
}

pub(super) fn is_key_error(kind: &JsonInputErrorKind) -> bool {
    matches!(
        kind,
        JsonInputErrorKind::EmptyKey | JsonInputErrorKind::DuplicateKey
    )
}
