use super::custom_validation::{
    FieldArrayCustomIssue, FieldArrayValidationContext, FieldArrayValidationRow,
    FieldArrayValidator, merge_custom_issues, validation_context_from_object_rows,
};
use super::deleted_rows::active_object_row_count;
use super::model::{
    FieldArrayFieldValue, FieldArrayObjectIssue, FieldArrayObjectReport, FieldArrayObjectRow,
    parse_json_number_value,
};
use super::object_field::{
    FieldArrayFieldValidationContext, FieldArrayFieldValidator, FieldArrayObjectField,
    FieldArrayObjectFieldEditor, field_key, select_option_allows_value,
};
use super::props::FieldArrayText;
use crate::form::{
    JsonBackedValidationIssue, JsonBackedValidationReport, JsonInputError, JsonInputErrorKind,
    JsonInputValidity, submitted_name,
};
use regex::Regex;
use serde_json::Value;
use std::collections::{BTreeMap, HashSet};
use yew::{AttrValue, Callback};

#[cfg(test)]
pub(crate) fn validate_object_rows(
    rows: &[FieldArrayObjectRow],
    fields: &[FieldArrayObjectField],
    min_items: Option<usize>,
) -> FieldArrayObjectReport {
    validate_object_rows_with_delete(rows, fields, min_items)
}

pub(crate) fn validate_object_rows_with_delete(
    rows: &[FieldArrayObjectRow],
    fields: &[FieldArrayObjectField],
    min_items: Option<usize>,
) -> FieldArrayObjectReport {
    validate_object_rows_with_delete_and_custom(rows, fields, min_items, &[], None)
}

pub(crate) fn validate_object_rows_with_delete_and_custom(
    rows: &[FieldArrayObjectRow],
    fields: &[FieldArrayObjectField],
    min_items: Option<usize>,
    validators: &[FieldArrayValidator],
    custom_validate: Option<&Callback<FieldArrayValidationContext, Vec<FieldArrayCustomIssue>>>,
) -> FieldArrayObjectReport {
    let mut issues = Vec::new();
    let mut errors = Vec::new();
    let active_count = active_object_row_count(rows);
    let validation_context = validation_context_from_object_rows(rows, fields);

    if let Some(min_items) = min_items
        && active_count < min_items
    {
        errors.push(JsonInputError {
            path: "$".to_owned(),
            message: format!("Add at least {min_items} items."),
            kind: JsonInputErrorKind::RequiredObjectEmpty,
        });
    }

    for (row_index, row) in rows.iter().enumerate() {
        if row.deleted {
            continue;
        }

        for field in fields {
            let key = field_key(field);
            let Some(value) = row.values.get(&key) else {
                continue;
            };

            if field.required && !field.hidden && value.is_empty_string() {
                let message = if matches!(field.editor, FieldArrayObjectFieldEditor::Select { .. })
                {
                    "Choose a value.".to_owned()
                } else {
                    "Enter a value.".to_owned()
                };
                push_field_issue(
                    &mut issues,
                    &mut errors,
                    row_index,
                    &key,
                    message,
                    JsonInputErrorKind::EmptyKey,
                );
                continue;
            }

            if let (FieldArrayObjectFieldEditor::Select { .. }, FieldArrayFieldValue::String(value)) =
                (&field.editor, value)
                && !value.trim().is_empty()
                && !select_option_allows_value(field, value)
            {
                let message = "Choose a valid option.".to_owned();
                push_field_issue(
                    &mut issues,
                    &mut errors,
                    row_index,
                    &key,
                    message,
                    JsonInputErrorKind::UnsupportedType,
                );
                continue;
            }

            if let FieldArrayFieldValue::Number { raw } = value
                && parse_json_number_value(raw).is_none()
            {
                let message = "Enter a valid number.".to_owned();
                push_field_issue(
                    &mut issues,
                    &mut errors,
                    row_index,
                    &key,
                    message,
                    JsonInputErrorKind::InvalidNumber,
                );
            }

            validate_field_validators(
                &mut issues,
                &mut errors,
                row_index,
                row,
                field,
                value,
                &validation_context,
            );
        }
    }

    let mut report = FieldArrayObjectReport {
        validity: JsonInputValidity {
            is_valid: errors.is_empty(),
            errors,
        },
        issues,
    };

    let validator_issues = run_array_validators(validators, &validation_context);
    merge_custom_issues(&mut report, validator_issues);

    if let Some(custom_validate) = custom_validate {
        let custom_issues = custom_validate.emit(validation_context);
        merge_custom_issues(&mut report, custom_issues);
    }

    report
}

fn validate_field_validators(
    issues: &mut Vec<FieldArrayObjectIssue>,
    errors: &mut Vec<JsonInputError>,
    row_index: usize,
    row: &FieldArrayObjectRow,
    field: &FieldArrayObjectField,
    field_value: &FieldArrayFieldValue,
    validation_context: &FieldArrayValidationContext,
) {
    if row.deleted || field.hidden || field.validators.is_empty() {
        return;
    }

    let key = field_key(field);
    let context =
        field_validation_context(row_index, &key, row, field, field_value, validation_context);

    for validator in &field.validators {
        let message = match validator {
            FieldArrayFieldValidator::RequiredTrimmed { message } => context
                .value
                .as_str()
                .is_some_and(|value| value.trim().is_empty())
                .then(|| message.to_string()),
            FieldArrayFieldValidator::Pattern { pattern, message } => {
                match Regex::new(pattern.as_str()) {
                    Ok(regex) => context
                        .value
                        .as_str()
                        .is_some_and(|value| !value.is_empty() && !regex.is_match(value))
                        .then(|| message.to_string()),
                    Err(_) => Some(message.to_string()),
                }
            }
            FieldArrayFieldValidator::Custom { validate } => validate
                .emit(context.clone())
                .map(|message| message.to_string()),
        };

        if let Some(message) = message {
            push_field_issue(
                issues,
                errors,
                row_index,
                &key,
                message,
                match validator {
                    FieldArrayFieldValidator::RequiredTrimmed { .. } => {
                        JsonInputErrorKind::EmptyValue
                    }
                    FieldArrayFieldValidator::Pattern { .. }
                    | FieldArrayFieldValidator::Custom { .. } => {
                        JsonInputErrorKind::UnsupportedType
                    }
                },
            );
        }
    }
}

fn field_validation_context(
    row_index: usize,
    key: &str,
    row: &FieldArrayObjectRow,
    field: &FieldArrayObjectField,
    field_value: &FieldArrayFieldValue,
    validation_context: &FieldArrayValidationContext,
) -> FieldArrayFieldValidationContext {
    let row_context = validation_context
        .rows
        .iter()
        .find(|row| row.index == row_index)
        .cloned()
        .unwrap_or_else(|| FieldArrayValidationRow {
            index: row_index,
            deleted: row.deleted,
            values: BTreeMap::new(),
        });
    let value = row_context
        .get(key)
        .cloned()
        .unwrap_or_else(|| validation_value_from_field_value(field_value));

    FieldArrayFieldValidationContext {
        row_index,
        key: field.key.clone(),
        label: field.label.clone(),
        value,
        row: row_context,
    }
}

fn validation_value_from_field_value(value: &FieldArrayFieldValue) -> Value {
    match value {
        FieldArrayFieldValue::String(value) => Value::String(value.clone()),
        FieldArrayFieldValue::Number { raw } => {
            parse_json_number_value(raw).unwrap_or_else(|| Value::String(raw.clone()))
        }
        FieldArrayFieldValue::Boolean(value) => Value::Bool(*value),
        FieldArrayFieldValue::Null => Value::Null,
        FieldArrayFieldValue::Json(value) => value.clone(),
    }
}

fn run_array_validators(
    validators: &[FieldArrayValidator],
    context: &FieldArrayValidationContext,
) -> Vec<FieldArrayCustomIssue> {
    let mut issues = Vec::new();

    for validator in validators {
        match validator {
            FieldArrayValidator::UniqueField {
                key,
                message,
                trim,
                case_sensitive,
            } => issues.extend(unique_field_issues(
                context,
                key,
                message,
                *trim,
                *case_sensitive,
            )),
            FieldArrayValidator::Custom { validate } => {
                issues.extend(validate.emit(context.clone()));
            }
        }
    }

    issues
}

fn unique_field_issues(
    context: &FieldArrayValidationContext,
    key: &AttrValue,
    message: &AttrValue,
    trim: bool,
    case_sensitive: bool,
) -> Vec<FieldArrayCustomIssue> {
    let key_string = key.to_string();
    let mut seen = BTreeMap::<String, Vec<usize>>::new();

    for row in context.rows.iter().filter(|row| !row.deleted) {
        let Some(value) = row.get_string(&key_string) else {
            continue;
        };
        let value = if trim { value.trim() } else { value };
        if value.is_empty() {
            continue;
        }

        let comparable = if case_sensitive {
            value.to_owned()
        } else {
            value.to_lowercase()
        };
        seen.entry(comparable).or_default().push(row.index);
    }

    seen.into_values()
        .filter(|row_indices| row_indices.len() > 1)
        .flat_map(|row_indices| {
            row_indices.into_iter().map(|row_index| {
                FieldArrayCustomIssue::field(row_index, key.clone(), message.clone())
                    .with_kind(JsonInputErrorKind::DuplicateKey)
            })
        })
        .collect()
}

fn push_field_issue(
    issues: &mut Vec<FieldArrayObjectIssue>,
    errors: &mut Vec<JsonInputError>,
    row_index: usize,
    key: &str,
    message: String,
    kind: JsonInputErrorKind,
) {
    issues.push(FieldArrayObjectIssue {
        row_index,
        key: Some(key.to_owned()),
        message: message.clone(),
        kind: kind.clone(),
    });
    errors.push(JsonInputError {
        path: format!("$[{row_index}].{key}"),
        message,
        kind,
    });
}

pub(crate) fn issue_count_copy(count: usize) -> String {
    if count == 1 {
        "1 issue needs attention".to_owned()
    } else {
        format!("{count} issues need attention")
    }
}

pub(crate) fn friendly_validation_summary_items(
    report: &FieldArrayObjectReport,
    fields: &[FieldArrayObjectField],
    text: &FieldArrayText,
) -> Vec<String> {
    let mut items = report
        .validity
        .errors
        .iter()
        .filter(|error| error.path == "$")
        .map(|error| error.message.clone())
        .collect::<Vec<_>>();

    items.extend(report.issues.iter().map(|issue| {
        let field_label = issue
            .key
            .as_deref()
            .and_then(|key| field_label_for_key(fields, key));
        let label = friendly_issue_label(issue.row_index, field_label, text);

        format!("{label}: {}", issue.message)
    }));

    if items.is_empty() && !report.validity.is_valid {
        items.extend(
            report
                .validity
                .errors
                .iter()
                .map(|error| error.message.clone()),
        );
    }

    items
}

pub(crate) fn field_array_validation_report(
    id: &AttrValue,
    name: &Option<AttrValue>,
    label: &AttrValue,
    report: &FieldArrayObjectReport,
    fields: &[FieldArrayObjectField],
    text: &FieldArrayText,
) -> JsonBackedValidationReport {
    let mut covered_paths = HashSet::new();
    let mut issues = report
        .issues
        .iter()
        .map(|issue| {
            let raw_path = match &issue.key {
                Some(key) => format!("$[{}].{key}", issue.row_index),
                None => format!("$[{}]", issue.row_index),
            };
            covered_paths.insert(raw_path.clone());

            let field_label = issue
                .key
                .as_deref()
                .and_then(|key| field_label_for_key(fields, key))
                .cloned();

            JsonBackedValidationIssue {
                message: AttrValue::from(issue.message.clone()),
                label: field_label.clone(),
                location: Some(AttrValue::from(friendly_issue_label(
                    issue.row_index,
                    field_label.as_ref(),
                    text,
                ))),
                path: Some(AttrValue::from(raw_path.clone())),
                row_index: Some(issue.row_index),
                key: issue.key.clone().map(AttrValue::from),
                raw_path: Some(AttrValue::from(raw_path)),
            }
        })
        .collect::<Vec<_>>();

    issues.extend(
        report
            .validity
            .errors
            .iter()
            .filter(|error| !covered_paths.contains(&error.path))
            .map(|error| {
                let path = AttrValue::from(error.path.clone());

                JsonBackedValidationIssue {
                    message: AttrValue::from(error.message.clone()),
                    label: None,
                    location: None,
                    path: Some(path.clone()),
                    row_index: None,
                    key: None,
                    raw_path: Some(path),
                }
            }),
    );

    JsonBackedValidationReport {
        is_valid: report.validity.is_valid,
        field_id: id.clone(),
        field_name: submitted_name(id, name),
        label: (!label.as_str().trim().is_empty()).then(|| label.clone()),
        issues,
    }
}

pub(crate) fn friendly_issue_label(
    row_index: usize,
    field_label: Option<&AttrValue>,
    text: &FieldArrayText,
) -> String {
    let row_label = format!(
        "{} {}",
        capitalize_label(text.item_label.as_str()),
        row_index + 1
    );

    match field_label {
        Some(field_label) if !field_label.is_empty() => {
            format!("{row_label} · {field_label}")
        }
        _ => row_label,
    }
}

pub(crate) fn message_for_kind(kind: &JsonInputErrorKind) -> &'static str {
    match kind {
        JsonInputErrorKind::InvalidNumber => "Enter a valid number.",
        JsonInputErrorKind::EmptyKey => "Enter a value.",
        JsonInputErrorKind::EmptyValue => "Enter a value.",
        JsonInputErrorKind::DuplicateKey => "Value must be unique.",
        JsonInputErrorKind::MaxDepthExceeded => "Maximum depth exceeded.",
        JsonInputErrorKind::RequiredObjectEmpty => "Add at least one item.",
        JsonInputErrorKind::UnsupportedType => "Unsupported value type.",
    }
}

fn field_label_for_key<'a>(
    fields: &'a [FieldArrayObjectField],
    key: &str,
) -> Option<&'a AttrValue> {
    fields
        .iter()
        .find(|field| field_key(field) == key)
        .map(|field| &field.label)
}

fn capitalize_label(label: &str) -> String {
    let mut chars = label.trim().chars();
    let Some(first) = chars.next() else {
        return "Item".to_owned();
    };

    first.to_uppercase().chain(chars).collect::<String>()
}
