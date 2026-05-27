use super::super::normalize::{
    normalize_object_rows_initial, normalize_object_rows_initial_with_delete,
};
use super::super::object_field::{FieldArrayObjectField, FieldArraySelectOption};
use super::super::props::FieldArrayDeleteConfig;
use super::super::serialize::{value_from_object_rows, value_from_object_rows_with_delete};
use super::game_fields;
use crate::form::{FieldArrayDeleteBehavior, JsonValueType};
use serde_json::json;

#[test]
fn field_array_object_mode_boolean_serializes_as_bool() {
    let rows = normalize_object_rows_initial(
        Some(json!([{ "id": 1, "name": "Elden Ring", "beat": true }])),
        &game_fields(),
        true,
    );

    assert_eq!(
        value_from_object_rows(&rows.rows, &game_fields()).unwrap()[0]["beat"],
        json!(true)
    );
}

#[test]
fn field_array_select_value_serializes_as_json_string() {
    let fields = vec![FieldArrayObjectField::select(
        "type",
        "Type",
        vec![
            FieldArraySelectOption::same("string"),
            FieldArraySelectOption::same("integer"),
        ],
    )];
    let rows = normalize_object_rows_initial(Some(json!([{ "type": "integer" }])), &fields, true);

    assert_eq!(
        value_from_object_rows(&rows.rows, &fields).unwrap(),
        json!([{ "type": "integer" }])
    );
}

#[test]
fn field_array_object_mode_removing_row_preserves_remaining_ids() {
    let mut rows = normalize_object_rows_initial(
        Some(json!([
            { "id": 1, "name": "Resident Evil Requiem" },
            { "id": 2, "name": "Elden Ring" }
        ])),
        &game_fields(),
        true,
    )
    .rows;
    rows.remove(0);

    assert_eq!(
        value_from_object_rows(&rows, &game_fields()).unwrap(),
        json!([
            {
                "id": 2,
                "name": "Elden Ring",
                "hours_played": "",
                "beat": false
            }
        ])
    );
}

#[test]
fn field_array_mark_deleted_keeps_existing_row_with_marker_and_hidden_id() {
    let delete_config =
        FieldArrayDeleteConfig::from_behavior(&FieldArrayDeleteBehavior::mark_deleted());
    let mut rows = normalize_object_rows_initial_with_delete(
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
        &delete_config,
    )
    .rows;
    rows[0].deleted = true;

    assert_eq!(
        value_from_object_rows_with_delete(&rows, &game_fields(), &delete_config).unwrap(),
        json!([
            {
                "id": 1,
                "name": "Resident Evil Requiem",
                "hours_played": "3",
                "beat": false,
                "_deleted": true
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
fn field_array_mark_deleted_omits_false_marker_for_active_rows() {
    let delete_config =
        FieldArrayDeleteConfig::from_behavior(&FieldArrayDeleteBehavior::mark_deleted());
    let rows = normalize_object_rows_initial_with_delete(
        Some(json!([{ "id": 1, "name": "Resident Evil Requiem", "_deleted": false }])),
        &game_fields(),
        true,
        &delete_config,
    );

    assert_eq!(
        value_from_object_rows_with_delete(&rows.rows, &game_fields(), &delete_config).unwrap(),
        json!([
            {
                "id": 1,
                "name": "Resident Evil Requiem",
                "hours_played": "",
                "beat": false
            }
        ])
    );
}

#[test]
fn field_array_deleted_rows_skip_invalid_field_serialization() {
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

    assert_eq!(
        value_from_object_rows_with_delete(&rows.rows, &fields, &delete_config).unwrap(),
        json!([{ "id": 1, "name": "", "_deleted": true }])
    );
}

#[test]
fn field_array_undo_delete_removes_marker_from_serialized_output() {
    let delete_config =
        FieldArrayDeleteConfig::from_behavior(&FieldArrayDeleteBehavior::mark_deleted());
    let mut rows = normalize_object_rows_initial_with_delete(
        Some(json!([{ "id": 1, "name": "Resident Evil Requiem", "_deleted": true }])),
        &game_fields(),
        true,
        &delete_config,
    )
    .rows;

    rows[0].deleted = false;

    assert_eq!(
        value_from_object_rows_with_delete(&rows, &game_fields(), &delete_config).unwrap(),
        json!([
            {
                "id": 1,
                "name": "Resident Evil Requiem",
                "hours_played": "",
                "beat": false
            }
        ])
    );
}
