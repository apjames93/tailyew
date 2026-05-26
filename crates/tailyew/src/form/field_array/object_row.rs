use super::deleted_rows::{remove_or_mark_object_row, restore_object_row};
use super::model::{FieldArrayFieldValue, FieldArrayObjectIssue, FieldArrayObjectRow};
use super::object_field::{
    FieldArrayObjectField, FieldArrayObjectFieldEditor, default_value_for_field, field_key,
};
use super::props::FieldArrayDeleteConfig;
use super::styles::{field_array_row_class, object_fields_grid_class, object_row_action_class};
use crate::{
    AddIcon, Button, ButtonSize, ButtonType, Checkbox, DeleteIcon, Input, InputSize, InputType,
    Select, SelectOption, SelectSize,
};
use uuid::Uuid;
use yew::prelude::*;

pub(super) struct ObjectRowRenderContext<'a> {
    pub row_index: usize,
    pub row: &'a FieldArrayObjectRow,
    pub visible_fields: &'a [FieldArrayObjectField],
    pub issues: &'a [FieldArrayObjectIssue],
    pub rows: UseStateHandle<Vec<FieldArrayObjectRow>>,
    pub delete_config: &'a FieldArrayDeleteConfig,
    pub remove_disabled: bool,
    pub show_all_validation: bool,
}

pub(super) fn render_object_row(context: ObjectRowRenderContext<'_>) -> Html {
    let ObjectRowRenderContext {
        row_index,
        row,
        visible_fields,
        issues,
        rows,
        delete_config,
        remove_disabled,
        show_all_validation,
    } = context;
    let row_id = row.id;
    let row_deleted = row.deleted;
    let visible_row_issues = issues
        .iter()
        .filter(|issue| {
            issue.row_index == row_index
                && issue.key.is_none()
                && (show_all_validation || !row.touched_fields.is_empty())
        })
        .collect::<Vec<_>>();
    let action_disabled = if row_deleted {
        !delete_config.allow_restore
    } else {
        remove_disabled
    };
    let on_action = {
        let rows = rows.clone();
        let delete_config = delete_config.clone();

        Callback::from(move |_| {
            if row_deleted {
                if !delete_config.allow_restore {
                    return;
                }

                restore_object_row(&rows, row_id);
                return;
            }

            let mut next_rows = (*rows).clone();
            if !remove_or_mark_object_row(&mut next_rows, row_id, &delete_config) {
                return;
            }
            rows.set(next_rows);
        })
    };

    let row_class = field_array_row_class(row_deleted);

    html! {
        <div class={row_class}>
            <div class={object_row_action_class()}>
                { render_row_action_button(row_index, row_deleted, on_action.clone(), action_disabled) }
            </div>

            <div class={object_fields_grid_class()}>
                { for visible_fields.iter().map(|field| {
                    render_object_field(ObjectFieldRenderContext {
                        row_index,
                        row,
                        field,
                        issues,
                        rows: rows.clone(),
                        row_deleted,
                        show_all_validation,
                    })
                }) }
            </div>

            if !visible_row_issues.is_empty() {
                <div class="mt-2 space-y-1 rounded-md border border-red-200 bg-red-50 px-3 py-2 text-xs font-medium text-red-700 dark:border-red-900 dark:bg-red-950 dark:text-red-200">
                    { for visible_row_issues.iter().map(|issue| html! {
                        <p>{ issue.message.clone() }</p>
                    }) }
                </div>
            }

            if row_deleted {
                <p class="mt-2 text-xs font-medium text-amber-700 dark:text-amber-300">
                    { "Marked for removal" }
                </p>
            }
        </div>
    }
}

struct ObjectFieldRenderContext<'a> {
    row_index: usize,
    row: &'a FieldArrayObjectRow,
    field: &'a FieldArrayObjectField,
    issues: &'a [FieldArrayObjectIssue],
    rows: UseStateHandle<Vec<FieldArrayObjectRow>>,
    row_deleted: bool,
    show_all_validation: bool,
}

fn render_object_field(context: ObjectFieldRenderContext<'_>) -> Html {
    let ObjectFieldRenderContext {
        row_index,
        row,
        field,
        issues,
        rows,
        row_deleted,
        show_all_validation,
    } = context;
    let key = field_key(field);
    let field_id = format!("field-array-{}-{key}", row.id);
    let issue = issues.iter().find(|issue| {
        issue.row_index == row_index
            && issue.key.as_deref() == Some(key.as_str())
            && (show_all_validation || row.is_touched(&key))
    });
    let error_id = issue
        .as_ref()
        .map(|_| AttrValue::from(format!("{field_id}-error")));
    let value = row.values.get(&key).cloned().unwrap_or_else(|| {
        FieldArrayFieldValue::from_value(&default_value_for_field(field), field)
    });

    html! {
        <div class="min-w-0 space-y-1">
            <label
                for={field_id.clone()}
                class={classes!(
                    "block",
                    "text-xs",
                    "font-medium",
                    "text-gray-600",
                    "dark:text-gray-400",
                )}
            >
                { field.label.clone() }
            </label>
            {
                match value {
                    FieldArrayFieldValue::String(value) => {
                        if matches!(field.editor, FieldArrayObjectFieldEditor::Select { .. }) {
                            render_object_select_field(
                                TextObjectFieldRender {
                                    field_id,
                                    row_id: row.id,
                                    field,
                                    key,
                                    value,
                                    invalid: issue.is_some(),
                                    error_id: error_id.clone(),
                                    rows,
                                    disabled: row_deleted || !field.editable,
                                },
                            )
                        } else {
                            render_object_string_field(
                                TextObjectFieldRender {
                                    field_id,
                                    row_id: row.id,
                                    field,
                                    key,
                                    value,
                                    invalid: issue.is_some(),
                                    error_id: error_id.clone(),
                                    rows,
                                    disabled: row_deleted || !field.editable,
                                },
                            )
                        }
                    },
                    FieldArrayFieldValue::Number { raw } => render_object_number_field(
                        TextObjectFieldRender {
                            field_id,
                            row_id: row.id,
                            field,
                            key,
                            value: raw,
                            invalid: issue.is_some(),
                            error_id: error_id.clone(),
                            rows,
                            disabled: row_deleted || !field.editable,
                        },
                    ),
                    FieldArrayFieldValue::Boolean(value) => render_object_boolean_field(
                        row.id,
                        field,
                        key,
                        value,
                        rows,
                        row_deleted,
                    ),
                    FieldArrayFieldValue::Null => html! {
                        <div class="flex h-9 items-center rounded-md border border-gray-200 bg-gray-50 px-3 font-mono text-sm text-gray-600 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-300">
                            { "null" }
                        </div>
                    },
                    FieldArrayFieldValue::Json(value) => html! {
                        <div class="flex min-h-9 items-center rounded-md border border-gray-200 bg-gray-50 px-3 font-mono text-xs text-gray-600 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-300">
                            { serde_json::to_string(&value).unwrap_or_else(|_| "null".to_owned()) }
                        </div>
                    },
                }
            }
            if let Some(helper_text) = &field.helper_text {
                <p class="text-xs text-gray-500 dark:text-gray-400">{ helper_text.clone() }</p>
            }
            if let (Some(issue), Some(error_id)) = (issue, error_id) {
                <p id={error_id} class="text-xs font-medium text-red-600 dark:text-red-300">
                    { issue.message.clone() }
                </p>
            }
        </div>
    }
}

struct TextObjectFieldRender<'a> {
    field_id: String,
    row_id: Uuid,
    field: &'a FieldArrayObjectField,
    key: String,
    value: String,
    invalid: bool,
    error_id: Option<AttrValue>,
    rows: UseStateHandle<Vec<FieldArrayObjectRow>>,
    disabled: bool,
}

fn render_object_string_field(context: TextObjectFieldRender<'_>) -> Html {
    let TextObjectFieldRender {
        field_id,
        row_id,
        field,
        key,
        value,
        invalid,
        error_id,
        rows,
        disabled,
    } = context;
    let on_change = {
        let rows = rows.clone();
        let key = key.clone();

        Callback::from(move |next_value: String| {
            update_row_field(
                &rows,
                row_id,
                &key,
                FieldArrayFieldValue::String(next_value),
                true,
            );
        })
    };
    let on_blur = mark_touched_on_blur(rows.clone(), row_id, key.clone());

    html! {
        <Input
            id={field_id}
            label={field.label.clone()}
            value={Some(AttrValue::from(value))}
            placeholder={field.placeholder.clone().unwrap_or_default()}
            input_type={InputType::Text}
            size={InputSize::Small}
            visually_hidden_label={true}
            marginless={true}
            disabled={disabled}
            on_change={Some(on_change)}
            on_blur={Some(on_blur)}
            aria_invalid={Some(invalid)}
            aria_describedby={error_id}
            class={classes!(invalid.then_some("border-red-500"))}
        />
    }
}

fn render_object_select_field(context: TextObjectFieldRender<'_>) -> Html {
    let TextObjectFieldRender {
        field_id,
        row_id,
        field,
        key,
        value,
        invalid,
        error_id,
        rows,
        disabled,
    } = context;
    let options = match &field.editor {
        FieldArrayObjectFieldEditor::Select { options, .. } => options
            .iter()
            .filter(|option| !option.disabled)
            .map(|option| SelectOption {
                label: option.label.to_string(),
                value: option.value.to_string(),
            })
            .collect::<Vec<_>>(),
        FieldArrayObjectFieldEditor::Auto => Vec::new(),
    };
    let on_change = {
        let rows = rows.clone();
        let key = key.clone();

        Callback::from(move |next_value: String| {
            update_row_field(
                &rows,
                row_id,
                &key,
                FieldArrayFieldValue::String(next_value),
                true,
            );
        })
    };

    html! {
        <Select
            id={field_id}
            label={field.label.clone()}
            value={Some(AttrValue::from(value))}
            options={options}
            size={SelectSize::Small}
            visually_hidden_label={true}
            container_class={classes!("mb-0")}
            class={classes!(invalid.then_some("border-red-500"))}
            disabled={disabled}
            required={field.required}
            on_change={Some(on_change)}
            aria_invalid={Some(invalid)}
            aria_describedby={error_id}
        />
    }
}

fn render_object_number_field(context: TextObjectFieldRender<'_>) -> Html {
    let TextObjectFieldRender {
        field_id,
        row_id,
        field,
        key,
        value: raw,
        invalid,
        error_id,
        rows,
        disabled,
    } = context;
    let on_change = {
        let rows = rows.clone();
        let key = key.clone();

        Callback::from(move |next_value: String| {
            update_row_field(
                &rows,
                row_id,
                &key,
                FieldArrayFieldValue::Number { raw: next_value },
                true,
            );
        })
    };
    let on_blur = mark_touched_on_blur(rows.clone(), row_id, key.clone());

    html! {
        <Input
            id={field_id}
            label={field.label.clone()}
            value={Some(AttrValue::from(raw))}
            placeholder={field.placeholder.clone().unwrap_or_else(|| AttrValue::from("0"))}
            input_type={InputType::Text}
            size={InputSize::Small}
            visually_hidden_label={true}
            marginless={true}
            disabled={disabled}
            on_change={Some(on_change)}
            on_blur={Some(on_blur)}
            aria_invalid={Some(invalid)}
            aria_describedby={error_id}
            class={classes!(invalid.then_some("border-red-500"))}
        />
    }
}

fn render_object_boolean_field(
    row_id: Uuid,
    field: &FieldArrayObjectField,
    key: String,
    value: bool,
    rows: UseStateHandle<Vec<FieldArrayObjectRow>>,
    row_deleted: bool,
) -> Html {
    let on_change = {
        let rows = rows.clone();
        let key = key.clone();

        Callback::from(move |next_value: bool| {
            update_row_field(
                &rows,
                row_id,
                &key,
                FieldArrayFieldValue::Boolean(next_value),
                true,
            );
        })
    };

    html! {
        <div class="flex h-9 items-center px-1">
            <Checkbox
                id={format!("field-array-{row_id}-{key}")}
                label={if value { "true" } else { "false" }}
                checked={value}
                disabled={row_deleted || !field.editable}
                on_change={Some(on_change)}
                aria_label={Some(field.label.clone())}
            />
        </div>
    }
}

fn render_row_action_button(
    row_index: usize,
    row_deleted: bool,
    on_click: Callback<MouseEvent>,
    disabled: bool,
) -> Html {
    if row_deleted {
        return html! {
            <Button
                button_type={ButtonType::Ghost}
                size={ButtonSize::IconSmall}
                disabled={disabled}
                on_click={on_click}
                class="shadow-none"
                aria_label={Some(AttrValue::from(format!("Undo delete item {}", row_index + 1)))}
                title={Some(AttrValue::from("Undo delete"))}
            >
                <AddIcon size={14} decorative=true />
            </Button>
        };
    }

    html! {
        <Button
            button_type={ButtonType::DangerGhost}
            size={ButtonSize::IconSmall}
            disabled={disabled}
            on_click={on_click}
            class="shadow-none"
            aria_label={Some(AttrValue::from(format!("Remove item {}", row_index + 1)))}
            title={Some(AttrValue::from("Remove item"))}
        >
            <DeleteIcon size={14} decorative=true />
        </Button>
    }
}

fn mark_touched_on_blur(
    rows: UseStateHandle<Vec<FieldArrayObjectRow>>,
    row_id: Uuid,
    key: String,
) -> Callback<FocusEvent> {
    Callback::from(move |_| {
        let mut next_rows = (*rows).clone();
        if let Some(row) = next_rows.iter_mut().find(|row| row.id == row_id) {
            row.touch(&key);
        }
        rows.set(next_rows);
    })
}

fn update_row_field(
    rows: &UseStateHandle<Vec<FieldArrayObjectRow>>,
    row_id: Uuid,
    key: &str,
    value: FieldArrayFieldValue,
    touched: bool,
) {
    let mut next_rows = (**rows).clone();
    if let Some(row) = next_rows.iter_mut().find(|row| row.id == row_id) {
        row.values.insert(key.to_owned(), value);
        if touched {
            row.touch(key);
        }
    }
    rows.set(next_rows);
}
