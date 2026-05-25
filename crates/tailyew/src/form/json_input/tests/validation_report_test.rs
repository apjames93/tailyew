use super::super::validation_report::{
    JsonBackedFormReportsStore, section_map_from_pairs, summary_entries_from_report,
    summary_entries_from_reports, validation_report_from_json_input_validity,
};
use crate::form::{
    JsonBackedValidationIssue, JsonBackedValidationReport, JsonInputError, JsonInputErrorKind,
    JsonInputValidity,
};
use yew::AttrValue;

fn report(
    field_name: &str,
    label: &str,
    is_valid: bool,
    issues: Vec<JsonBackedValidationIssue>,
) -> JsonBackedValidationReport {
    JsonBackedValidationReport {
        is_valid,
        field_id: AttrValue::from(format!("{field_name}_editor")),
        field_name: AttrValue::from(field_name.to_owned()),
        label: Some(AttrValue::from(label.to_owned())),
        issues,
    }
}

fn issue(message: &str) -> JsonBackedValidationIssue {
    JsonBackedValidationIssue {
        message: AttrValue::from(message.to_owned()),
        label: None,
        location: None,
        path: None,
        row_index: None,
        key: None,
        raw_path: None,
    }
}

#[test]
fn json_backed_valid_report_has_no_issues() {
    let report = validation_report_from_json_input_validity(
        &AttrValue::from("headers_editor"),
        &Some(AttrValue::from("headers")),
        &AttrValue::from("Headers"),
        &JsonInputValidity {
            is_valid: true,
            errors: Vec::new(),
        },
    );

    assert!(report.is_valid);
    assert_eq!(report.field_id, AttrValue::from("headers_editor"));
    assert_eq!(report.field_name, AttrValue::from("headers"));
    assert_eq!(report.label, Some(AttrValue::from("Headers")));
    assert!(report.issues.is_empty());
}

#[test]
fn json_backed_invalid_report_preserves_raw_path_and_message() {
    let report = validation_report_from_json_input_validity(
        &AttrValue::from("json_editor"),
        &None,
        &AttrValue::from("JSON"),
        &JsonInputValidity {
            is_valid: false,
            errors: vec![JsonInputError {
                path: "$.name".to_owned(),
                message: "Enter a value.".to_owned(),
                kind: JsonInputErrorKind::EmptyValue,
            }],
        },
    );

    assert!(!report.is_valid);
    assert_eq!(report.field_name, AttrValue::from("json_editor"));
    assert_eq!(report.issues[0].message, AttrValue::from("Enter a value."));
    assert_eq!(report.issues[0].path, Some(AttrValue::from("$.name")));
    assert_eq!(report.issues[0].raw_path, Some(AttrValue::from("$.name")));
}

#[test]
fn json_backed_report_clears_for_same_component_when_valid() {
    let id = AttrValue::from("headers_editor");
    let name = Some(AttrValue::from("headers"));
    let label = AttrValue::from("Headers");
    let invalid_report = validation_report_from_json_input_validity(
        &id,
        &name,
        &label,
        &JsonInputValidity {
            is_valid: false,
            errors: vec![JsonInputError {
                path: "$.Authorization".to_owned(),
                message: "Enter a value.".to_owned(),
                kind: JsonInputErrorKind::EmptyValue,
            }],
        },
    );
    let valid_report = validation_report_from_json_input_validity(
        &id,
        &name,
        &label,
        &JsonInputValidity {
            is_valid: true,
            errors: Vec::new(),
        },
    );

    assert!(!invalid_report.is_valid);
    assert_eq!(invalid_report.issues.len(), 1);
    assert!(valid_report.is_valid);
    assert!(valid_report.issues.is_empty());
    assert_eq!(valid_report.field_id, id);
    assert_eq!(valid_report.field_name, AttrValue::from("headers"));
    assert_eq!(valid_report.label, Some(label));
}

#[test]
fn json_backed_form_reports_empty_store_is_valid() {
    let store = JsonBackedFormReportsStore::default();

    assert!(store.all_valid());
    assert!(store.is_valid("headers"));
    assert_eq!(store.issue_count_all(), 0);
}

#[test]
fn json_backed_form_reports_track_invalid_field() {
    let mut store = JsonBackedFormReportsStore::default();

    store.apply_report(
        "headers",
        report("headers", "Headers", false, vec![issue("Enter a value.")]),
    );

    assert!(!store.is_valid("headers"));
    assert_eq!(store.issue_count("headers"), 1);
    assert_eq!(store.issue_count_all(), 1);
}

#[test]
fn json_backed_form_reports_valid_report_clears_previous_errors() {
    let mut store = JsonBackedFormReportsStore::default();

    store.apply_report(
        "headers",
        report("headers", "Headers", false, vec![issue("Enter a value.")]),
    );
    store.apply_report("headers", report("headers", "Headers", true, Vec::new()));

    assert!(store.is_valid("headers"));
    assert_eq!(store.issue_count("headers"), 0);
    assert_eq!(
        store
            .report("headers")
            .expect("report remains tracked")
            .issues,
        Vec::new()
    );
}

#[test]
fn json_backed_form_reports_all_valid_reflects_tracked_reports() {
    let mut store = JsonBackedFormReportsStore::default();

    store.apply_report(
        "parameters",
        report("parameters", "Parameters", true, Vec::new()),
    );
    store.apply_report(
        "headers",
        report("headers", "Headers", false, vec![issue("Enter a value.")]),
    );

    assert!(!store.all_valid());

    store.apply_report("headers", report("headers", "Headers", true, Vec::new()));

    assert!(store.all_valid());
}

#[test]
fn json_backed_form_reports_section_counts_use_field_mapping() {
    let mut store = JsonBackedFormReportsStore::default();
    let sections = section_map_from_pairs(vec![
        ("parameters", "schema"),
        ("headers", "advanced"),
        ("static_values", "advanced"),
    ]);

    store.apply_report(
        "headers",
        report("headers", "Headers", false, vec![issue("Enter a value.")]),
    );
    store.apply_report(
        "static_values",
        report(
            "static_values",
            "Static values",
            false,
            vec![issue("Enter a value."), issue("Choose a valid option.")],
        ),
    );

    assert!(store.section_is_valid(&sections, "schema"));
    assert!(!store.section_is_valid(&sections, "advanced"));
    assert_eq!(store.section_issue_count(&sections, "advanced"), 3);
    assert_eq!(store.section_reports(&sections, "advanced").len(), 2);
}

#[test]
fn json_backed_summary_entries_preserve_label_location_and_message() {
    let report = report(
        "parameters",
        "Parameters",
        false,
        vec![JsonBackedValidationIssue {
            message: AttrValue::from("Field names must be unique."),
            label: Some(AttrValue::from("Field name")),
            location: Some(AttrValue::from("Item 2 · Field name")),
            path: Some(AttrValue::from("$[1].name")),
            row_index: Some(1),
            key: Some(AttrValue::from("name")),
            raw_path: Some(AttrValue::from("$[1].name")),
        }],
    );

    let entries = summary_entries_from_report(&report);

    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].field_name, AttrValue::from("parameters"));
    assert_eq!(entries[0].field_label, Some(AttrValue::from("Field name")));
    assert_eq!(
        entries[0].location,
        Some(AttrValue::from("Item 2 · Field name"))
    );
    assert_eq!(
        entries[0].message,
        AttrValue::from("Field names must be unique.")
    );
    assert_eq!(summary_entries_from_reports(&[report]).len(), 1);
}
