use super::object_field::FieldArrayObjectField;
use super::{FieldArrayCustomIssue, FieldArrayValidationContext, FieldArrayValidator};
use crate::form::{JsonBackedValidationReport, JsonInputValidity, JsonValueType};
use serde_json::Value;
use yew::prelude::*;

/// Delete behavior for object rows in FieldArray.
#[derive(Clone, PartialEq, Debug, Default)]
pub enum FieldArrayDeleteBehavior {
    #[default]
    Remove,
    MarkDeleted {
        marker_key: AttrValue,
        identity_key: Option<AttrValue>,
        hide_marked_rows: bool,
        allow_restore: bool,
    },
}

impl FieldArrayDeleteBehavior {
    pub fn mark_deleted() -> Self {
        Self::MarkDeleted {
            marker_key: AttrValue::from("_deleted"),
            identity_key: Some(AttrValue::from("id")),
            hide_marked_rows: true,
            allow_restore: true,
        }
    }

    pub fn mark_deleted_with_key(marker_key: impl Into<AttrValue>) -> Self {
        Self::MarkDeleted {
            marker_key: marker_key.into(),
            identity_key: Some(AttrValue::from("id")),
            hide_marked_rows: true,
            allow_restore: true,
        }
    }
}

/// Small text customization surface for FieldArray status and item labels.
#[derive(Clone, PartialEq, Debug)]
pub struct FieldArrayText {
    pub item_label: AttrValue,
    pub item_label_plural: AttrValue,
    pub valid_status: AttrValue,
    pub invalid_status: AttrValue,
    pub deleted_rows_description: AttrValue,
}

impl Default for FieldArrayText {
    fn default() -> Self {
        Self {
            item_label: AttrValue::from("item"),
            item_label_plural: AttrValue::from("items"),
            valid_status: AttrValue::from("Ready to submit."),
            invalid_status: AttrValue::from("Fix the highlighted fields before submitting."),
            deleted_rows_description: AttrValue::from(
                "Marked items will be removed when you save.",
            ),
        }
    }
}

/// Props for FieldArray, a JSON array editor for scalar or object rows.
#[derive(Properties, PartialEq, Clone)]
pub struct FieldArrayProps {
    pub id: AttrValue,
    #[prop_or_default]
    pub name: Option<AttrValue>,
    pub label: AttrValue,

    #[prop_or_default]
    pub helper_text: Option<AttrValue>,

    #[prop_or_default]
    pub text: FieldArrayText,

    #[prop_or_default]
    pub initial_value: Option<Value>,

    #[prop_or(JsonValueType::String)]
    pub item_type: JsonValueType,

    #[prop_or_default]
    pub allowed_item_types: Option<Vec<JsonValueType>>,

    #[prop_or_default]
    pub object_fields: Option<Vec<FieldArrayObjectField>>,

    #[prop_or(true)]
    pub preserve_unknown_fields: bool,

    #[prop_or_default]
    pub delete_behavior: FieldArrayDeleteBehavior,

    #[prop_or_default]
    pub placeholder: Option<AttrValue>,

    #[prop_or_else(|| AttrValue::from("Add item"))]
    pub add_label: AttrValue,

    #[prop_or_default]
    pub min_items: Option<usize>,

    #[prop_or_default]
    pub max_items: Option<usize>,

    #[prop_or(true)]
    pub allow_remove: bool,

    #[prop_or_default]
    pub show_json_preview: bool,

    #[prop_or(true)]
    pub block_form_submit_when_invalid: bool,

    #[prop_or_default]
    pub on_json_change: Option<Callback<Value>>,

    #[prop_or_default]
    pub custom_validate: Option<Callback<FieldArrayValidationContext, Vec<FieldArrayCustomIssue>>>,

    #[prop_or_default]
    pub validators: Vec<FieldArrayValidator>,

    #[prop_or_default]
    pub on_validity_change: Option<Callback<JsonInputValidity>>,

    #[prop_or_default]
    pub on_validation_report_change: Option<Callback<JsonBackedValidationReport>>,
}

#[derive(Clone, PartialEq, Debug)]
pub(crate) struct FieldArrayDeleteConfig {
    pub marker_key: Option<String>,
    pub identity_key: Option<String>,
    pub hide_marked_rows: bool,
    pub allow_restore: bool,
}

impl FieldArrayDeleteConfig {
    pub(crate) fn remove() -> Self {
        Self {
            marker_key: None,
            identity_key: None,
            hide_marked_rows: false,
            allow_restore: false,
        }
    }

    pub(crate) fn from_behavior(behavior: &FieldArrayDeleteBehavior) -> Self {
        match behavior {
            FieldArrayDeleteBehavior::Remove => Self::remove(),
            FieldArrayDeleteBehavior::MarkDeleted {
                marker_key,
                identity_key,
                hide_marked_rows,
                allow_restore,
            } => Self {
                marker_key: Some(marker_key.to_string()),
                identity_key: identity_key.as_ref().map(ToString::to_string),
                hide_marked_rows: *hide_marked_rows,
                allow_restore: *allow_restore,
            },
        }
    }

    pub(crate) fn is_mark_deleted(&self) -> bool {
        self.marker_key.is_some()
    }
}
