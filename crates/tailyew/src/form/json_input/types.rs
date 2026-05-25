use serde_json::Value;
use std::fmt;
use uuid::Uuid;
use yew::prelude::*;

use super::validation_report::JsonBackedValidationReport;

pub(super) type ModelUpdater = Box<dyn FnOnce(&mut JsonModel)>;

pub(super) const DEFAULT_KEY_PLACEHOLDER: &str = "Property name";
pub(super) const DEFAULT_VALUE_PLACEHOLDER: &str = "Value";

/// JSON value kinds that can be edited by JsonInput and JSON-backed wrappers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum JsonValueType {
    #[default]
    String,
    Number,
    Boolean,
    Null,
    Object,
    Array,
}

impl JsonValueType {
    pub(super) fn all() -> Vec<Self> {
        vec![
            Self::String,
            Self::Number,
            Self::Boolean,
            Self::Null,
            Self::Object,
            Self::Array,
        ]
    }

    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::String => "string",
            Self::Number => "number",
            Self::Boolean => "boolean",
            Self::Null => "null",
            Self::Object => "object",
            Self::Array => "array",
        }
    }

    pub(super) fn label(self) -> &'static str {
        match self {
            Self::String => "String",
            Self::Number => "Number",
            Self::Boolean => "Boolean",
            Self::Null => "Null",
            Self::Object => "Object",
            Self::Array => "Array",
        }
    }

    pub(super) fn from_str(value: &str) -> Option<Self> {
        match value {
            "string" => Some(Self::String),
            "number" => Some(Self::Number),
            "boolean" => Some(Self::Boolean),
            "null" => Some(Self::Null),
            "object" => Some(Self::Object),
            "array" => Some(Self::Array),
            _ => None,
        }
    }

    pub(super) fn is_composite(self) -> bool {
        matches!(self, Self::Object | Self::Array)
    }
}

impl fmt::Display for JsonValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}

/// Visual density for JsonInput row controls.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum JsonInputDensity {
    #[default]
    Compact,
    Comfortable,
}

/// Controls when JsonInput surfaces validation messages in the editor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum JsonValidationMode {
    #[default]
    OnBlurOrSubmit,
    Always,
}

/// One segment in a JsonInput path policy.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum JsonInputPathSegment {
    Key(String),
    AnyKey,
    Index(usize),
    AnyIndex,
}

/// Path builder used to target nested JsonInput editing policies.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct JsonInputPath {
    pub segments: Vec<JsonInputPathSegment>,
}

impl JsonInputPath {
    pub fn root() -> Self {
        Self::default()
    }

    pub fn key(key: impl Into<String>) -> Self {
        Self {
            segments: vec![JsonInputPathSegment::Key(key.into())],
        }
    }

    pub fn keys<I, K>(keys: I) -> Self
    where
        I: IntoIterator<Item = K>,
        K: Into<String>,
    {
        Self {
            segments: keys
                .into_iter()
                .map(|key| JsonInputPathSegment::Key(key.into()))
                .collect(),
        }
    }

    pub fn index(mut self, index: usize) -> Self {
        self.segments.push(JsonInputPathSegment::Index(index));
        self
    }

    pub fn any_key(mut self) -> Self {
        self.segments.push(JsonInputPathSegment::AnyKey);
        self
    }

    pub fn any_index(mut self) -> Self {
        self.segments.push(JsonInputPathSegment::AnyIndex);
        self
    }

    pub(super) fn child_key(&self, key: impl Into<String>) -> Self {
        let mut next = self.clone();
        next.segments.push(JsonInputPathSegment::Key(key.into()));
        next
    }

    pub(super) fn child_index(&self, index: usize) -> Self {
        let mut next = self.clone();
        next.segments.push(JsonInputPathSegment::Index(index));
        next
    }

    pub(super) fn parent(&self) -> Self {
        let mut next = self.clone();
        next.segments.pop();
        next
    }

    pub(super) fn depth(&self) -> usize {
        self.segments.len()
    }
}

/// Editing policy for one JsonInput path or path pattern.
#[derive(Clone, PartialEq, Debug, Default)]
pub struct JsonInputPathPolicy {
    pub path: JsonInputPath,
    pub key_editable: Option<bool>,
    pub type_editable: Option<bool>,
    pub value_editable: Option<bool>,
    pub allowed_types: Option<Vec<JsonValueType>>,
    pub default_new_type: Option<JsonValueType>,
    pub removable: Option<bool>,
    pub allow_add_children: Option<bool>,
    pub allow_remove_children: Option<bool>,
}

impl JsonInputPathPolicy {
    pub fn for_path(path: JsonInputPath) -> Self {
        Self {
            path,
            ..Self::default()
        }
    }

    pub fn for_key(key: impl Into<String>) -> Self {
        Self::for_path(JsonInputPath::key(key))
    }

    pub fn keys<I, K>(keys: I) -> Self
    where
        I: IntoIterator<Item = K>,
        K: Into<String>,
    {
        Self::for_path(JsonInputPath::keys(keys))
    }

    pub fn any_index(mut self) -> Self {
        self.path = self.path.any_index();
        self
    }

    pub fn any_key(mut self) -> Self {
        self.path = self.path.any_key();
        self
    }

    pub fn index(mut self, index: usize) -> Self {
        self.path = self.path.index(index);
        self
    }

    pub fn key_editable(mut self, value: bool) -> Self {
        self.key_editable = Some(value);
        self
    }

    pub fn type_editable(mut self, value: bool) -> Self {
        self.type_editable = Some(value);
        self
    }

    pub fn value_editable(mut self, value: bool) -> Self {
        self.value_editable = Some(value);
        self
    }

    pub fn allowed_types(mut self, value: Vec<JsonValueType>) -> Self {
        self.allowed_types = Some(value);
        self
    }

    pub fn default_new_type(mut self, value: JsonValueType) -> Self {
        self.default_new_type = Some(value);
        self
    }

    pub fn removable(mut self, value: bool) -> Self {
        self.removable = Some(value);
        self
    }

    pub fn allow_add_children(mut self, value: bool) -> Self {
        self.allow_add_children = Some(value);
        self
    }

    pub fn allow_remove_children(mut self, value: bool) -> Self {
        self.allow_remove_children = Some(value);
        self
    }
}

/// Simple validity state emitted by JsonInput and JSON-backed wrappers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonInputValidity {
    pub is_valid: bool,
    pub errors: Vec<JsonInputError>,
}

/// One validation issue found inside a JsonInput draft.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonInputError {
    pub path: String,
    pub message: String,
    pub kind: JsonInputErrorKind,
}

/// Machine-readable category for a JsonInput validation issue.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JsonInputErrorKind {
    EmptyKey,
    EmptyValue,
    DuplicateKey,
    InvalidNumber,
    MaxDepthExceeded,
    RequiredObjectEmpty,
    UnsupportedType,
}

/// Props for JsonInput, a JSON editor that submits one hidden JSON value.
#[derive(Properties, PartialEq, Clone)]
pub struct JsonInputProps {
    pub id: AttrValue,
    #[prop_or_default]
    pub name: Option<AttrValue>,
    pub label: AttrValue,
    #[prop_or_default]
    pub initial_value: Option<Value>,
    #[prop_or_default]
    pub on_json_change: Option<Callback<Value>>,
    #[prop_or(true)]
    pub display_buttons: bool,
    #[prop_or(false)]
    pub require_at_least_one: bool,
    #[prop_or(false)]
    pub require_string_values: bool,
    #[prop_or_else(|| AttrValue::from("Enter a value."))]
    pub empty_string_value_message: AttrValue,
    #[prop_or_default]
    pub disable_keys: bool,
    #[prop_or_default]
    pub disable_values: bool,
    #[prop_or_default]
    pub helper_text: Option<AttrValue>,
    #[prop_or_default]
    pub placeholder_key: Option<AttrValue>,
    #[prop_or_default]
    pub placeholder_value: Option<AttrValue>,
    #[prop_or(false)]
    pub show_json_preview: bool,
    #[prop_or(false)]
    pub allow_raw_json_paste: bool,
    #[prop_or_else(default_paste_label)]
    pub paste_label: AttrValue,
    #[prop_or_default]
    pub paste_helper_text: Option<AttrValue>,
    #[prop_or_else(default_paste_placeholder)]
    pub paste_placeholder: AttrValue,
    #[prop_or_else(default_apply_paste_label)]
    pub apply_paste_label: AttrValue,
    #[prop_or_default]
    pub allowed_types: Option<Vec<JsonValueType>>,
    #[prop_or_default]
    pub default_new_type: JsonValueType,
    #[prop_or_default]
    pub max_depth: Option<usize>,
    #[prop_or_default]
    pub on_validity_change: Option<Callback<JsonInputValidity>>,
    #[prop_or_default]
    pub on_validation_report_change: Option<Callback<JsonBackedValidationReport>>,
    #[prop_or_default]
    pub validation_mode: JsonValidationMode,
    #[prop_or(false)]
    pub validation_requested: bool,
    #[prop_or_default]
    pub validation_request_id: Option<u64>,
    #[prop_or(true)]
    pub block_form_submit_when_invalid: bool,
    #[prop_or_default]
    pub density: JsonInputDensity,
    #[prop_or_default]
    pub path_policies: Option<Vec<JsonInputPathPolicy>>,
    #[prop_or_else(|| AttrValue::from("Add property"))]
    pub add_property_label: AttrValue,
    #[prop_or_else(|| AttrValue::from("Add item"))]
    pub add_item_label: AttrValue,
}

pub(crate) fn default_paste_label() -> AttrValue {
    AttrValue::from("Paste JSON")
}

pub(crate) fn default_paste_placeholder() -> AttrValue {
    AttrValue::from("{\n  \"example\": true\n}")
}

pub(crate) fn default_apply_paste_label() -> AttrValue {
    AttrValue::from("Apply JSON")
}

#[derive(Clone, PartialEq)]
pub(super) struct JsonInputConfig {
    pub display_buttons: bool,
    pub require_at_least_one: bool,
    pub require_string_values: bool,
    pub empty_string_value_message: AttrValue,
    pub disable_keys: bool,
    pub disable_values: bool,
    pub placeholder_key: AttrValue,
    pub placeholder_value: AttrValue,
    pub allowed_types: Vec<JsonValueType>,
    pub default_new_type: JsonValueType,
    pub max_depth: Option<usize>,
    pub density: JsonInputDensity,
    pub path_policies: Vec<JsonInputPathPolicy>,
    pub add_property_label: AttrValue,
    pub add_item_label: AttrValue,
}

impl JsonInputConfig {
    pub(super) fn from_props(props: &JsonInputProps) -> Self {
        let allowed_types = props
            .allowed_types
            .clone()
            .filter(|types| !types.is_empty())
            .unwrap_or_else(JsonValueType::all);
        let default_new_type = if allowed_types.contains(&props.default_new_type) {
            props.default_new_type
        } else {
            allowed_types.first().copied().unwrap_or_default()
        };

        Self {
            display_buttons: props.display_buttons,
            require_at_least_one: props.require_at_least_one,
            require_string_values: props.require_string_values,
            empty_string_value_message: props.empty_string_value_message.clone(),
            disable_keys: props.disable_keys,
            disable_values: props.disable_values,
            placeholder_key: props
                .placeholder_key
                .clone()
                .unwrap_or_else(|| AttrValue::from(DEFAULT_KEY_PLACEHOLDER)),
            placeholder_value: props
                .placeholder_value
                .clone()
                .unwrap_or_else(|| AttrValue::from(DEFAULT_VALUE_PLACEHOLDER)),
            allowed_types,
            default_new_type,
            max_depth: props.max_depth,
            density: props.density,
            path_policies: props.path_policies.clone().unwrap_or_default(),
            add_property_label: props.add_property_label.clone(),
            add_item_label: props.add_item_label.clone(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub(super) struct JsonModel {
    pub id: Uuid,
    pub kind: JsonNodeKind,
    pub touched: bool,
    pub expanded: bool,
    pub replaced_kind: Option<JsonNodeKind>,
}

#[derive(Clone, PartialEq, Debug)]
pub(super) enum JsonNodeKind {
    String(String),
    Number { raw: String },
    Boolean(bool),
    Null,
    Object(Vec<JsonPropertyNode>),
    Array(Vec<JsonArrayItemNode>),
}

#[derive(Clone, PartialEq, Debug)]
pub(super) struct JsonPropertyNode {
    pub id: Uuid,
    pub key: String,
    pub key_touched: bool,
    pub value: JsonModel,
}

#[derive(Clone, PartialEq, Debug)]
pub(super) struct JsonArrayItemNode {
    pub id: Uuid,
    pub value: JsonModel,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub(super) enum JsonPathSegment {
    Property(Uuid),
    ArrayItem(Uuid),
}

#[derive(Clone, PartialEq, Debug)]
pub(super) struct JsonInputIssue {
    pub node_id: Option<Uuid>,
    pub path: String,
    pub message: String,
    pub kind: JsonInputErrorKind,
}

#[derive(Clone, PartialEq, Debug)]
pub(super) struct JsonValidationReport {
    pub validity: JsonInputValidity,
    pub issues: Vec<JsonInputIssue>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum ValidationVisibility {
    All,
    Touched,
}
