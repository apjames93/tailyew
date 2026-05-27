use serde_json::Value;
use yew::prelude::*;

use crate::form::{JsonBackedValidationReport, JsonInputValidity};

/// Props for TagsInput, a chip editor that submits a JSON string array.
#[derive(Properties, PartialEq, Clone)]
pub struct TagsInputProps {
    pub id: AttrValue,
    #[prop_or_default]
    pub name: Option<AttrValue>,
    pub label: AttrValue,

    #[prop_or_default]
    pub helper_text: Option<AttrValue>,

    #[prop_or_default]
    pub initial_tags: Vec<String>,

    #[prop_or_else(|| AttrValue::from("Add tag"))]
    pub placeholder: AttrValue,

    #[prop_or(false)]
    pub allow_duplicates: bool,

    #[prop_or(true)]
    pub allow_custom_tags: bool,

    #[prop_or_default]
    pub min_tags: Option<usize>,

    #[prop_or_default]
    pub max_tags: Option<usize>,

    #[prop_or_default]
    pub suggestions: Vec<String>,

    #[prop_or_default]
    pub on_change: Option<Callback<Vec<String>>>,

    #[prop_or_default]
    pub on_json_change: Option<Callback<Value>>,

    #[prop_or_default]
    pub on_validity_change: Option<Callback<JsonInputValidity>>,

    #[prop_or_default]
    pub on_validation_report_change: Option<Callback<JsonBackedValidationReport>>,

    #[prop_or(true)]
    pub block_form_submit_when_invalid: bool,
}
