use super::model::{FieldArrayFieldValue, FieldArrayObjectRow};
use super::object_field::{FieldArrayObjectField, default_value_for_field, field_key};
use super::props::FieldArrayDeleteConfig;
use crate::form::form_helpers::json_field_support::normalize_array_initial;
use serde_json::{Map, Value};
use std::collections::BTreeMap;
use uuid::Uuid;

pub(crate) struct NormalizedObjectRows {
    pub rows: Vec<FieldArrayObjectRow>,
    pub warning: Option<&'static str>,
}

#[cfg(test)]
pub(crate) fn normalize_object_rows_initial(
    value: Option<Value>,
    fields: &[FieldArrayObjectField],
    preserve_unknown_fields: bool,
) -> NormalizedObjectRows {
    normalize_object_rows_initial_with_delete(
        value,
        fields,
        preserve_unknown_fields,
        &FieldArrayDeleteConfig::remove(),
    )
}

pub(crate) fn normalize_object_rows_initial_with_delete(
    value: Option<Value>,
    fields: &[FieldArrayObjectField],
    preserve_unknown_fields: bool,
    delete_config: &FieldArrayDeleteConfig,
) -> NormalizedObjectRows {
    let (array, mut warning) = normalize_array_initial(value);
    let mut non_object_items = false;
    let rows = match array {
        Value::Array(items) => items
            .into_iter()
            .map(|item| match item {
                Value::Object(object) => object_row_from_map_with_delete(
                    object,
                    fields,
                    preserve_unknown_fields,
                    delete_config,
                ),
                _ => {
                    non_object_items = true;
                    new_object_row(fields)
                }
            })
            .collect(),
        _ => Vec::new(),
    };

    if non_object_items {
        warning = Some("Non-object items were normalized using field defaults.");
    }

    NormalizedObjectRows { rows, warning }
}

pub(crate) fn object_row_from_map_with_delete(
    mut object: Map<String, Value>,
    fields: &[FieldArrayObjectField],
    preserve_unknown_fields: bool,
    delete_config: &FieldArrayDeleteConfig,
) -> FieldArrayObjectRow {
    let source_keys = object.keys().cloned().collect::<Vec<_>>();
    let deleted = delete_config
        .marker_key
        .as_ref()
        .and_then(|marker_key| object.remove(marker_key))
        .and_then(|value| value.as_bool())
        .unwrap_or(false);

    let mut values = BTreeMap::new();
    for field in fields {
        let key = field_key(field);
        let value = object
            .remove(&key)
            .unwrap_or_else(|| default_value_for_field(field));
        values.insert(key, FieldArrayFieldValue::from_value(&value, field));
    }

    FieldArrayObjectRow {
        id: Uuid::new_v4(),
        values,
        unknown_values: if preserve_unknown_fields {
            object
        } else {
            Map::new()
        },
        touched_fields: Vec::new(),
        source_keys,
        deleted,
    }
}

pub(crate) fn new_object_row(fields: &[FieldArrayObjectField]) -> FieldArrayObjectRow {
    let mut values = BTreeMap::new();
    for field in fields {
        let value = default_value_for_field(field);
        values.insert(
            field_key(field),
            FieldArrayFieldValue::from_value(&value, field),
        );
    }

    FieldArrayObjectRow {
        id: Uuid::new_v4(),
        values,
        unknown_values: Map::new(),
        touched_fields: Vec::new(),
        source_keys: Vec::new(),
        deleted: false,
    }
}
