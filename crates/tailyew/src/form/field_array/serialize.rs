use super::model::{FieldArrayFieldValue, FieldArrayObjectRow};
use super::object_field::{FieldArrayObjectField, default_value_for_field, field_key};
use super::props::FieldArrayDeleteConfig;
use super::validation::{message_for_kind, validate_object_rows_with_delete};
use crate::form::{JsonInputError, JsonInputValidity};
use serde_json::Value;

#[cfg(test)]
pub(crate) fn value_from_object_rows(
    rows: &[FieldArrayObjectRow],
    fields: &[FieldArrayObjectField],
) -> Result<Value, JsonInputValidity> {
    value_from_object_rows_with_delete(rows, fields, &FieldArrayDeleteConfig::remove())
}

pub(crate) fn value_from_object_rows_with_delete(
    rows: &[FieldArrayObjectRow],
    fields: &[FieldArrayObjectField],
    delete_config: &FieldArrayDeleteConfig,
) -> Result<Value, JsonInputValidity> {
    let report = validate_object_rows_with_delete(rows, fields, None);
    if !report.validity.is_valid {
        return Err(report.validity);
    }

    let mut values = Vec::with_capacity(rows.len());
    for row in rows {
        let mut object = row.unknown_values.clone();
        if let Some(marker_key) = &delete_config.marker_key {
            object.remove(marker_key);
        }

        for field in fields {
            let key = field_key(field);
            let field_value = row.values.get(&key).cloned().unwrap_or_else(|| {
                FieldArrayFieldValue::from_value(&default_value_for_field(field), field)
            });
            let value = match field_value.to_value() {
                Ok(value) => value,
                Err(_) if row.deleted => continue,
                Err(kind) => {
                    return Err(JsonInputValidity {
                        is_valid: false,
                        errors: vec![JsonInputError {
                            path: key.clone(),
                            message: message_for_kind(&kind).to_owned(),
                            kind,
                        }],
                    });
                }
            };
            object.insert(key, value);
        }

        if row.deleted
            && let Some(marker_key) = &delete_config.marker_key
        {
            object.insert(marker_key.clone(), Value::Bool(true));
        }

        values.push(Value::Object(object));
    }

    Ok(Value::Array(values))
}
