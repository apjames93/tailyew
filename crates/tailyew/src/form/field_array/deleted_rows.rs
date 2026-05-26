use super::model::{FieldArrayFieldValue, FieldArrayObjectRow, json_value_has_identity};
use super::object_field::{FieldArrayObjectField, field_key};
use super::props::{FieldArrayDeleteConfig, FieldArrayText};
use crate::{Button, ButtonSize, ButtonType};
use serde_json::Value;
use uuid::Uuid;
use yew::prelude::*;

pub(crate) fn active_object_row_count(rows: &[FieldArrayObjectRow]) -> usize {
    rows.iter().filter(|row| !row.deleted).count()
}

pub(crate) fn remove_or_mark_object_row(
    rows: &mut Vec<FieldArrayObjectRow>,
    row_id: Uuid,
    delete_config: &FieldArrayDeleteConfig,
) -> bool {
    let Some(row_index) = rows.iter().position(|row| row.id == row_id) else {
        return false;
    };

    if delete_config.is_mark_deleted()
        && row_has_identity(&rows[row_index], delete_config.identity_key.as_deref())
    {
        rows[row_index].deleted = true;
        return true;
    }

    rows.remove(row_index);
    true
}

pub(crate) fn restore_object_row(rows: &UseStateHandle<Vec<FieldArrayObjectRow>>, row_id: Uuid) {
    let mut next_rows = (**rows).clone();
    if let Some(row) = next_rows.iter_mut().find(|row| row.id == row_id) {
        row.deleted = false;
    }
    rows.set(next_rows);
}

pub(crate) fn row_has_identity(row: &FieldArrayObjectRow, identity_key: Option<&str>) -> bool {
    let Some(identity_key) = identity_key else {
        return true;
    };

    if !row.source_keys.iter().any(|key| key == identity_key) {
        return false;
    }

    row.values
        .get(identity_key)
        .is_some_and(FieldArrayFieldValue::has_identity_value)
        || row
            .unknown_values
            .get(identity_key)
            .is_some_and(json_value_has_identity)
}

pub(crate) fn object_row_summary(
    row: &FieldArrayObjectRow,
    fields: &[FieldArrayObjectField],
    identity_key: Option<&str>,
    fallback_index: usize,
) -> String {
    for field in fields.iter().filter(|field| !field.hidden) {
        let key = field_key(field);
        if let Some(FieldArrayFieldValue::String(value)) = row.values.get(&key)
            && !value.trim().is_empty()
        {
            return value.clone();
        }
    }

    if let Some(identity_key) = identity_key
        && let Some(value) = row
            .values
            .get(identity_key)
            .and_then(|value| value.to_value().ok())
            .or_else(|| row.unknown_values.get(identity_key).cloned())
        && json_value_has_identity(&value)
    {
        return format!("{identity_key}: {}", value_for_summary(&value));
    }

    format!("Item {}", fallback_index + 1)
}

pub(crate) fn render_deleted_items_review(
    rows: &UseStateHandle<Vec<FieldArrayObjectRow>>,
    fields: &[FieldArrayObjectField],
    delete_config: &FieldArrayDeleteConfig,
    text: &FieldArrayText,
) -> Html {
    let deleted_rows = rows
        .iter()
        .enumerate()
        .filter(|(_, row)| row.deleted)
        .collect::<Vec<_>>();
    let deleted_count = deleted_rows.len();
    let count_copy = if deleted_count == 1 {
        format!("1 {} marked for removal", text.item_label)
    } else {
        format!(
            "{deleted_count} {} marked for removal",
            text.item_label_plural
        )
    };

    html! {
        <div class="bg-amber-50/60 px-2 py-2 text-xs dark:bg-amber-950/30">
            <div class="flex flex-col gap-2 sm:flex-row sm:items-center sm:justify-between">
                <p class="text-sm font-medium text-amber-900 dark:text-amber-100">
                    { count_copy }
                </p>
                if delete_config.allow_restore {
                    <p class="text-xs text-amber-800 dark:text-amber-200">
                        { text.deleted_rows_description.clone() }
                    </p>
                }
            </div>

            if delete_config.allow_restore {
                <div class="mt-2 flex flex-wrap gap-2">
                    { for deleted_rows.into_iter().map(|(row_index, row)| {
                        let row_id = row.id;
                        let rows = rows.clone();
                        let summary = object_row_summary(
                            row,
                            fields,
                            delete_config.identity_key.as_deref(),
                            row_index,
                        );
                        let aria_label = AttrValue::from(format!("Undo delete {summary}"));
                        let on_restore = Callback::from(move |_| {
                            restore_object_row(&rows, row_id);
                        });

                        html! {
                            <div class="inline-flex items-center gap-2 rounded-full border border-amber-200 bg-white px-2 py-1 text-xs text-amber-900 shadow-sm dark:border-amber-800 dark:bg-gray-900 dark:text-amber-100">
                                <span>{ summary }</span>
                                <Button
                                    button_type={ButtonType::Ghost}
                                    size={ButtonSize::Small}
                                    on_click={on_restore}
                                    class="h-6 px-2 text-xs shadow-none"
                                    aria_label={Some(aria_label)}
                                >
                                    { "Undo" }
                                </Button>
                            </div>
                        }
                    }) }
                </div>
            }
        </div>
    }
}

fn value_for_summary(value: &Value) -> String {
    match value {
        Value::String(value) => value.clone(),
        value => value.to_string(),
    }
}
