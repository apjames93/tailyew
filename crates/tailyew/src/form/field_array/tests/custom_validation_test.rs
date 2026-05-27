use super::super::custom_validation::{
    FieldArrayCustomIssue, FieldArrayValidationContext, validation_context_from_object_rows,
};
use super::super::normalize::normalize_object_rows_initial;
use super::super::object_field::{FieldArrayObjectField, FieldArraySelectOption};
use super::super::validation::validate_object_rows_with_delete_and_custom;
use crate::form::{JsonInputErrorKind, JsonValueType};
use serde_json::json;
use std::collections::BTreeMap;
use yew::Callback;

#[test]
fn field_array_validation_context_exposes_current_object_row_values() {
    let fields = vec![
        FieldArrayObjectField::hidden("id", JsonValueType::Number),
        FieldArrayObjectField::string("name", "Name"),
        FieldArrayObjectField::select(
            "access",
            "Access",
            vec![
                FieldArraySelectOption::same("read"),
                FieldArraySelectOption::same("admin"),
            ],
        ),
        FieldArrayObjectField::boolean("required", "Required"),
        FieldArrayObjectField::number("score", "Score"),
    ];
    let rows = normalize_object_rows_initial(
        Some(json!([
            {
                "id": 7,
                "name": "orders",
                "access": "admin",
                "required": true,
                "score": "not-a-number",
                "unknown_backend_key": "preserved"
            }
        ])),
        &fields,
        true,
    );

    let context = validation_context_from_object_rows(&rows.rows, &fields);
    let row = &context.rows[0];

    assert_eq!(row.get("id"), Some(&json!(7)));
    assert_eq!(row.get_string("name"), Some("orders"));
    assert_eq!(row.get_string("access"), Some("admin"));
    assert_eq!(row.get_bool("required"), Some(true));
    assert_eq!(row.get("score"), Some(&json!("not-a-number")));
    assert_eq!(row.get("unknown_backend_key"), Some(&json!("preserved")));
    assert_eq!(context.value[0]["access"], json!("admin"));
}

#[test]
fn field_array_custom_validator_can_return_no_issues() {
    let fields = vec![FieldArrayObjectField::string("name", "Name").required(true)];
    let rows = normalize_object_rows_initial(Some(json!([{ "name": "order_id" }])), &fields, true);
    let custom_validate = Callback::from(|_: FieldArrayValidationContext| Vec::new());

    let report = validate_object_rows_with_delete_and_custom(
        &rows.rows,
        &fields,
        None,
        &[],
        Some(&custom_validate),
    );

    assert!(report.validity.is_valid);
}

#[test]
fn field_array_custom_field_issue_merges_into_inline_object_issues() {
    let fields = vec![FieldArrayObjectField::string("name", "Name")];
    let rows = normalize_object_rows_initial(Some(json!([{ "name": "1invalid" }])), &fields, true);
    let custom_validate = Callback::from(|_: FieldArrayValidationContext| {
        vec![FieldArrayCustomIssue::field(
            0,
            "name",
            "Start with a letter.",
        )]
    });

    let report = validate_object_rows_with_delete_and_custom(
        &rows.rows,
        &fields,
        None,
        &[],
        Some(&custom_validate),
    );

    assert!(!report.validity.is_valid);
    assert_eq!(report.validity.errors[0].path, "$[0].name");
    assert_eq!(report.issues[0].row_index, 0);
    assert_eq!(report.issues[0].key.as_deref(), Some("name"));
    assert_eq!(report.issues[0].message, "Start with a letter.");
}

#[test]
fn field_array_custom_root_and_row_issues_merge_into_report() {
    let fields = vec![FieldArrayObjectField::string("name", "Name")];
    let rows = normalize_object_rows_initial(Some(json!([{ "name": "orders" }])), &fields, true);
    let custom_validate = Callback::from(|_: FieldArrayValidationContext| {
        vec![
            FieldArrayCustomIssue::root("Array-level issue."),
            FieldArrayCustomIssue::row(0, "Row-level issue."),
        ]
    });

    let report = validate_object_rows_with_delete_and_custom(
        &rows.rows,
        &fields,
        None,
        &[],
        Some(&custom_validate),
    );

    assert!(!report.validity.is_valid);
    assert!(report.validity.errors.iter().any(|error| error.path == "$"));
    assert!(
        report
            .validity
            .errors
            .iter()
            .any(|error| error.path == "$[0]")
    );
    assert_eq!(report.issues[0].key, None);
    assert_eq!(report.issues[0].message, "Row-level issue.");
}

#[test]
fn field_array_custom_issue_can_override_error_kind() {
    let fields = vec![FieldArrayObjectField::string("name", "Name")];
    let rows = normalize_object_rows_initial(Some(json!([{ "name": "orders" }])), &fields, true);
    let custom_validate = Callback::from(|_: FieldArrayValidationContext| {
        vec![
            FieldArrayCustomIssue::field(0, "name", "Value must be unique.")
                .with_kind(JsonInputErrorKind::DuplicateKey),
        ]
    });

    let report = validate_object_rows_with_delete_and_custom(
        &rows.rows,
        &fields,
        None,
        &[],
        Some(&custom_validate),
    );

    assert_eq!(
        report.validity.errors[0].kind,
        JsonInputErrorKind::DuplicateKey
    );
}

#[test]
fn field_array_custom_validator_can_mark_duplicate_names() {
    let fields = vec![FieldArrayObjectField::string("name", "Name")];
    let rows = normalize_object_rows_initial(
        Some(json!([{ "name": "order_id" }, { "name": "order_id" }])),
        &fields,
        true,
    );
    let custom_validate = Callback::from(|context: FieldArrayValidationContext| {
        let mut issues = Vec::new();
        let mut seen = BTreeMap::<String, usize>::new();

        for row in context.rows.iter().filter(|row| !row.deleted) {
            let Some(name) = row.get_string("name").map(str::trim) else {
                continue;
            };
            if name.is_empty() {
                continue;
            }

            if let Some(first_index) = seen.insert(name.to_owned(), row.index) {
                issues.push(FieldArrayCustomIssue::field(
                    first_index,
                    "name",
                    "Names must be unique.",
                ));
                issues.push(FieldArrayCustomIssue::field(
                    row.index,
                    "name",
                    "Names must be unique.",
                ));
            }
        }

        issues
    });

    let report = validate_object_rows_with_delete_and_custom(
        &rows.rows,
        &fields,
        None,
        &[],
        Some(&custom_validate),
    );

    assert!(!report.validity.is_valid);
    assert_eq!(report.issues.len(), 2);
    assert!(report.issues.iter().any(|issue| issue.row_index == 0));
    assert!(report.issues.iter().any(|issue| issue.row_index == 1));
}

#[test]
fn field_array_builtin_invalid_state_stays_invalid_with_custom_validator() {
    let fields = vec![FieldArrayObjectField::string("name", "Name").required(true)];
    let rows = normalize_object_rows_initial(Some(json!([{ "name": "" }])), &fields, true);
    let custom_validate = Callback::from(|_: FieldArrayValidationContext| Vec::new());

    let report = validate_object_rows_with_delete_and_custom(
        &rows.rows,
        &fields,
        None,
        &[],
        Some(&custom_validate),
    );

    assert!(!report.validity.is_valid);
    assert_eq!(report.validity.errors[0].message, "Enter a value.");
}
