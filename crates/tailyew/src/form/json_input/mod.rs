mod array_editor;
mod array_item_row;
mod controls;
pub(crate) mod form_bridge;
mod model;
mod object_editor;
mod policy;
pub(crate) mod presets;
mod property_row;
mod types;
mod validation;
mod validation_report;
mod validation_summary;
mod value_editor;

#[cfg(test)]
mod tests;

use self::array_editor::JsonArrayEditor;
use self::controls::{
    JsonRowKind, render_type_select, root_scalar_grid_columns_class, show_type_column_for_path,
};
use self::form_bridge::{JsonBackedHiddenInput, JsonFormValidityBridge};
use self::model::{model_from_value, summarize_value, value_from_model};
use self::object_editor::JsonObjectEditor;
use self::types::*;
use self::validation::validate_model_report;
use self::validation_report::validation_report_from_json_input_validity;
use self::validation_summary::JsonValidationSummary;
use self::value_editor::JsonValueEditor;
use crate::{Button, ButtonSize, ButtonType, Textarea};
use serde_json::{Map, Value};
use yew::prelude::*;

pub use self::types::{
    JsonInputDensity, JsonInputError, JsonInputErrorKind, JsonInputPath, JsonInputPathPolicy,
    JsonInputPathSegment, JsonInputProps, JsonInputValidity, JsonValidationMode, JsonValueType,
};
pub use self::validation_report::{
    JsonBackedFormReportsHandle, JsonBackedValidationIssue, JsonBackedValidationReport,
    JsonBackedValidationSummaryEntry, summary_entries_from_report, summary_entries_from_reports,
    use_json_backed_form_reports, use_json_backed_form_reports_with_sections,
};

/// Edits arbitrary JSON and submits the current valid value through a hidden input.
#[component(JsonInput)]
pub fn json_input(props: &JsonInputProps) -> Html {
    let config = JsonInputConfig::from_props(props);
    let initial_model = {
        let initial = props
            .initial_value
            .clone()
            .unwrap_or_else(|| Value::Object(Map::new()));
        model_from_value(&initial)
    };
    let initial_valid_json =
        value_from_model(&initial_model).unwrap_or_else(|_| Value::Object(Map::new()));

    let model = use_state(|| initial_model);
    let last_valid_json = use_state(|| initial_valid_json);
    let local_validation_requested = use_state(|| false);
    let has_mounted = use_mut_ref(|| false);
    let paste_open = use_state(|| false);
    let paste_value = use_state(String::new);
    let paste_error = use_state(|| None::<String>);

    let full_report =
        validate_model_report(&model, &config, ValidationVisibility::All, "$".to_owned());
    let show_all_validation = should_show_all_validation(
        props.validation_mode,
        props.validation_requested,
        props.validation_request_id,
        *local_validation_requested,
    );
    let visible_report = validate_model_report(
        &model,
        &config,
        if show_all_validation {
            ValidationVisibility::All
        } else {
            ValidationVisibility::Touched
        },
        "$".to_owned(),
    );

    {
        let model_snapshot = (*model).clone();
        let validity = full_report.validity.clone();
        let validation_report = validation_report_from_json_input_validity(
            &props.id,
            &props.name,
            &props.label,
            &validity,
        );
        let last_valid_json = last_valid_json.clone();
        let on_json_change = props.on_json_change.clone();
        let on_validity_change = props.on_validity_change.clone();
        let on_validation_report_change = props.on_validation_report_change.clone();
        let has_mounted = has_mounted.clone();

        use_effect_with(
            (model_snapshot, validity, validation_report),
            move |(model_snapshot, validity, validation_report)| {
                if validity.is_valid
                    && let Ok(next_json) = value_from_model(model_snapshot)
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

    let update_model = {
        let model = model.clone();
        Callback::from(move |updater: ModelUpdater| {
            let mut next_model = (*model).clone();
            updater(&mut next_model);
            model.set(next_model);
        })
    };

    let serialized_json = hidden_json_value(&model, &last_valid_json, &full_report.validity);
    let serialized_json_string =
        serde_json::to_string(&serialized_json).unwrap_or_else(|_| "{}".to_owned());

    let helper_id = props
        .helper_text
        .as_ref()
        .map(|_| AttrValue::from(format!("{}-helper", props.id.as_str())));
    let preview_json = serde_json::to_string_pretty(&serialized_json)
        .unwrap_or_else(|_| serialized_json_string.clone());

    let on_paste_toggle = {
        let paste_open = paste_open.clone();
        Callback::from(move |_| paste_open.set(!*paste_open))
    };
    let on_paste_input = {
        let paste_value = paste_value.clone();
        Callback::from(move |value: String| {
            paste_value.set(value);
        })
    };
    let on_apply_paste = {
        let model = model.clone();
        let paste_value = paste_value.clone();
        let paste_error = paste_error.clone();
        let local_validation_requested = local_validation_requested.clone();
        Callback::from(move |_| match parse_pasted_json(&paste_value) {
            Ok(next_model) => {
                model.set(next_model);
                paste_error.set(None);
                local_validation_requested.set(false);
            }
            Err(err) => paste_error.set(Some(err)),
        })
    };

    html! {
        <section
            class="rounded-xl border border-gray-200 bg-white shadow-sm dark:border-gray-700 dark:bg-gray-900"
            aria-describedby={helper_id.clone()}
        >
            <div class="border-b border-gray-200 px-4 py-4 dark:border-gray-700 sm:px-5">
                <div class="flex flex-col gap-2 sm:flex-row sm:items-start sm:justify-between">
                    <div>
                        if !props.label.as_str().trim().is_empty() {
                            <h3 class="text-base font-semibold text-gray-900 dark:text-gray-100">
                                { props.label.clone() }
                            </h3>
                        }

                        if let Some(helper_text) = &props.helper_text {
                            <p id={helper_id} class="mt-1 max-w-3xl text-sm text-gray-600 dark:text-gray-400">
                                { helper_text }
                            </p>
                        }
                    </div>

                    <span class="inline-flex w-fit items-center rounded-full bg-gray-100 px-2.5 py-1 text-xs font-medium text-gray-700 dark:bg-gray-800 dark:text-gray-300">
                        { summarize_value(&model) }
                    </span>
                </div>
            </div>

            <div class="space-y-4 px-4 py-4 sm:px-5">
                if !visible_report.validity.errors.is_empty() {
                    <JsonValidationSummary validity={visible_report.validity.clone()} />
                }

                { render_root_editor(&model, &config, visible_report.issues.clone(), update_model.clone()) }

                { render_submission_status(&full_report.validity, &visible_report.validity, show_all_validation) }

                if props.allow_raw_json_paste {
                    <div class="border-t border-gray-200 pt-4 dark:border-gray-700">
                        <Button
                            button_type={ButtonType::Ghost}
                            size={ButtonSize::Small}
                            on_click={on_paste_toggle}
                            aria_expanded={Some(AttrValue::from(paste_open.to_string()))}
                            aria_controls={Some(AttrValue::from(format!("{}-paste-json", props.id.as_str())))}
                        >
                            { props.paste_label.clone() }
                        </Button>

                        if *paste_open {
                            <div id={format!("{}-paste-json", props.id.as_str())} class="mt-3 space-y-2">
                                if let Some(paste_helper_text) = &props.paste_helper_text {
                                    <p class="text-xs text-gray-600 dark:text-gray-400">
                                        { paste_helper_text.clone() }
                                    </p>
                                }
                                <Textarea
                                    id={format!("{}-paste-json-input", props.id.as_str())}
                                    label={props.paste_label.clone()}
                                    value={Some(AttrValue::from((*paste_value).clone()))}
                                    on_change={Some(on_paste_input)}
                                    placeholder={props.paste_placeholder.clone()}
                                    rows={6}
                                    visually_hidden_label={true}
                                    container_class="space-y-2"
                                    class="min-h-32 w-full font-mono text-sm text-gray-900 dark:text-gray-100"
                                    error={(*paste_error).as_ref().map(|error| AttrValue::from(error.clone()))}
                                />
                                <Button
                                    button_type={ButtonType::Primary}
                                    size={ButtonSize::Small}
                                    on_click={on_apply_paste}
                                >
                                    { props.apply_paste_label.clone() }
                                </Button>
                            </div>
                        }
                    </div>
                }

                if props.show_json_preview {
                    <details class="rounded-md border border-gray-200 bg-gray-50 dark:border-gray-700 dark:bg-gray-800">
                        <summary class="cursor-pointer px-3 py-2 text-sm font-medium text-gray-800 dark:text-gray-100">
                            { "JSON preview" }
                        </summary>
                        <div class="border-t border-gray-200 px-3 py-3 dark:border-gray-700">
                            <p class="mb-2 text-xs text-gray-600 dark:text-gray-400">
                                { preview_status_copy(&full_report.validity, &visible_report.validity) }
                            </p>
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
                value={AttrValue::from(serialized_json_string)}
            />
            if props.block_form_submit_when_invalid {
                <JsonFormValidityBridge
                    id={props.id.clone()}
                    label={props.label.clone()}
                    is_valid={full_report.validity.is_valid}
                    on_validation_requested={Some({
                        let local_validation_requested = local_validation_requested.clone();
                        Callback::from(move |_| local_validation_requested.set(true))
                    })}
                />
            }
        </section>
    }
}

fn parse_pasted_json(raw_json: &str) -> Result<JsonModel, String> {
    serde_json::from_str::<Value>(raw_json)
        .map(|value| model_from_value(&value))
        .map_err(|err| format!("Invalid JSON: {err}"))
}

fn render_submission_status(
    full_validity: &JsonInputValidity,
    visible_validity: &JsonInputValidity,
    show_all_validation: bool,
) -> Html {
    let Some((class, message)) = (if full_validity.is_valid {
        Some((
            "rounded-md border border-gray-200 bg-gray-50 px-3 py-2 text-xs text-gray-600 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-300",
            "Current draft will be submitted.",
        ))
    } else if visible_validity.is_valid && !show_all_validation {
        None
    } else {
        Some((
            "rounded-md border border-amber-200 bg-amber-50 px-3 py-2 text-xs text-amber-800 dark:border-amber-900 dark:bg-amber-950 dark:text-amber-200",
            "Fix the highlighted fields before submitting.",
        ))
    }) else {
        return Html::default();
    };

    html! {
        <div class={class} role="status">
            { message }
        </div>
    }
}

fn should_show_all_validation(
    validation_mode: JsonValidationMode,
    validation_requested: bool,
    validation_request_id: Option<u64>,
    local_validation_requested: bool,
) -> bool {
    validation_mode == JsonValidationMode::Always
        || validation_requested
        || validation_request_id.is_some()
        || local_validation_requested
}

fn hidden_json_value(
    model: &JsonModel,
    last_valid_json: &Value,
    full_validity: &JsonInputValidity,
) -> Value {
    if full_validity.is_valid {
        value_from_model(model).unwrap_or_else(|_| last_valid_json.clone())
    } else {
        last_valid_json.clone()
    }
}

fn preview_status_copy(
    full_validity: &JsonInputValidity,
    visible_validity: &JsonInputValidity,
) -> &'static str {
    if full_validity.is_valid {
        "Preview reflects the JSON that will be submitted."
    } else if visible_validity.is_valid {
        "Preview will update after incomplete rows are fixed."
    } else {
        "Preview will update after validation errors are fixed."
    }
}

fn render_root_editor(
    model: &JsonModel,
    config: &JsonInputConfig,
    issues: Vec<JsonInputIssue>,
    update_model: Callback<ModelUpdater>,
) -> Html {
    match &model.kind {
        JsonNodeKind::Object(_) => html! {
            <JsonObjectEditor
                node={model.clone()}
                path={Vec::<JsonPathSegment>::new()}
                policy_path={JsonInputPath::root()}
                depth={0}
                config={config.clone()}
                issues={issues}
                update_model={update_model}
            />
        },
        JsonNodeKind::Array(_) => html! {
            <JsonArrayEditor
                node={model.clone()}
                path={Vec::<JsonPathSegment>::new()}
                policy_path={JsonInputPath::root()}
                depth={0}
                config={config.clone()}
                issues={issues}
                update_model={update_model}
            />
        },
        _ => html! {
            <div class={if show_type_column_for_path(config, &JsonInputPath::root(), JsonRowKind::RootScalar) {
                classes!("grid", "grid-cols-1", "gap-2", root_scalar_grid_columns_class(true), "md:items-center")
            } else {
                classes!("grid", "grid-cols-1", "gap-2")
            }}>
                if show_type_column_for_path(config, &JsonInputPath::root(), JsonRowKind::RootScalar) {
                    { render_type_select(
                        model,
                        &Vec::<JsonPathSegment>::new(),
                        &JsonInputPath::root(),
                        config,
                        update_model.clone(),
                        "Root value type",
                    ) }
                }
                <JsonValueEditor
                    node={model.clone()}
                    path={Vec::<JsonPathSegment>::new()}
                    depth={0}
                    config={config.clone()}
                    issues={issues}
                    aria_label_prefix="Root value"
                    error_id={None::<String>}
                    policy_path={JsonInputPath::root()}
                    update_model={update_model}
                />
            </div>
        },
    }
}
