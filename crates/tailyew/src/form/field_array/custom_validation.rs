use super::model::{
    FieldArrayFieldValue, FieldArrayObjectIssue, FieldArrayObjectReport, FieldArrayObjectRow,
    parse_json_number_value,
};
use super::object_field::{FieldArrayObjectField, field_key};
use crate::form::{JsonInputError, JsonInputErrorKind};
use serde_json::{Map, Value};
use std::{collections::BTreeMap, fmt};
use yew::{AttrValue, Callback};

/// Snapshot passed to FieldArray custom validators.
#[derive(Clone, PartialEq, Debug)]
pub struct FieldArrayValidationContext {
    pub value: Value,
    pub rows: Vec<FieldArrayValidationRow>,
}

/// One row in a FieldArray validation context.
#[derive(Clone, PartialEq, Debug)]
pub struct FieldArrayValidationRow {
    pub index: usize,
    pub deleted: bool,
    pub values: BTreeMap<String, Value>,
}

impl FieldArrayValidationRow {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.values.get(key)
    }

    pub fn get_string(&self, key: &str) -> Option<&str> {
        self.get(key).and_then(Value::as_str)
    }

    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.get(key).and_then(Value::as_bool)
    }
}

/// Target location for a custom FieldArray validation issue.
#[derive(Clone, PartialEq, Debug)]
pub enum FieldArrayCustomIssueTarget {
    Root,
    Row { row_index: usize },
    Field { row_index: usize, key: AttrValue },
}

/// Custom validation issue returned by a FieldArray validator.
#[derive(Clone, PartialEq, Debug)]
pub struct FieldArrayCustomIssue {
    pub target: FieldArrayCustomIssueTarget,
    pub message: AttrValue,
    pub kind: JsonInputErrorKind,
}

impl FieldArrayCustomIssue {
    pub fn root(message: impl Into<AttrValue>) -> Self {
        Self {
            target: FieldArrayCustomIssueTarget::Root,
            message: message.into(),
            kind: JsonInputErrorKind::UnsupportedType,
        }
    }

    pub fn row(row_index: usize, message: impl Into<AttrValue>) -> Self {
        Self {
            target: FieldArrayCustomIssueTarget::Row { row_index },
            message: message.into(),
            kind: JsonInputErrorKind::UnsupportedType,
        }
    }

    pub fn field(
        row_index: usize,
        key: impl Into<AttrValue>,
        message: impl Into<AttrValue>,
    ) -> Self {
        Self {
            target: FieldArrayCustomIssueTarget::Field {
                row_index,
                key: key.into(),
            },
            message: message.into(),
            kind: JsonInputErrorKind::UnsupportedType,
        }
    }

    pub fn with_kind(mut self, kind: JsonInputErrorKind) -> Self {
        self.kind = kind;
        self
    }
}

/// Reusable array-level validators for FieldArray object rows.
#[derive(Clone, PartialEq)]
pub enum FieldArrayValidator {
    UniqueField {
        key: AttrValue,
        message: AttrValue,
        trim: bool,
        case_sensitive: bool,
    },
    Custom {
        validate: Callback<FieldArrayValidationContext, Vec<FieldArrayCustomIssue>>,
    },
}

impl fmt::Debug for FieldArrayValidator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UniqueField {
                key,
                message,
                trim,
                case_sensitive,
            } => f
                .debug_struct("UniqueField")
                .field("key", key)
                .field("message", message)
                .field("trim", trim)
                .field("case_sensitive", case_sensitive)
                .finish(),
            Self::Custom { .. } => f.debug_struct("Custom").finish_non_exhaustive(),
        }
    }
}

impl FieldArrayValidator {
    pub fn unique_field(key: impl Into<AttrValue>, message: impl Into<AttrValue>) -> Self {
        Self::UniqueField {
            key: key.into(),
            message: message.into(),
            trim: false,
            case_sensitive: true,
        }
    }

    pub fn unique_field_trimmed(key: impl Into<AttrValue>, message: impl Into<AttrValue>) -> Self {
        Self::UniqueField {
            key: key.into(),
            message: message.into(),
            trim: true,
            case_sensitive: true,
        }
    }

    pub fn custom(
        validate: Callback<FieldArrayValidationContext, Vec<FieldArrayCustomIssue>>,
    ) -> Self {
        Self::Custom { validate }
    }

    pub fn case_sensitive(mut self, value: bool) -> Self {
        if let Self::UniqueField { case_sensitive, .. } = &mut self {
            *case_sensitive = value;
        }
        self
    }
}

pub(crate) fn validation_context_from_object_rows(
    rows: &[FieldArrayObjectRow],
    fields: &[FieldArrayObjectField],
) -> FieldArrayValidationContext {
    let rows = rows
        .iter()
        .enumerate()
        .map(|(index, row)| validation_row_from_object_row(index, row, fields))
        .collect::<Vec<_>>();
    let value = Value::Array(
        rows.iter()
            .map(|row| {
                Value::Object(
                    row.values
                        .iter()
                        .map(|(key, value)| (key.clone(), value.clone()))
                        .collect::<Map<_, _>>(),
                )
            })
            .collect(),
    );

    FieldArrayValidationContext { value, rows }
}

pub(crate) fn merge_custom_issues(
    report: &mut FieldArrayObjectReport,
    custom_issues: Vec<FieldArrayCustomIssue>,
) {
    for issue in custom_issues {
        let message = issue.message.to_string();
        let kind = issue.kind.clone();

        match issue.target {
            FieldArrayCustomIssueTarget::Root => {
                report.validity.errors.push(JsonInputError {
                    path: "$".to_owned(),
                    message,
                    kind,
                });
            }
            FieldArrayCustomIssueTarget::Row { row_index } => {
                report.issues.push(FieldArrayObjectIssue {
                    row_index,
                    key: None,
                    message: message.clone(),
                    kind: kind.clone(),
                });
                report.validity.errors.push(JsonInputError {
                    path: format!("$[{row_index}]"),
                    message,
                    kind,
                });
            }
            FieldArrayCustomIssueTarget::Field { row_index, key } => {
                let key = key.to_string();
                report.issues.push(FieldArrayObjectIssue {
                    row_index,
                    key: Some(key.clone()),
                    message: message.clone(),
                    kind: kind.clone(),
                });
                report.validity.errors.push(JsonInputError {
                    path: format!("$[{row_index}].{key}"),
                    message,
                    kind,
                });
            }
        }
    }

    report.validity.is_valid = report.validity.errors.is_empty();
}

fn validation_row_from_object_row(
    index: usize,
    row: &FieldArrayObjectRow,
    fields: &[FieldArrayObjectField],
) -> FieldArrayValidationRow {
    let mut values = row
        .unknown_values
        .iter()
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect::<BTreeMap<_, _>>();

    for field in fields {
        let key = field_key(field);
        if let Some(field_value) = row.values.get(&key) {
            values.insert(key, validation_value_from_field_value(field_value));
        }
    }

    FieldArrayValidationRow {
        index,
        deleted: row.deleted,
        values,
    }
}

fn validation_value_from_field_value(value: &FieldArrayFieldValue) -> Value {
    match value {
        FieldArrayFieldValue::String(value) => Value::String(value.clone()),
        FieldArrayFieldValue::Number { raw } => {
            parse_json_number_value(raw).unwrap_or_else(|| Value::String(raw.clone()))
        }
        FieldArrayFieldValue::Boolean(value) => Value::Bool(*value),
        FieldArrayFieldValue::Null => Value::Null,
        FieldArrayFieldValue::Json(value) => value.clone(),
    }
}
