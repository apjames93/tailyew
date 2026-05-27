use super::super::deleted_rows::{
    active_object_row_count, object_row_summary, remove_or_mark_object_row,
};
use super::super::normalize::{
    new_object_row, normalize_object_rows_initial, normalize_object_rows_initial_with_delete,
};
use super::super::props::FieldArrayDeleteConfig;
use super::super::serialize::{value_from_object_rows, value_from_object_rows_with_delete};
use super::super::validation::validate_object_rows;
use super::game_fields;
use crate::form::FieldArrayDeleteBehavior;
use serde_json::json;

#[test]
fn field_array_default_delete_behavior_hard_removes_rows() {
    let mut rows = normalize_object_rows_initial(
        Some(json!([
            { "id": 1, "name": "Resident Evil Requiem" },
            { "id": 2, "name": "Elden Ring" }
        ])),
        &game_fields(),
        true,
    )
    .rows;
    let removed_id = rows[0].id;

    assert!(remove_or_mark_object_row(
        &mut rows,
        removed_id,
        &FieldArrayDeleteConfig::remove()
    ));

    assert_eq!(rows.len(), 1);
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
fn field_array_initial_deleted_marker_excludes_row_from_active_count() {
    let delete_config =
        FieldArrayDeleteConfig::from_behavior(&FieldArrayDeleteBehavior::mark_deleted());
    let rows = normalize_object_rows_initial_with_delete(
        Some(json!([{ "id": 1, "name": "Resident Evil Requiem", "_deleted": true }])),
        &game_fields(),
        true,
        &delete_config,
    );

    assert_eq!(active_object_row_count(&rows.rows), 0);
}

#[test]
fn field_array_mark_deleted_hard_removes_new_rows_without_identity() {
    let delete_config =
        FieldArrayDeleteConfig::from_behavior(&FieldArrayDeleteBehavior::mark_deleted());
    let mut rows = vec![new_object_row(&game_fields())];
    let row_id = rows[0].id;

    assert!(remove_or_mark_object_row(&mut rows, row_id, &delete_config));

    assert!(rows.is_empty());
}

#[test]
fn field_array_mark_deleted_preserves_unknown_fields_on_deleted_rows() {
    let delete_config =
        FieldArrayDeleteConfig::from_behavior(&FieldArrayDeleteBehavior::mark_deleted());
    let mut rows = normalize_object_rows_initial_with_delete(
        Some(json!([{ "id": 1, "name": "Resident Evil Requiem", "platform": "PS5" }])),
        &game_fields(),
        true,
        &delete_config,
    )
    .rows;
    let row_id = rows[0].id;

    assert!(remove_or_mark_object_row(&mut rows, row_id, &delete_config));

    assert_eq!(
        value_from_object_rows_with_delete(&rows, &game_fields(), &delete_config).unwrap()[0]["platform"],
        json!("PS5")
    );
}

#[test]
fn field_array_active_counts_and_min_validation_ignore_deleted_rows() {
    let delete_config =
        FieldArrayDeleteConfig::from_behavior(&FieldArrayDeleteBehavior::mark_deleted());
    let mut rows = normalize_object_rows_initial_with_delete(
        Some(json!([
            { "id": 1, "name": "Resident Evil Requiem" },
            { "id": 2, "name": "Elden Ring" }
        ])),
        &game_fields(),
        true,
        &delete_config,
    )
    .rows;
    let row_id = rows[0].id;

    assert!(remove_or_mark_object_row(&mut rows, row_id, &delete_config));
    assert_eq!(active_object_row_count(&rows), 1);

    let report = validate_object_rows(&rows, &game_fields(), Some(2));
    assert!(!report.validity.is_valid);
}

#[test]
fn field_array_object_row_summary_prefers_visible_name_then_identity() {
    let delete_config =
        FieldArrayDeleteConfig::from_behavior(&FieldArrayDeleteBehavior::mark_deleted());
    let rows = normalize_object_rows_initial_with_delete(
        Some(json!([{ "id": 1, "name": "Resident Evil Requiem" }])),
        &game_fields(),
        true,
        &delete_config,
    );

    assert_eq!(
        object_row_summary(&rows.rows[0], &game_fields(), Some("id"), 0),
        "Resident Evil Requiem"
    );
}
