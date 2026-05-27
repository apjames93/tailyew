use super::super::normalize::{
    new_object_row, normalize_object_rows_initial, normalize_object_rows_initial_with_delete,
};
use super::super::object_field::{FieldArrayObjectField, FieldArraySelectOption};
use super::super::props::FieldArrayDeleteConfig;
use super::super::serialize::value_from_object_rows;
use super::game_fields;
use crate::form::{FieldArrayDeleteBehavior, JsonValueType};
use serde_json::json;

#[test]
fn field_array_object_mode_preserves_initial_objects_and_hidden_ids() {
    let rows = normalize_object_rows_initial(
        Some(json!([
            {
                "id": 1,
                "name": "Resident Evil Requiem",
                "hours_played": "3",
                "beat": false
            },
            {
                "id": 2,
                "name": "Elden Ring",
                "hours_played": "100",
                "beat": true
            }
        ])),
        &game_fields(),
        true,
    );

    assert!(rows.warning.is_none());
    assert_eq!(
        value_from_object_rows(&rows.rows, &game_fields()).unwrap(),
        json!([
            {
                "id": 1,
                "name": "Resident Evil Requiem",
                "hours_played": "3",
                "beat": false
            },
            {
                "id": 2,
                "name": "Elden Ring",
                "hours_played": "100",
                "beat": true
            }
        ])
    );
}

#[test]
fn field_array_object_mode_preserves_unknown_fields_by_default() {
    let rows = normalize_object_rows_initial(
        Some(json!([
            {
                "id": 1,
                "name": "Resident Evil Requiem",
                "platform": "PS5"
            }
        ])),
        &game_fields(),
        true,
    );

    assert_eq!(
        value_from_object_rows(&rows.rows, &game_fields()).unwrap(),
        json!([
            {
                "id": 1,
                "name": "Resident Evil Requiem",
                "hours_played": "",
                "beat": false,
                "platform": "PS5"
            }
        ])
    );
}

#[test]
fn field_array_object_mode_new_row_uses_defaults() {
    let fields = vec![
        FieldArrayObjectField::hidden("id", JsonValueType::Number).default_value(json!(999)),
        FieldArrayObjectField::string("name", "Game")
            .required(true)
            .default_value(json!("Untitled")),
        FieldArrayObjectField::boolean("beat", "Beat").default_value(json!(true)),
    ];
    let row = new_object_row(&fields);

    assert_eq!(
        value_from_object_rows(&[row], &fields).unwrap(),
        json!([
            {
                "id": 999,
                "name": "Untitled",
                "beat": true
            }
        ])
    );
}

#[test]
fn field_array_object_mode_new_row_uses_select_default() {
    let fields = vec![FieldArrayObjectField::select(
        "type",
        "Type",
        vec![
            FieldArraySelectOption::same("string"),
            FieldArraySelectOption::same("boolean"),
        ],
    )];
    let row = new_object_row(&fields);

    assert_eq!(
        value_from_object_rows(&[row], &fields).unwrap(),
        json!([{ "type": "string" }])
    );
}

#[test]
fn field_array_object_mode_preserves_initial_select_value() {
    let fields = vec![FieldArrayObjectField::select(
        "type",
        "Type",
        vec![
            FieldArraySelectOption::same("string"),
            FieldArraySelectOption::same("boolean"),
        ],
    )];
    let rows = normalize_object_rows_initial(Some(json!([{ "type": "boolean" }])), &fields, true);

    assert_eq!(
        value_from_object_rows(&rows.rows, &fields).unwrap(),
        json!([{ "type": "boolean" }])
    );
}

#[test]
fn field_array_object_mode_normalizes_non_object_items() {
    let rows = normalize_object_rows_initial(Some(json!(["not object"])), &game_fields(), true);

    assert_eq!(rows.rows.len(), 1);
    assert!(rows.warning.is_some());
}

#[test]
fn field_array_initial_deleted_marker_normalizes_as_deleted() {
    let delete_config =
        FieldArrayDeleteConfig::from_behavior(&FieldArrayDeleteBehavior::mark_deleted());
    let rows = normalize_object_rows_initial_with_delete(
        Some(json!([{ "id": 1, "name": "Resident Evil Requiem", "_deleted": true }])),
        &game_fields(),
        true,
        &delete_config,
    );

    assert!(rows.rows[0].deleted);
}
