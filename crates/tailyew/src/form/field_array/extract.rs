use serde_json::{Map, Value};
use std::fmt;

/// Parsed object rows from a FieldArray JSON value.
///
/// FieldArray still submits ordinary JSON arrays. This helper gives apps a
/// small, typed extraction layer before they build their own domain structs.
#[derive(Clone, Debug, PartialEq)]
pub struct FieldArrayRows {
    rows: Vec<FieldArrayRow>,
    options: FieldArrayRowsOptions,
}

/// One object row from a FieldArray JSON array.
#[derive(Clone, Debug, PartialEq)]
pub struct FieldArrayRow {
    pub index: usize,
    pub value: Map<String, Value>,
}

/// Options for interpreting FieldArray rows during extraction.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldArrayRowsOptions {
    pub deleted_marker_key: String,
}

impl Default for FieldArrayRowsOptions {
    fn default() -> Self {
        Self {
            deleted_marker_key: "_deleted".to_owned(),
        }
    }
}

/// Errors returned while extracting typed values from FieldArray JSON rows.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FieldArrayRowsError {
    ExpectedArray,
    ExpectedObject {
        index: usize,
    },
    MissingField {
        index: usize,
        key: String,
    },
    InvalidType {
        index: usize,
        key: String,
        expected: &'static str,
    },
    InvalidEnum {
        index: usize,
        key: String,
        allowed: Vec<String>,
    },
}

impl FieldArrayRows {
    pub fn from_value(value: &Value) -> Result<Self, FieldArrayRowsError> {
        Self::from_value_with_options(value, FieldArrayRowsOptions::default())
    }

    pub fn from_value_with_options(
        value: &Value,
        options: FieldArrayRowsOptions,
    ) -> Result<Self, FieldArrayRowsError> {
        let Value::Array(values) = value else {
            return Err(FieldArrayRowsError::ExpectedArray);
        };

        let rows = values
            .iter()
            .enumerate()
            .map(|(index, value)| {
                let Value::Object(object) = value else {
                    return Err(FieldArrayRowsError::ExpectedObject { index });
                };

                Ok(FieldArrayRow {
                    index,
                    value: object.clone(),
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { rows, options })
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &FieldArrayRow> {
        self.rows.iter()
    }

    pub fn into_vec(self) -> Vec<FieldArrayRow> {
        self.rows
    }

    pub fn active(&self) -> impl Iterator<Item = &FieldArrayRow> {
        self.rows
            .iter()
            .filter(|row| !row.is_deleted(&self.options.deleted_marker_key))
    }

    pub fn deleted(&self) -> impl Iterator<Item = &FieldArrayRow> {
        self.rows
            .iter()
            .filter(|row| row.is_deleted(&self.options.deleted_marker_key))
    }
}

impl FieldArrayRow {
    pub fn value(&self) -> &Map<String, Value> {
        &self.value
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.value.get(key)
    }

    pub fn required_string(&self, key: &str) -> Result<String, FieldArrayRowsError> {
        match self.required_value_ref(key)? {
            Value::String(value) => Ok(value.clone()),
            _ => Err(self.invalid_type(key, "a string")),
        }
    }

    pub fn optional_string(&self, key: &str) -> Result<Option<String>, FieldArrayRowsError> {
        match self.get(key) {
            None => Ok(None),
            Some(Value::String(value)) => Ok(Some(value.clone())),
            Some(_) => Err(self.invalid_type(key, "a string")),
        }
    }

    pub fn required_bool(&self, key: &str) -> Result<bool, FieldArrayRowsError> {
        match self.required_value_ref(key)? {
            Value::Bool(value) => Ok(*value),
            _ => Err(self.invalid_type(key, "a boolean")),
        }
    }

    pub fn optional_bool(&self, key: &str) -> Result<Option<bool>, FieldArrayRowsError> {
        match self.get(key) {
            None => Ok(None),
            Some(Value::Bool(value)) => Ok(Some(*value)),
            Some(_) => Err(self.invalid_type(key, "a boolean")),
        }
    }

    pub fn required_i64(&self, key: &str) -> Result<i64, FieldArrayRowsError> {
        match self.required_value_ref(key)? {
            Value::Number(value) => value
                .as_i64()
                .ok_or_else(|| self.invalid_type(key, "an integer")),
            _ => Err(self.invalid_type(key, "an integer")),
        }
    }

    pub fn optional_i64(&self, key: &str) -> Result<Option<i64>, FieldArrayRowsError> {
        match self.get(key) {
            None => Ok(None),
            Some(Value::Number(value)) => value
                .as_i64()
                .map(Some)
                .ok_or_else(|| self.invalid_type(key, "an integer")),
            Some(_) => Err(self.invalid_type(key, "an integer")),
        }
    }

    pub fn required_f64(&self, key: &str) -> Result<f64, FieldArrayRowsError> {
        match self.required_value_ref(key)? {
            Value::Number(value) => value
                .as_f64()
                .ok_or_else(|| self.invalid_type(key, "a number")),
            _ => Err(self.invalid_type(key, "a number")),
        }
    }

    pub fn optional_f64(&self, key: &str) -> Result<Option<f64>, FieldArrayRowsError> {
        match self.get(key) {
            None => Ok(None),
            Some(Value::Number(value)) => value
                .as_f64()
                .map(Some)
                .ok_or_else(|| self.invalid_type(key, "a number")),
            Some(_) => Err(self.invalid_type(key, "a number")),
        }
    }

    pub fn required_value(&self, key: &str) -> Result<Value, FieldArrayRowsError> {
        self.required_value_ref(key).cloned()
    }

    pub fn optional_value(&self, key: &str) -> Option<Value> {
        self.get(key).cloned()
    }

    pub fn required_string_enum(
        &self,
        key: &str,
        allowed: &[&str],
    ) -> Result<String, FieldArrayRowsError> {
        let value = self.required_string(key)?;
        self.validate_string_enum(key, value, allowed)
    }

    pub fn optional_string_enum(
        &self,
        key: &str,
        allowed: &[&str],
    ) -> Result<Option<String>, FieldArrayRowsError> {
        self.optional_string(key)?
            .map(|value| self.validate_string_enum(key, value, allowed))
            .transpose()
    }

    fn required_value_ref(&self, key: &str) -> Result<&Value, FieldArrayRowsError> {
        self.get(key)
            .ok_or_else(|| FieldArrayRowsError::MissingField {
                index: self.index,
                key: key.to_owned(),
            })
    }

    fn validate_string_enum(
        &self,
        key: &str,
        value: String,
        allowed: &[&str],
    ) -> Result<String, FieldArrayRowsError> {
        if allowed.iter().any(|allowed| *allowed == value) {
            Ok(value)
        } else {
            Err(FieldArrayRowsError::InvalidEnum {
                index: self.index,
                key: key.to_owned(),
                allowed: allowed.iter().map(|value| (*value).to_owned()).collect(),
            })
        }
    }

    fn invalid_type(&self, key: &str, expected: &'static str) -> FieldArrayRowsError {
        FieldArrayRowsError::InvalidType {
            index: self.index,
            key: key.to_owned(),
            expected,
        }
    }

    fn is_deleted(&self, deleted_marker_key: &str) -> bool {
        self.value
            .get(deleted_marker_key)
            .and_then(Value::as_bool)
            .unwrap_or(false)
    }
}

impl fmt::Display for FieldArrayRowsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FieldArrayRowsError::ExpectedArray => {
                write!(formatter, "Expected FieldArray value to be a JSON array.")
            }
            FieldArrayRowsError::ExpectedObject { index } => {
                write!(formatter, "Expected row {} to be a JSON object.", index + 1)
            }
            FieldArrayRowsError::MissingField { index, key } => {
                write!(
                    formatter,
                    "Missing required field \"{}\" in row {}.",
                    key,
                    index + 1
                )
            }
            FieldArrayRowsError::InvalidType {
                index,
                key,
                expected,
            } => {
                write!(
                    formatter,
                    "Expected field \"{}\" in row {} to be {}.",
                    key,
                    index + 1,
                    expected
                )
            }
            FieldArrayRowsError::InvalidEnum {
                index,
                key,
                allowed,
            } => {
                write!(
                    formatter,
                    "Expected field \"{}\" in row {} to be one of: {}.",
                    key,
                    index + 1,
                    allowed.join(", ")
                )
            }
        }
    }
}

impl std::error::Error for FieldArrayRowsError {}
