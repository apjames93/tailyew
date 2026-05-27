use super::super::extract::{FieldArrayRows, FieldArrayRowsError, FieldArrayRowsOptions};
use serde_json::json;

#[test]
fn field_array_rows_accepts_array_of_objects() {
    let rows = FieldArrayRows::from_value(&json!([
        { "name": "order_id" },
        { "name": "include_history" }
    ]))
    .unwrap();

    assert_eq!(rows.len(), 2);
    assert_eq!(rows.iter().next().unwrap().index, 0);
    assert_eq!(
        rows.iter().nth(1).unwrap().get("name"),
        Some(&json!("include_history"))
    );
}

#[test]
fn field_array_rows_rejects_non_array_values() {
    let error = FieldArrayRows::from_value(&json!({ "name": "order_id" })).unwrap_err();

    assert_eq!(error, FieldArrayRowsError::ExpectedArray);
    assert_eq!(
        error.to_string(),
        "Expected FieldArray value to be a JSON array."
    );
}

#[test]
fn field_array_rows_rejects_non_object_rows_with_index() {
    let error = FieldArrayRows::from_value(&json!([{ "name": "order_id" }, true])).unwrap_err();

    assert_eq!(error, FieldArrayRowsError::ExpectedObject { index: 1 });
    assert_eq!(error.to_string(), "Expected row 2 to be a JSON object.");
}

#[test]
fn field_array_rows_active_and_deleted_use_default_marker() {
    let rows = FieldArrayRows::from_value(&json!([
        { "name": "active" },
        { "name": "deleted", "_deleted": true },
        { "name": "false-marker", "_deleted": false }
    ]))
    .unwrap();

    assert_eq!(
        rows.active().map(|row| row.index).collect::<Vec<_>>(),
        vec![0, 2]
    );
    assert_eq!(
        rows.deleted().map(|row| row.index).collect::<Vec<_>>(),
        vec![1]
    );
}

#[test]
fn field_array_rows_active_and_deleted_support_custom_marker() {
    let rows = FieldArrayRows::from_value_with_options(
        &json!([
            { "name": "active" },
            { "name": "deleted", "remove": true }
        ]),
        FieldArrayRowsOptions {
            deleted_marker_key: "remove".to_owned(),
        },
    )
    .unwrap();

    assert_eq!(
        rows.active().map(|row| row.index).collect::<Vec<_>>(),
        vec![0]
    );
    assert_eq!(
        rows.deleted().map(|row| row.index).collect::<Vec<_>>(),
        vec![1]
    );
}

#[test]
fn field_array_row_required_string_returns_string() {
    let row = FieldArrayRows::from_value(&json!([{ "name": "order_id" }]))
        .unwrap()
        .into_vec()
        .remove(0);

    assert_eq!(row.required_string("name").unwrap(), "order_id");
}

#[test]
fn field_array_row_required_string_errors_on_missing_field() {
    let row = FieldArrayRows::from_value(&json!([{}]))
        .unwrap()
        .into_vec()
        .remove(0);
    let error = row.required_string("name").unwrap_err();

    assert_eq!(
        error,
        FieldArrayRowsError::MissingField {
            index: 0,
            key: "name".to_owned()
        }
    );
    assert_eq!(
        error.to_string(),
        "Missing required field \"name\" in row 1."
    );
}

#[test]
fn field_array_row_required_string_errors_on_non_string() {
    let row = FieldArrayRows::from_value(&json!([{ "name": 123 }]))
        .unwrap()
        .into_vec()
        .remove(0);
    let error = row.required_string("name").unwrap_err();

    assert_eq!(
        error,
        FieldArrayRowsError::InvalidType {
            index: 0,
            key: "name".to_owned(),
            expected: "a string"
        }
    );
}

#[test]
fn field_array_row_optional_string_returns_none_when_missing() {
    let row = FieldArrayRows::from_value(&json!([{}]))
        .unwrap()
        .into_vec()
        .remove(0);

    assert_eq!(row.optional_string("name").unwrap(), None);
}

#[test]
fn field_array_row_bool_accessors_read_boolean_values() {
    let row = FieldArrayRows::from_value(&json!([{ "required": true }]))
        .unwrap()
        .into_vec()
        .remove(0);

    assert!(row.required_bool("required").unwrap());
    assert_eq!(row.optional_bool("missing").unwrap(), None);
}

#[test]
fn field_array_row_number_accessors_read_json_numbers() {
    let row = FieldArrayRows::from_value(&json!([{ "count": 3, "score": 4.5 }]))
        .unwrap()
        .into_vec()
        .remove(0);

    assert_eq!(row.required_i64("count").unwrap(), 3);
    assert_eq!(row.optional_i64("missing").unwrap(), None);
    assert_eq!(row.required_f64("score").unwrap(), 4.5);
    assert_eq!(row.optional_f64("missing").unwrap(), None);
}

#[test]
fn field_array_row_value_accessors_clone_json_values() {
    let row = FieldArrayRows::from_value(&json!([{ "metadata": { "source": "manual" } }]))
        .unwrap()
        .into_vec()
        .remove(0);

    assert_eq!(
        row.required_value("metadata").unwrap(),
        json!({ "source": "manual" })
    );
    assert_eq!(row.optional_value("missing"), None);
}

#[test]
fn field_array_row_required_string_enum_accepts_allowed_value() {
    let row = FieldArrayRows::from_value(&json!([{ "type": "boolean" }]))
        .unwrap()
        .into_vec()
        .remove(0);

    assert_eq!(
        row.required_string_enum("type", &["string", "number", "integer", "boolean"])
            .unwrap(),
        "boolean"
    );
}

#[test]
fn field_array_row_required_string_enum_rejects_disallowed_value() {
    let row = FieldArrayRows::from_value(&json!([{ "type": "date" }]))
        .unwrap()
        .into_vec()
        .remove(0);
    let error = row
        .required_string_enum("type", &["string", "number", "integer", "boolean"])
        .unwrap_err();

    assert_eq!(
        error,
        FieldArrayRowsError::InvalidEnum {
            index: 0,
            key: "type".to_owned(),
            allowed: vec![
                "string".to_owned(),
                "number".to_owned(),
                "integer".to_owned(),
                "boolean".to_owned()
            ]
        }
    );
    assert_eq!(
        error.to_string(),
        "Expected field \"type\" in row 1 to be one of: string, number, integer, boolean."
    );
}

#[test]
fn field_array_row_optional_string_enum_returns_none_when_missing() {
    let row = FieldArrayRows::from_value(&json!([{}]))
        .unwrap()
        .into_vec()
        .remove(0);

    assert_eq!(
        row.optional_string_enum("type", &["string", "number"])
            .unwrap(),
        None
    );
}
