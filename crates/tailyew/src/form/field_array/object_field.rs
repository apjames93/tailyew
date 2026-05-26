use super::custom_validation::FieldArrayValidationRow;
use crate::form::JsonValueType;
use serde_json::{Map, Value};
use std::fmt;
use yew::{AttrValue, Callback};

/// Context passed to one FieldArray object field custom validator.
#[derive(Clone, PartialEq, Debug)]
pub struct FieldArrayFieldValidationContext {
    pub row_index: usize,
    pub key: AttrValue,
    pub label: AttrValue,
    pub value: Value,
    pub row: FieldArrayValidationRow,
}

/// Reusable validators attached to a single FieldArray object field.
#[derive(Clone, PartialEq)]
pub enum FieldArrayFieldValidator {
    RequiredTrimmed {
        message: AttrValue,
    },
    Pattern {
        pattern: AttrValue,
        message: AttrValue,
    },
    Custom {
        validate: Callback<FieldArrayFieldValidationContext, Option<AttrValue>>,
    },
}

impl fmt::Debug for FieldArrayFieldValidator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RequiredTrimmed { message } => f
                .debug_struct("RequiredTrimmed")
                .field("message", message)
                .finish(),
            Self::Pattern { pattern, message } => f
                .debug_struct("Pattern")
                .field("pattern", pattern)
                .field("message", message)
                .finish(),
            Self::Custom { .. } => f.debug_struct("Custom").finish_non_exhaustive(),
        }
    }
}

/// Select option for a string-backed FieldArray object field.
#[derive(Clone, PartialEq, Debug)]
pub struct FieldArraySelectOption {
    pub label: AttrValue,
    pub value: AttrValue,
    pub disabled: bool,
}

impl FieldArraySelectOption {
    pub fn new(label: impl Into<AttrValue>, value: impl Into<AttrValue>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            disabled: false,
        }
    }

    pub fn same(value: impl Into<AttrValue>) -> Self {
        let value = value.into();
        Self {
            label: value.clone(),
            value,
            disabled: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Editor used for a FieldArray object field.
#[derive(Clone, PartialEq, Debug)]
pub enum FieldArrayObjectFieldEditor {
    Auto,
    Select {
        options: Vec<FieldArraySelectOption>,
        placeholder: Option<AttrValue>,
    },
}

/// Definition for one field in FieldArray object-row mode.
#[derive(Clone, PartialEq, Debug)]
pub struct FieldArrayObjectField {
    pub key: AttrValue,
    pub label: AttrValue,
    pub value_type: JsonValueType,
    pub editable: bool,
    pub hidden: bool,
    pub required: bool,
    pub placeholder: Option<AttrValue>,
    pub helper_text: Option<AttrValue>,
    pub default_value: Option<Value>,
    pub editor: FieldArrayObjectFieldEditor,
    pub validators: Vec<FieldArrayFieldValidator>,
}

impl FieldArrayObjectField {
    pub fn string(key: impl Into<AttrValue>, label: impl Into<AttrValue>) -> Self {
        Self::new(key, label, JsonValueType::String)
    }

    pub fn number(key: impl Into<AttrValue>, label: impl Into<AttrValue>) -> Self {
        Self::new(key, label, JsonValueType::Number)
    }

    pub fn boolean(key: impl Into<AttrValue>, label: impl Into<AttrValue>) -> Self {
        Self::new(key, label, JsonValueType::Boolean)
    }

    pub fn null(key: impl Into<AttrValue>, label: impl Into<AttrValue>) -> Self {
        Self::new(key, label, JsonValueType::Null)
    }

    pub fn hidden(key: impl Into<AttrValue>, value_type: JsonValueType) -> Self {
        let key = key.into();
        Self {
            key: key.clone(),
            label: key,
            value_type,
            editable: false,
            hidden: true,
            required: false,
            placeholder: None,
            helper_text: None,
            default_value: None,
            editor: FieldArrayObjectFieldEditor::Auto,
            validators: Vec::new(),
        }
    }

    pub fn select(
        key: impl Into<AttrValue>,
        label: impl Into<AttrValue>,
        options: Vec<FieldArraySelectOption>,
    ) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            value_type: JsonValueType::String,
            editable: true,
            hidden: false,
            required: false,
            placeholder: None,
            helper_text: None,
            default_value: None,
            editor: FieldArrayObjectFieldEditor::Select {
                options,
                placeholder: None,
            },
            validators: Vec::new(),
        }
    }

    pub fn placeholder(mut self, value: impl Into<AttrValue>) -> Self {
        self.placeholder = Some(value.into());
        self
    }

    pub fn helper_text(mut self, value: impl Into<AttrValue>) -> Self {
        self.helper_text = Some(value.into());
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn editable(mut self, editable: bool) -> Self {
        self.editable = editable;
        self
    }

    pub fn hidden_field(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }

    pub fn default_value(mut self, value: Value) -> Self {
        self.default_value = Some(value);
        self
    }

    pub fn select_placeholder(mut self, value: impl Into<AttrValue>) -> Self {
        if let FieldArrayObjectFieldEditor::Select { placeholder, .. } = &mut self.editor {
            *placeholder = Some(value.into());
        }
        self
    }

    pub fn required_trimmed(mut self, message: impl Into<AttrValue>) -> Self {
        self.validators
            .push(FieldArrayFieldValidator::RequiredTrimmed {
                message: message.into(),
            });
        self
    }

    pub fn pattern(mut self, pattern: impl Into<AttrValue>, message: impl Into<AttrValue>) -> Self {
        self.validators.push(FieldArrayFieldValidator::Pattern {
            pattern: pattern.into(),
            message: message.into(),
        });
        self
    }

    pub fn validate_field(
        mut self,
        validate: Callback<FieldArrayFieldValidationContext, Option<AttrValue>>,
    ) -> Self {
        self.validators
            .push(FieldArrayFieldValidator::Custom { validate });
        self
    }

    fn new(
        key: impl Into<AttrValue>,
        label: impl Into<AttrValue>,
        value_type: JsonValueType,
    ) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            value_type,
            editable: true,
            hidden: false,
            required: false,
            placeholder: None,
            helper_text: None,
            default_value: None,
            editor: FieldArrayObjectFieldEditor::Auto,
            validators: Vec::new(),
        }
    }
}

pub(crate) fn default_value_for_type(value_type: JsonValueType) -> Value {
    match value_type {
        JsonValueType::String => Value::String(String::new()),
        JsonValueType::Number => Value::Number(0.into()),
        JsonValueType::Boolean => Value::Bool(false),
        JsonValueType::Null => Value::Null,
        JsonValueType::Object => Value::Object(Map::new()),
        JsonValueType::Array => Value::Array(Vec::new()),
    }
}

pub(crate) fn default_value_for_field(field: &FieldArrayObjectField) -> Value {
    if let Some(default_value) = &field.default_value {
        return default_value.clone();
    }

    match &field.editor {
        FieldArrayObjectFieldEditor::Select { options, .. } => options
            .iter()
            .find(|option| !option.disabled)
            .or_else(|| options.first())
            .map(|option| Value::String(option.value.to_string()))
            .unwrap_or_else(|| Value::String(String::new())),
        FieldArrayObjectFieldEditor::Auto => default_value_for_type(field.value_type),
    }
}

pub(crate) fn select_option_allows_value(field: &FieldArrayObjectField, value: &str) -> bool {
    match &field.editor {
        FieldArrayObjectFieldEditor::Select { options, .. } => options
            .iter()
            .any(|option| !option.disabled && option.value.as_str() == value),
        FieldArrayObjectFieldEditor::Auto => true,
    }
}

pub(crate) fn visible_object_fields(
    fields: &[FieldArrayObjectField],
) -> Vec<FieldArrayObjectField> {
    fields
        .iter()
        .filter(|field| !field.hidden)
        .cloned()
        .collect()
}

pub(crate) fn field_key(field: &FieldArrayObjectField) -> String {
    field.key.to_string()
}
