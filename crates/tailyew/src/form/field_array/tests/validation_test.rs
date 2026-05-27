use super::super::normalize::{
    normalize_object_rows_initial, normalize_object_rows_initial_with_delete,
};
use super::super::object_field::{FieldArrayObjectField, FieldArraySelectOption};
use super::super::props::FieldArrayDeleteConfig;
use super::super::validation::{
    field_array_validation_report, friendly_validation_summary_items, validate_object_rows,
    validate_object_rows_with_delete_and_custom,
};
use super::game_fields;
use crate::form::{
    FieldArrayCustomIssue, FieldArrayDeleteBehavior, FieldArrayFieldValidationContext,
    FieldArrayText, FieldArrayValidationContext, FieldArrayValidator, JsonInputErrorKind,
    JsonValueType,
};
use serde_json::json;
use yew::AttrValue;
use yew::Callback;

#[test]
fn field_array_object_mode_validates_required_fields() {
    let rows = normalize_object_rows_initial(
        Some(json!([{ "id": 1, "name": "", "hours_played": "3", "beat": false }])),
        &game_fields(),
        true,
    );
    let report = validate_object_rows(&rows.rows, &game_fields(), None);

    assert!(!report.validity.is_valid);
    assert_eq!(report.validity.errors[0].path, "$[0].name");
}

#[test]
fn field_array_validation_summary_uses_friendly_field_labels() {
    let rows = normalize_object_rows_initial(
        Some(json!([
            { "id": 1, "name": "Resident Evil", "hours_played": "3", "beat": false },
            { "id": 2, "name": "Elden Ring", "hours_played": "100", "beat": true },
            { "id": 3, "name": "", "hours_played": "8", "beat": false }
        ])),
        &game_fields(),
        true,
    );
    let report = validate_object_rows(&rows.rows, &game_fields(), None);
    let items =
        friendly_validation_summary_items(&report, &game_fields(), &FieldArrayText::default());

    assert!(items.iter().all(|item| !item.contains("$[2].name")));
    assert!(
        items
            .iter()
            .any(|item| item == "Item 3 · Game: Enter a value.")
    );
}

#[test]
fn field_array_validation_summary_uses_custom_item_label() {
    let fields = vec![FieldArrayObjectField::string("name", "Name").required(true)];
    let rows = normalize_object_rows_initial(
        Some(json!([
            { "name": "first" },
            { "name": "second" },
            { "name": "" }
        ])),
        &fields,
        true,
    );
    let report = validate_object_rows(&rows.rows, &fields, None);
    let text = FieldArrayText {
        item_label: "game".into(),
        item_label_plural: "games".into(),
        ..FieldArrayText::default()
    };
    let items = friendly_validation_summary_items(&report, &fields, &text);

    assert!(
        items
            .iter()
            .any(|item| item == "Game 3 · Name: Enter a value.")
    );
}

#[test]
fn field_array_custom_validation_summary_uses_friendly_field_label() {
    let fields = vec![FieldArrayObjectField::string("name", "Field name")];
    let rows = normalize_object_rows_initial(Some(json!([{ "name": "order_id" }])), &fields, true);
    let custom_validate = Callback::from(|_: FieldArrayValidationContext| {
        vec![FieldArrayCustomIssue::field(
            0,
            "name",
            "Field names must be unique.",
        )]
    });
    let report = validate_object_rows_with_delete_and_custom(
        &rows.rows,
        &fields,
        None,
        &[],
        Some(&custom_validate),
    );
    let text = FieldArrayText {
        item_label: "model input".into(),
        item_label_plural: "model inputs".into(),
        ..FieldArrayText::default()
    };
    let items = friendly_validation_summary_items(&report, &fields, &text);

    assert!(
        items
            .iter()
            .any(|item| { item == "Model input 1 · Field name: Field names must be unique." })
    );
}

#[test]
fn field_array_validation_report_uses_row_and_field_metadata() {
    let fields = vec![
        FieldArrayObjectField::string("name", "Field name").required_trimmed("Enter a field name."),
    ];
    let rows = normalize_object_rows_initial(Some(json!([{ "name": "   " }])), &fields, true);
    let report = validate_object_rows(&rows.rows, &fields, None);
    let validation_report = field_array_validation_report(
        &AttrValue::from("model_inputs_editor"),
        &Some(AttrValue::from("model_inputs")),
        &AttrValue::from("Model inputs"),
        &report,
        &fields,
        &FieldArrayText {
            item_label: "model input".into(),
            item_label_plural: "model inputs".into(),
            ..FieldArrayText::default()
        },
    );

    assert!(!validation_report.is_valid);
    assert_eq!(
        validation_report.field_id,
        AttrValue::from("model_inputs_editor")
    );
    assert_eq!(
        validation_report.field_name,
        AttrValue::from("model_inputs")
    );
    assert_eq!(
        validation_report.label,
        Some(AttrValue::from("Model inputs"))
    );
    assert_eq!(validation_report.issues[0].row_index, Some(0));
    assert_eq!(
        validation_report.issues[0].key,
        Some(AttrValue::from("name"))
    );
    assert_eq!(
        validation_report.issues[0].label,
        Some(AttrValue::from("Field name"))
    );
    assert_eq!(
        validation_report.issues[0].location,
        Some(AttrValue::from("Model input 1 · Field name"))
    );
    assert_eq!(
        validation_report.issues[0].raw_path,
        Some(AttrValue::from("$[0].name"))
    );
}

#[test]
fn field_array_validation_report_flags_all_unique_duplicates() {
    let fields = vec![FieldArrayObjectField::string("name", "Field name")];
    let rows = normalize_object_rows_initial(
        Some(json!([{ "name": "order_id" }, { "name": " order_id " }])),
        &fields,
        true,
    );
    let validators = vec![FieldArrayValidator::unique_field_trimmed(
        "name",
        "Field names must be unique.",
    )];
    let report =
        validate_object_rows_with_delete_and_custom(&rows.rows, &fields, None, &validators, None);
    let validation_report = field_array_validation_report(
        &AttrValue::from("model_inputs_editor"),
        &None,
        &AttrValue::from("Model inputs"),
        &report,
        &fields,
        &FieldArrayText::default(),
    );

    let row_indices = validation_report
        .issues
        .iter()
        .map(|issue| issue.row_index)
        .collect::<Vec<_>>();

    assert_eq!(row_indices, vec![Some(0), Some(1)]);
    assert!(
        validation_report
            .issues
            .iter()
            .all(|issue| issue.key == Some(AttrValue::from("name")))
    );
}

#[test]
fn field_array_validation_report_clears_when_valid() {
    let fields =
        vec![FieldArrayObjectField::string("name", "Name").required_trimmed("Enter a name.")];
    let rows = normalize_object_rows_initial(Some(json!([{ "name": "order_id" }])), &fields, true);
    let report = validate_object_rows(&rows.rows, &fields, None);
    let validation_report = field_array_validation_report(
        &AttrValue::from("names_editor"),
        &None,
        &AttrValue::from("Names"),
        &report,
        &fields,
        &FieldArrayText::default(),
    );

    assert!(validation_report.is_valid);
    assert!(validation_report.issues.is_empty());
}

#[test]
fn field_array_validation_report_clears_after_fixing_same_field() {
    let fields =
        vec![FieldArrayObjectField::string("name", "Name").required_trimmed("Enter a name.")];
    let invalid_rows = normalize_object_rows_initial(Some(json!([{ "name": " " }])), &fields, true);
    let valid_rows =
        normalize_object_rows_initial(Some(json!([{ "name": "order_id" }])), &fields, true);
    let invalid_report = validate_object_rows(&invalid_rows.rows, &fields, None);
    let valid_report = validate_object_rows(&valid_rows.rows, &fields, None);

    let invalid_validation_report = field_array_validation_report(
        &AttrValue::from("names_editor"),
        &Some(AttrValue::from("names")),
        &AttrValue::from("Names"),
        &invalid_report,
        &fields,
        &FieldArrayText::default(),
    );
    let valid_validation_report = field_array_validation_report(
        &AttrValue::from("names_editor"),
        &Some(AttrValue::from("names")),
        &AttrValue::from("Names"),
        &valid_report,
        &fields,
        &FieldArrayText::default(),
    );

    assert!(!invalid_validation_report.is_valid);
    assert_eq!(invalid_validation_report.issues.len(), 1);
    assert!(valid_validation_report.is_valid);
    assert!(valid_validation_report.issues.is_empty());
    assert_eq!(
        valid_validation_report.field_id,
        AttrValue::from("names_editor")
    );
    assert_eq!(valid_validation_report.field_name, AttrValue::from("names"));
    assert_eq!(
        valid_validation_report.label,
        Some(AttrValue::from("Names"))
    );
}

#[test]
fn field_array_object_mode_validates_number_fields() {
    let fields = vec![FieldArrayObjectField::number("score", "Score")];
    let rows = normalize_object_rows_initial(Some(json!([{ "score": "abc" }])), &fields, true);
    let report = validate_object_rows(&rows.rows, &fields, None);

    assert!(!report.validity.is_valid);
    assert_eq!(
        report.validity.errors[0].kind,
        JsonInputErrorKind::InvalidNumber
    );
}

#[test]
fn field_array_deleted_rows_skip_required_and_number_validation() {
    let delete_config =
        FieldArrayDeleteConfig::from_behavior(&FieldArrayDeleteBehavior::mark_deleted());
    let fields = vec![
        FieldArrayObjectField::hidden("id", JsonValueType::Number),
        FieldArrayObjectField::string("name", "Game").required(true),
        FieldArrayObjectField::number("score", "Score"),
    ];
    let rows = normalize_object_rows_initial_with_delete(
        Some(json!([{ "id": 1, "name": "", "score": "bad", "_deleted": true }])),
        &fields,
        true,
        &delete_config,
    );
    let report = validate_object_rows(&rows.rows, &fields, None);

    assert!(report.validity.is_valid);
}

#[test]
fn field_array_select_field_validates_required_empty_value() {
    let fields = vec![
        FieldArrayObjectField::select("type", "Type", vec![FieldArraySelectOption::same("string")])
            .required(true),
    ];
    let rows = normalize_object_rows_initial(Some(json!([{ "type": "" }])), &fields, true);
    let report = validate_object_rows(&rows.rows, &fields, None);

    assert!(!report.validity.is_valid);
    assert_eq!(report.validity.errors[0].message, "Choose a value.");
}

#[test]
fn field_array_select_field_validates_allowed_options() {
    let fields = vec![FieldArrayObjectField::select(
        "type",
        "Type",
        vec![
            FieldArraySelectOption::same("string"),
            FieldArraySelectOption::same("boolean"),
        ],
    )];
    let rows = normalize_object_rows_initial(Some(json!([{ "type": "object" }])), &fields, true);
    let report = validate_object_rows(&rows.rows, &fields, None);

    assert!(!report.validity.is_valid);
    assert_eq!(
        report.validity.errors[0].kind,
        JsonInputErrorKind::UnsupportedType
    );
    assert_eq!(report.validity.errors[0].message, "Choose a valid option.");
}

#[test]
fn field_array_select_field_accepts_valid_option() {
    let fields = vec![FieldArrayObjectField::select(
        "type",
        "Type",
        vec![
            FieldArraySelectOption::same("string"),
            FieldArraySelectOption::same("boolean"),
        ],
    )];
    let rows = normalize_object_rows_initial(Some(json!([{ "type": "boolean" }])), &fields, true);
    let report = validate_object_rows(&rows.rows, &fields, None);

    assert!(report.validity.is_valid);
}

#[test]
fn field_array_deleted_rows_skip_select_validation() {
    let delete_config =
        FieldArrayDeleteConfig::from_behavior(&FieldArrayDeleteBehavior::mark_deleted());
    let fields = vec![
        FieldArrayObjectField::hidden("id", JsonValueType::Number),
        FieldArrayObjectField::select("type", "Type", vec![FieldArraySelectOption::same("string")]),
    ];
    let rows = normalize_object_rows_initial_with_delete(
        Some(json!([{ "id": 1, "type": "object", "_deleted": true }])),
        &fields,
        true,
        &delete_config,
    );
    let report = validate_object_rows(&rows.rows, &fields, None);

    assert!(report.validity.is_valid);
}

#[test]
fn field_array_required_trimmed_rejects_empty_and_whitespace_strings() {
    let fields = vec![
        FieldArrayObjectField::string("name", "Name").required_trimmed("Enter a trimmed value."),
    ];
    let rows = normalize_object_rows_initial(
        Some(json!([{ "name": "" }, { "name": "   " }])),
        &fields,
        true,
    );
    let report = validate_object_rows(&rows.rows, &fields, None);

    assert!(!report.validity.is_valid);
    assert_eq!(report.issues.len(), 2);
    assert!(
        report
            .issues
            .iter()
            .all(|issue| issue.message == "Enter a trimmed value.")
    );
}

#[test]
fn field_array_required_trimmed_accepts_non_empty_trimmed_string() {
    let fields = vec![
        FieldArrayObjectField::string("name", "Name").required_trimmed("Enter a trimmed value."),
    ];
    let rows =
        normalize_object_rows_initial(Some(json!([{ "name": " order_id " }])), &fields, true);
    let report = validate_object_rows(&rows.rows, &fields, None);

    assert!(report.validity.is_valid);
}

#[test]
fn field_array_pattern_validator_accepts_matching_value_and_rejects_non_match() {
    let fields = vec![
        FieldArrayObjectField::string("name", "Name")
            .pattern(r"^[A-Za-z][A-Za-z0-9_]*$", "Use an identifier."),
    ];
    let rows = normalize_object_rows_initial(
        Some(json!([{ "name": "order_id" }, { "name": "1bad" }])),
        &fields,
        true,
    );
    let report = validate_object_rows(&rows.rows, &fields, None);

    assert!(!report.validity.is_valid);
    assert_eq!(report.issues.len(), 1);
    assert_eq!(report.issues[0].row_index, 1);
    assert_eq!(report.issues[0].message, "Use an identifier.");
}

#[test]
fn field_array_invalid_pattern_does_not_panic() {
    let fields = vec![
        FieldArrayObjectField::string("name", "Name").pattern("[", "Pattern could not be applied."),
    ];
    let rows = normalize_object_rows_initial(Some(json!([{ "name": "order_id" }])), &fields, true);
    let report = validate_object_rows(&rows.rows, &fields, None);

    assert!(!report.validity.is_valid);
    assert_eq!(report.issues[0].message, "Pattern could not be applied.");
}

#[test]
fn field_array_custom_field_validator_can_return_inline_issue() {
    let fields = vec![
        FieldArrayObjectField::string("name", "Name").validate_field(Callback::from(
            |context: FieldArrayFieldValidationContext| {
                (context.value.as_str() == Some("reserved")).then(|| "Name is reserved.".into())
            },
        )),
    ];
    let rows = normalize_object_rows_initial(Some(json!([{ "name": "reserved" }])), &fields, true);
    let report = validate_object_rows(&rows.rows, &fields, None);

    assert!(!report.validity.is_valid);
    assert_eq!(report.issues[0].key.as_deref(), Some("name"));
    assert_eq!(report.issues[0].message, "Name is reserved.");
}

#[test]
fn field_array_deleted_rows_skip_field_level_validators() {
    let delete_config =
        FieldArrayDeleteConfig::from_behavior(&FieldArrayDeleteBehavior::mark_deleted());
    let fields = vec![
        FieldArrayObjectField::hidden("id", JsonValueType::Number),
        FieldArrayObjectField::string("name", "Name").required_trimmed("Enter a name."),
    ];
    let rows = normalize_object_rows_initial_with_delete(
        Some(json!([{ "id": 1, "name": "   ", "_deleted": true }])),
        &fields,
        true,
        &delete_config,
    );
    let report = validate_object_rows(&rows.rows, &fields, None);

    assert!(report.validity.is_valid);
}

#[test]
fn field_array_hidden_fields_skip_field_level_validators() {
    let fields = vec![
        FieldArrayObjectField::hidden("token", JsonValueType::String)
            .required_trimmed("Enter a token."),
    ];
    let rows = normalize_object_rows_initial(Some(json!([{ "token": "" }])), &fields, true);
    let report = validate_object_rows(&rows.rows, &fields, None);

    assert!(report.validity.is_valid);
}

#[test]
fn field_array_unique_field_flags_duplicate_active_rows() {
    let fields = vec![FieldArrayObjectField::string("name", "Name")];
    let rows = normalize_object_rows_initial(
        Some(json!([{ "name": "order_id" }, { "name": "order_id" }])),
        &fields,
        true,
    );
    let validators = vec![FieldArrayValidator::unique_field(
        "name",
        "Names must be unique.",
    )];
    let report =
        validate_object_rows_with_delete_and_custom(&rows.rows, &fields, None, &validators, None);

    assert!(!report.validity.is_valid);
    assert_eq!(report.issues.len(), 2);
    assert!(report.issues.iter().any(|issue| issue.row_index == 0));
    assert!(report.issues.iter().any(|issue| issue.row_index == 1));
}

#[test]
fn field_array_unique_field_ignores_deleted_rows_and_empty_values() {
    let delete_config =
        FieldArrayDeleteConfig::from_behavior(&FieldArrayDeleteBehavior::mark_deleted());
    let fields = vec![
        FieldArrayObjectField::hidden("id", JsonValueType::Number),
        FieldArrayObjectField::string("name", "Name"),
    ];
    let rows = normalize_object_rows_initial_with_delete(
        Some(json!([
            { "id": 1, "name": "order_id" },
            { "id": 2, "name": "order_id", "_deleted": true },
            { "name": "" },
            { "name": "" }
        ])),
        &fields,
        true,
        &delete_config,
    );
    let validators = vec![FieldArrayValidator::unique_field(
        "name",
        "Names must be unique.",
    )];
    let report =
        validate_object_rows_with_delete_and_custom(&rows.rows, &fields, None, &validators, None);

    assert!(report.validity.is_valid);
}

#[test]
fn field_array_unique_field_trimmed_treats_trimmed_values_as_duplicates() {
    let fields = vec![FieldArrayObjectField::string("name", "Name")];
    let rows = normalize_object_rows_initial(
        Some(json!([{ "name": "order_id" }, { "name": " order_id " }])),
        &fields,
        true,
    );
    let validators = vec![FieldArrayValidator::unique_field_trimmed(
        "name",
        "Names must be unique.",
    )];
    let report =
        validate_object_rows_with_delete_and_custom(&rows.rows, &fields, None, &validators, None);

    assert!(!report.validity.is_valid);
    assert_eq!(report.issues.len(), 2);
}

#[test]
fn field_array_unique_field_can_compare_case_insensitively() {
    let fields = vec![FieldArrayObjectField::string("name", "Name")];
    let rows = normalize_object_rows_initial(
        Some(json!([{ "name": "Order_ID" }, { "name": "order_id" }])),
        &fields,
        true,
    );
    let validators = vec![
        FieldArrayValidator::unique_field("name", "Names must be unique.").case_sensitive(false),
    ];
    let report =
        validate_object_rows_with_delete_and_custom(&rows.rows, &fields, None, &validators, None);

    assert!(!report.validity.is_valid);
    assert_eq!(report.issues.len(), 2);
}

#[test]
fn field_array_custom_array_validator_merges_with_report() {
    let fields = vec![FieldArrayObjectField::string("name", "Name")];
    let rows = normalize_object_rows_initial(Some(json!([{ "name": "order_id" }])), &fields, true);
    let validators = vec![FieldArrayValidator::custom(Callback::from(
        |_: FieldArrayValidationContext| {
            vec![FieldArrayCustomIssue::field(
                0,
                "name",
                "Custom array validator failed.",
            )]
        },
    ))];
    let report =
        validate_object_rows_with_delete_and_custom(&rows.rows, &fields, None, &validators, None);

    assert!(!report.validity.is_valid);
    assert_eq!(report.issues[0].message, "Custom array validator failed.");
    assert_eq!(report.validity.errors[0].path, "$[0].name");
}
