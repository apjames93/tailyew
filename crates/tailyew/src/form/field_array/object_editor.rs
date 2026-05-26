use super::FieldArrayModeProps;
use super::deleted_rows::{active_object_row_count, render_deleted_items_review};
use super::normalize::{new_object_row, normalize_object_rows_initial_with_delete};
use super::object_field::visible_object_fields;
use super::object_row::{ObjectRowRenderContext, render_object_row};
use super::props::FieldArrayDeleteConfig;
use super::serialize::value_from_object_rows_with_delete;
use super::styles::{
    field_array_add_action_class, field_array_add_container_class, field_array_body_class,
    field_array_count_class, field_array_empty_state_class, field_array_header_class,
    field_array_helper_class, field_array_list_class, field_array_preview_class,
    field_array_root_class, field_array_status_class, field_array_title_class,
    field_array_validation_summary_class,
};
use super::validation::{
    field_array_validation_report, friendly_validation_summary_items, issue_count_copy,
    validate_object_rows_with_delete_and_custom,
};
use crate::form::form_helpers::json_field_support::helper_with_warning;
use crate::form::json_input::form_bridge::{JsonBackedHiddenInput, JsonFormValidityBridge};
use crate::{AddIcon, Button, ButtonSize, ButtonType};
use serde_json::Value;
use yew::prelude::*;

#[component(ObjectFieldArray)]
pub(super) fn object_field_array(props: &FieldArrayModeProps) -> Html {
    let props = &props.props;
    let fields = props.object_fields.as_deref().unwrap_or_default();
    let delete_config = FieldArrayDeleteConfig::from_behavior(&props.delete_behavior);
    let normalized = normalize_object_rows_initial_with_delete(
        props.initial_value.clone(),
        fields,
        props.preserve_unknown_fields,
        &delete_config,
    );
    let rows = use_state(|| normalized.rows.clone());
    let last_valid_json = use_state(|| {
        let report = validate_object_rows_with_delete_and_custom(
            &normalized.rows,
            fields,
            props.min_items,
            &props.validators,
            props.custom_validate.as_ref(),
        );
        if report.validity.is_valid {
            value_from_object_rows_with_delete(&normalized.rows, fields, &delete_config)
                .unwrap_or_else(|_| Value::Array(Vec::new()))
        } else {
            Value::Array(Vec::new())
        }
    });
    let has_mounted = use_mut_ref(|| false);
    let validation_requested = use_state(|| false);
    let report = validate_object_rows_with_delete_and_custom(
        &rows,
        fields,
        props.min_items,
        &props.validators,
        props.custom_validate.as_ref(),
    );
    let validation_report = field_array_validation_report(
        &props.id,
        &props.name,
        &props.label,
        &report,
        fields,
        &props.text,
    );

    {
        let rows_snapshot = (*rows).clone();
        let fields = fields.to_vec();
        let delete_config = delete_config.clone();
        let validity = report.validity.clone();
        let validation_report = validation_report.clone();
        let last_valid_json = last_valid_json.clone();
        let on_json_change = props.on_json_change.clone();
        let on_validity_change = props.on_validity_change.clone();
        let on_validation_report_change = props.on_validation_report_change.clone();
        let has_mounted = has_mounted.clone();

        use_effect_with(
            (rows_snapshot, validity, validation_report),
            move |(rows_snapshot, validity, validation_report)| {
                if validity.is_valid
                    && let Ok(next_json) =
                        value_from_object_rows_with_delete(rows_snapshot, &fields, &delete_config)
                {
                    last_valid_json.set(next_json.clone());

                    let mut mounted = has_mounted.borrow_mut();
                    if *mounted {
                        if let Some(on_json_change) = &on_json_change {
                            on_json_change.emit(next_json);
                        }
                    } else {
                        *mounted = true;
                    }
                } else {
                    *has_mounted.borrow_mut() = true;
                }

                if let Some(on_validity_change) = &on_validity_change {
                    on_validity_change.emit(validity.clone());
                }
                if let Some(on_validation_report_change) = &on_validation_report_change {
                    on_validation_report_change.emit(validation_report.clone());
                }
            },
        );
    }

    let serialized_value = if report.validity.is_valid {
        value_from_object_rows_with_delete(&rows, fields, &delete_config)
            .unwrap_or_else(|_| (*last_valid_json).clone())
    } else {
        (*last_valid_json).clone()
    };
    let serialized_json =
        serde_json::to_string(&serialized_value).unwrap_or_else(|_| "[]".to_owned());
    let visible_fields = visible_object_fields(fields);
    let active_row_count = active_object_row_count(&rows);
    let deleted_row_count = rows.iter().filter(|row| row.deleted).count();
    let add_disabled = props
        .max_items
        .is_some_and(|max_items| active_row_count >= max_items);
    let remove_disabled = !props.allow_remove
        || props
            .min_items
            .is_some_and(|min_items| active_row_count <= min_items);
    let helper_text = helper_with_warning(&props.helper_text, normalized.warning);
    let helper_id = helper_text
        .as_ref()
        .map(|_| AttrValue::from(format!("{}-helper", props.id.as_str())));
    let active_count_label = if active_row_count == 1 {
        props.text.item_label.clone()
    } else {
        props.text.item_label_plural.clone()
    };
    let validation_summary_items = friendly_validation_summary_items(&report, fields, &props.text);

    let on_add_row = {
        let rows = rows.clone();
        let fields = fields.to_vec();

        Callback::from(move |_| {
            if add_disabled {
                return;
            }

            let mut next_rows = (*rows).clone();
            next_rows.push(new_object_row(&fields));
            rows.set(next_rows);
        })
    };

    let preview_json =
        serde_json::to_string_pretty(&serialized_value).unwrap_or_else(|_| serialized_json.clone());

    html! {
        <section
            class={field_array_root_class()}
            aria-describedby={helper_id.clone()}
        >
            <div class={field_array_header_class()}>
                <div class="flex flex-col gap-2 sm:flex-row sm:items-start sm:justify-between">
                    <div>
                        if !props.label.is_empty() {
                            <h3 class={field_array_title_class()}>
                                { props.label.clone() }
                            </h3>
                        }

                        if let Some(helper_text) = &helper_text {
                            <p id={helper_id} class={field_array_helper_class()}>
                                { helper_text.clone() }
                            </p>
                        }
                    </div>

                    <span class={field_array_count_class()}>
                        { format!("{active_row_count} {active_count_label}") }
                    </span>
                </div>
            </div>

            <div class={field_array_body_class()}>
                if !validation_summary_items.is_empty() {
                    <div class={field_array_validation_summary_class()} role="alert">
                        <p class="font-medium">
                            { issue_count_copy(validation_summary_items.len()) }
                        </p>
                        <ul class="mt-1 space-y-1 text-xs">
                            { for validation_summary_items.iter().take(4).map(|message| html! {
                                <li>
                                    { message.clone() }
                                </li>
                            }) }
                        </ul>
                    </div>
                }

                <div class={field_array_list_class()}>
                    if active_row_count == 0 {
                        <div class={field_array_empty_state_class()}>
                            <p class="font-medium text-gray-800 dark:text-gray-100">
                                { format!("No {} yet.", props.text.item_label_plural) }
                            </p>
                            <p class="mt-1 text-gray-600 dark:text-gray-400">
                                { "Add one to get started." }
                            </p>
                        </div>
                    } else {
                        <div class="space-y-3">
                            { for rows.iter().enumerate().filter(|(_, row)| {
                                !row.deleted || !delete_config.hide_marked_rows
                            }).map(|(row_index, row)| {
                                render_object_row(ObjectRowRenderContext {
                                    row_index,
                                    row,
                                    visible_fields: &visible_fields,
                                    issues: &report.issues,
                                    rows: rows.clone(),
                                    delete_config: &delete_config,
                                    remove_disabled,
                                    show_all_validation: *validation_requested,
                                })
                            }) }
                        </div>
                    }

                    if deleted_row_count > 0 && delete_config.hide_marked_rows {
                        { render_deleted_items_review(&rows, fields, &delete_config, &props.text) }
                    }

                    <div class={field_array_add_container_class()}>
                        <Button
                            button_type={ButtonType::Ghost}
                            size={ButtonSize::Small}
                            disabled={add_disabled}
                            on_click={on_add_row}
                            class={field_array_add_action_class()}
                            aria_label={Some(props.add_label.clone())}
                        >
                            <AddIcon size={16} decorative=true />
                            <span>{ props.add_label.clone() }</span>
                        </Button>
                        if add_disabled {
                            <span class="text-xs text-gray-500 dark:text-gray-400">
                                { "Maximum items reached." }
                            </span>
                        }
                    </div>
                </div>

                if report.validity.is_valid {
                    <div class={field_array_status_class(true)} role="status">
                        { props.text.valid_status.clone() }
                    </div>
                } else {
                    <div class={field_array_status_class(false)} role="status">
                        { props.text.invalid_status.clone() }
                    </div>
                }

                if props.show_json_preview {
                    <details class={field_array_preview_class()}>
                        <summary class="cursor-pointer px-3 py-2 text-sm font-medium text-gray-800 dark:text-gray-100">
                            { "JSON preview" }
                        </summary>
                        <div class="border-t border-gray-200 px-3 py-3 dark:border-gray-700">
                            <pre class="overflow-x-auto rounded bg-white p-3 font-mono text-xs text-gray-800 dark:bg-gray-900 dark:text-gray-100">
                                { preview_json }
                            </pre>
                        </div>
                    </details>
                }
            </div>

            <JsonBackedHiddenInput
                id={props.id.clone()}
                name={props.name.clone()}
                value={AttrValue::from(serialized_json)}
            />
            if props.block_form_submit_when_invalid {
                <JsonFormValidityBridge
                    id={props.id.clone()}
                    label={props.label.clone()}
                    is_valid={report.validity.is_valid}
                    on_validation_requested={Some({
                        let validation_requested = validation_requested.clone();
                        Callback::from(move |_| validation_requested.set(true))
                    })}
                />
            }
        </section>
    }
}
