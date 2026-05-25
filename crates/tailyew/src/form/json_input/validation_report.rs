use super::types::{JsonInputError, JsonInputValidity};
use crate::form::submitted_name;
use std::collections::BTreeMap;
use std::rc::Rc;
use yew::prelude::*;

/// Structured validation state for one JSON-backed form field.
#[derive(Clone, PartialEq, Debug)]
pub struct JsonBackedValidationReport {
    pub is_valid: bool,
    pub field_id: AttrValue,
    pub field_name: AttrValue,
    pub label: Option<AttrValue>,
    pub issues: Vec<JsonBackedValidationIssue>,
}

/// One user-facing and machine-readable issue in a JSON-backed field report.
#[derive(Clone, PartialEq, Debug)]
pub struct JsonBackedValidationIssue {
    pub message: AttrValue,
    pub label: Option<AttrValue>,
    pub location: Option<AttrValue>,
    pub path: Option<AttrValue>,
    pub row_index: Option<usize>,
    pub key: Option<AttrValue>,
    pub raw_path: Option<AttrValue>,
}

/// Flattened issue row suitable for section summaries or sticky form summaries.
#[derive(Clone, PartialEq, Debug)]
pub struct JsonBackedValidationSummaryEntry {
    pub field_name: AttrValue,
    pub field_label: Option<AttrValue>,
    pub location: Option<AttrValue>,
    pub message: AttrValue,
}

/// Hook handle for aggregating validation reports from multiple JSON-backed fields.
#[derive(Clone)]
pub struct JsonBackedFormReportsHandle {
    store: UseStateHandle<JsonBackedFormReportsStore>,
    sections: Rc<BTreeMap<String, String>>,
}

impl JsonBackedFormReportsHandle {
    /// Returns a callback that stores the latest report for `field_name`.
    pub fn on_report(
        &self,
        field_name: impl Into<AttrValue>,
    ) -> Option<Callback<JsonBackedValidationReport>> {
        let store = self.store.clone();
        let field_name = field_name.into().to_string();

        Some(Callback::from(move |report: JsonBackedValidationReport| {
            let mut next = (*store).clone();
            next.apply_report(&field_name, report);
            store.set(next);
        }))
    }

    /// Returns whether the latest report for `field_name` is valid.
    ///
    /// Fields without a report are treated as valid.
    pub fn is_valid(&self, field_name: &str) -> bool {
        self.store.is_valid(field_name)
    }

    /// Returns the latest issue count for one field.
    pub fn issue_count(&self, field_name: &str) -> usize {
        self.store.issue_count(field_name)
    }

    /// Returns the latest report for one field, if that field has emitted one.
    pub fn report(&self, field_name: &str) -> Option<JsonBackedValidationReport> {
        self.store.report(field_name)
    }

    /// Returns true when every tracked field is valid.
    pub fn all_valid(&self) -> bool {
        self.store.all_valid()
    }

    /// Returns the total issue count across all tracked fields.
    pub fn issue_count_all(&self) -> usize {
        self.store.issue_count_all()
    }

    /// Returns all latest reports in stable field-name order.
    pub fn reports(&self) -> Vec<JsonBackedValidationReport> {
        self.store.reports()
    }

    /// Returns whether every tracked field mapped to `section_key` is valid.
    pub fn section_is_valid(&self, section_key: &str) -> bool {
        self.store.section_is_valid(&self.sections, section_key)
    }

    /// Returns the total issue count for fields mapped to `section_key`.
    pub fn section_issue_count(&self, section_key: &str) -> usize {
        self.store.section_issue_count(&self.sections, section_key)
    }

    /// Returns latest reports for fields mapped to `section_key`.
    pub fn section_reports(&self, section_key: &str) -> Vec<JsonBackedValidationReport> {
        self.store.section_reports(&self.sections, section_key)
    }
}

/// Aggregates validation reports from JSON-backed fields by field name.
#[hook]
pub fn use_json_backed_form_reports() -> JsonBackedFormReportsHandle {
    use_json_backed_form_reports_with_sections(Vec::<(&str, &str)>::new())
}

/// Aggregates validation reports and maps field names to section keys.
#[hook]
pub fn use_json_backed_form_reports_with_sections<Field, Section>(
    sections: Vec<(Field, Section)>,
) -> JsonBackedFormReportsHandle
where
    Field: Into<AttrValue> + 'static,
    Section: Into<AttrValue> + 'static,
{
    let store = use_state_eq(JsonBackedFormReportsStore::default);

    JsonBackedFormReportsHandle {
        store,
        sections: Rc::new(section_map_from_pairs(sections)),
    }
}

#[derive(Clone, PartialEq, Debug, Default)]
pub(crate) struct JsonBackedFormReportsStore {
    reports: BTreeMap<String, JsonBackedValidationReport>,
}

impl JsonBackedFormReportsStore {
    pub(crate) fn apply_report(&mut self, field_name: &str, report: JsonBackedValidationReport) {
        self.reports.insert(field_name.to_owned(), report);
    }

    pub(crate) fn is_valid(&self, field_name: &str) -> bool {
        self.reports
            .get(field_name)
            .map(|report| report.is_valid)
            .unwrap_or(true)
    }

    pub(crate) fn issue_count(&self, field_name: &str) -> usize {
        self.reports
            .get(field_name)
            .map(|report| report.issues.len())
            .unwrap_or_default()
    }

    pub(crate) fn report(&self, field_name: &str) -> Option<JsonBackedValidationReport> {
        self.reports.get(field_name).cloned()
    }

    pub(crate) fn all_valid(&self) -> bool {
        self.reports.values().all(|report| report.is_valid)
    }

    pub(crate) fn issue_count_all(&self) -> usize {
        self.reports
            .values()
            .map(|report| report.issues.len())
            .sum()
    }

    pub(crate) fn reports(&self) -> Vec<JsonBackedValidationReport> {
        self.reports.values().cloned().collect()
    }

    pub(crate) fn section_is_valid(
        &self,
        sections: &BTreeMap<String, String>,
        section_key: &str,
    ) -> bool {
        sections
            .iter()
            .filter(|(_, section)| section.as_str() == section_key)
            .all(|(field_name, _)| self.is_valid(field_name))
    }

    pub(crate) fn section_issue_count(
        &self,
        sections: &BTreeMap<String, String>,
        section_key: &str,
    ) -> usize {
        sections
            .iter()
            .filter(|(_, section)| section.as_str() == section_key)
            .map(|(field_name, _)| self.issue_count(field_name))
            .sum()
    }

    pub(crate) fn section_reports(
        &self,
        sections: &BTreeMap<String, String>,
        section_key: &str,
    ) -> Vec<JsonBackedValidationReport> {
        sections
            .iter()
            .filter(|(_, section)| section.as_str() == section_key)
            .filter_map(|(field_name, _)| self.report(field_name))
            .collect()
    }
}

pub(crate) fn section_map_from_pairs<Field, Section>(
    sections: Vec<(Field, Section)>,
) -> BTreeMap<String, String>
where
    Field: Into<AttrValue>,
    Section: Into<AttrValue>,
{
    sections
        .into_iter()
        .map(|(field_name, section_key)| {
            (
                field_name.into().to_string(),
                section_key.into().to_string(),
            )
        })
        .collect()
}

/// Converts one validation report into user-facing summary entries.
pub fn summary_entries_from_report(
    report: &JsonBackedValidationReport,
) -> Vec<JsonBackedValidationSummaryEntry> {
    report
        .issues
        .iter()
        .map(|issue| JsonBackedValidationSummaryEntry {
            field_name: report.field_name.clone(),
            field_label: issue.label.clone().or_else(|| report.label.clone()),
            location: issue.location.clone(),
            message: issue.message.clone(),
        })
        .collect()
}

/// Converts multiple validation reports into user-facing summary entries.
pub fn summary_entries_from_reports(
    reports: &[JsonBackedValidationReport],
) -> Vec<JsonBackedValidationSummaryEntry> {
    reports
        .iter()
        .flat_map(summary_entries_from_report)
        .collect()
}

pub(crate) fn json_backed_report_label(label: &AttrValue) -> Option<AttrValue> {
    (!label.as_str().trim().is_empty()).then(|| label.clone())
}

pub(crate) fn validation_report_from_json_input_validity(
    id: &AttrValue,
    name: &Option<AttrValue>,
    label: &AttrValue,
    validity: &JsonInputValidity,
) -> JsonBackedValidationReport {
    JsonBackedValidationReport {
        is_valid: validity.is_valid,
        field_id: id.clone(),
        field_name: submitted_name(id, name),
        label: json_backed_report_label(label),
        issues: validity
            .errors
            .iter()
            .map(issue_from_json_input_error)
            .collect(),
    }
}

pub(crate) fn issue_from_json_input_error(error: &JsonInputError) -> JsonBackedValidationIssue {
    let path = AttrValue::from(error.path.clone());

    JsonBackedValidationIssue {
        message: AttrValue::from(error.message.clone()),
        label: None,
        location: None,
        path: Some(path.clone()),
        row_index: None,
        key: None,
        raw_path: Some(path),
    }
}
