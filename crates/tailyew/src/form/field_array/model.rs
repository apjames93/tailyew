use super::object_field::{
    FieldArrayObjectField, FieldArrayObjectFieldEditor, default_value_for_field,
    default_value_for_type,
};
use crate::form::{JsonInputErrorKind, JsonValueType};
use serde_json::{Map, Value};
use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(Clone, PartialEq, Debug)]
pub(crate) struct FieldArrayObjectRow {
    pub id: Uuid,
    pub values: BTreeMap<String, FieldArrayFieldValue>,
    pub unknown_values: Map<String, Value>,
    pub touched_fields: Vec<String>,
    pub source_keys: Vec<String>,
    pub deleted: bool,
}

#[derive(Clone, PartialEq, Debug)]
pub(crate) enum FieldArrayFieldValue {
    String(String),
    Number { raw: String },
    Boolean(bool),
    Null,
    Json(Value),
}

#[derive(Clone, PartialEq, Debug)]
pub(crate) struct FieldArrayObjectIssue {
    pub row_index: usize,
    pub key: Option<String>,
    pub message: String,
    pub kind: JsonInputErrorKind,
}

#[derive(Clone, PartialEq, Debug)]
pub(crate) struct FieldArrayObjectReport {
    pub validity: crate::form::JsonInputValidity,
    pub issues: Vec<FieldArrayObjectIssue>,
}

impl FieldArrayObjectRow {
    pub(crate) fn touch(&mut self, key: &str) {
        if !self.touched_fields.iter().any(|field| field == key) {
            self.touched_fields.push(key.to_owned());
        }
    }

    pub(crate) fn is_touched(&self, key: &str) -> bool {
        self.touched_fields.iter().any(|field| field == key)
    }
}

impl FieldArrayFieldValue {
    pub(crate) fn from_value(value: &Value, field: &FieldArrayObjectField) -> Self {
        match field.value_type {
            JsonValueType::String => match value {
                Value::String(value) => Self::String(value.clone()),
                Value::Null => Self::String(default_value_for_field(field).to_string_value()),
                value => Self::String(value_to_string(value, field)),
            },
            JsonValueType::Number => match value {
                Value::Number(number) => Self::Number {
                    raw: number.to_string(),
                },
                Value::String(value) => Self::Number { raw: value.clone() },
                _ => Self::Number {
                    raw: default_value_for_type(JsonValueType::Number).to_string(),
                },
            },
            JsonValueType::Boolean => Self::Boolean(value.as_bool().unwrap_or(false)),
            JsonValueType::Null => Self::Null,
            JsonValueType::Object | JsonValueType::Array => Self::Json(value.clone()),
        }
    }

    pub(crate) fn to_value(&self) -> Result<Value, JsonInputErrorKind> {
        match self {
            Self::String(value) => Ok(Value::String(value.clone())),
            Self::Number { raw } => {
                parse_json_number_value(raw).ok_or(JsonInputErrorKind::InvalidNumber)
            }
            Self::Boolean(value) => Ok(Value::Bool(*value)),
            Self::Null => Ok(Value::Null),
            Self::Json(value) => Ok(value.clone()),
        }
    }

    pub(crate) fn is_empty_string(&self) -> bool {
        matches!(self, Self::String(value) if value.trim().is_empty())
    }

    pub(crate) fn has_identity_value(&self) -> bool {
        match self {
            Self::String(value) => !value.trim().is_empty(),
            Self::Number { raw } => {
                let raw = raw.trim();
                !raw.is_empty() && raw != "0" && parse_json_number_value(raw).is_some()
            }
            Self::Boolean(_) => true,
            Self::Null => false,
            Self::Json(value) => json_value_has_identity(value),
        }
    }
}

fn value_to_string(value: &Value, field: &FieldArrayObjectField) -> String {
    if let Value::String(value) = value {
        return value.clone();
    }

    match &field.editor {
        FieldArrayObjectFieldEditor::Select { .. } => match default_value_for_field(field) {
            Value::String(value) => value,
            _ => String::new(),
        },
        FieldArrayObjectFieldEditor::Auto => value.to_string(),
    }
}

trait StringDefaultValue {
    fn to_string_value(self) -> String;
}

impl StringDefaultValue for Value {
    fn to_string_value(self) -> String {
        match self {
            Value::String(value) => value,
            Value::Null => String::new(),
            value => value.to_string(),
        }
    }
}

pub(crate) fn parse_json_number_value(raw: &str) -> Option<Value> {
    match serde_json::from_str::<Value>(raw.trim()) {
        Ok(Value::Number(number)) => Some(Value::Number(number)),
        _ => None,
    }
}

pub(crate) fn json_value_has_identity(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::String(value) => !value.trim().is_empty(),
        Value::Number(number) => {
            number.as_i64().is_some_and(|value| value != 0)
                || number.as_u64().is_some_and(|value| value != 0)
                || number.as_f64().is_some_and(|value| value != 0.0)
        }
        Value::Bool(_) | Value::Array(_) | Value::Object(_) => true,
    }
}
