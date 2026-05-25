use super::model::{array_path, duplicate_properties, parse_json_number, property_path};
use super::types::*;

pub(super) fn validate_model_report(
    model: &JsonModel,
    config: &JsonInputConfig,
    visibility: ValidationVisibility,
    root_path: String,
) -> JsonValidationReport {
    let mut issues = Vec::new();
    validate_node(model, config, visibility, &root_path, 0, true, &mut issues);

    JsonValidationReport {
        validity: JsonInputValidity {
            is_valid: issues.is_empty(),
            errors: issues
                .iter()
                .map(|issue| JsonInputError {
                    path: issue.path.clone(),
                    message: issue.message.clone(),
                    kind: issue.kind.clone(),
                })
                .collect(),
        },
        issues,
    }
}

fn validate_node(
    model: &JsonModel,
    config: &JsonInputConfig,
    visibility: ValidationVisibility,
    path: &str,
    depth: usize,
    is_root: bool,
    issues: &mut Vec<JsonInputIssue>,
) {
    if let Some(max_depth) = config.max_depth
        && depth > max_depth
    {
        issues.push(JsonInputIssue {
            node_id: Some(model.id),
            path: path.to_owned(),
            message: format!("Maximum nesting depth is {max_depth}."),
            kind: JsonInputErrorKind::MaxDepthExceeded,
        });
        return;
    }

    match &model.kind {
        JsonNodeKind::String(value) => {
            if config.require_string_values
                && should_validate_node(model, visibility)
                && value.trim().is_empty()
            {
                issues.push(JsonInputIssue {
                    node_id: Some(model.id),
                    path: path.to_owned(),
                    message: config.empty_string_value_message.to_string(),
                    kind: JsonInputErrorKind::EmptyValue,
                });
            }
        }
        JsonNodeKind::Number { raw } => {
            if should_validate_node(model, visibility) && parse_json_number(raw).is_none() {
                issues.push(JsonInputIssue {
                    node_id: Some(model.id),
                    path: path.to_owned(),
                    message: "Enter a valid JSON number.".to_owned(),
                    kind: JsonInputErrorKind::InvalidNumber,
                });
            }
        }
        JsonNodeKind::Object(properties) => {
            if is_root
                && config.require_at_least_one
                && properties.is_empty()
                && visibility == ValidationVisibility::All
            {
                issues.push(JsonInputIssue {
                    node_id: Some(model.id),
                    path: path.to_owned(),
                    message: "Add at least one property.".to_owned(),
                    kind: JsonInputErrorKind::RequiredObjectEmpty,
                });
            }

            for property in properties {
                let key_path = property_path(path, &property.key);
                if should_validate_key(property, visibility) && property.key.trim().is_empty() {
                    issues.push(JsonInputIssue {
                        node_id: Some(property.id),
                        path: key_path,
                        message: "Enter a property name.".to_owned(),
                        kind: JsonInputErrorKind::EmptyKey,
                    });
                }
            }

            for property in duplicate_properties(properties) {
                if should_validate_key(property, visibility) {
                    issues.push(JsonInputIssue {
                        node_id: Some(property.id),
                        path: property_path(path, &property.key),
                        message: "Property names must be unique at this level.".to_owned(),
                        kind: JsonInputErrorKind::DuplicateKey,
                    });
                }
            }

            for property in properties {
                validate_node(
                    &property.value,
                    config,
                    visibility,
                    &property_path(path, &property.key),
                    depth + 1,
                    false,
                    issues,
                );
            }
        }
        JsonNodeKind::Array(items) => {
            for (index, item) in items.iter().enumerate() {
                validate_node(
                    &item.value,
                    config,
                    visibility,
                    &array_path(path, index),
                    depth + 1,
                    false,
                    issues,
                );
            }
        }
        JsonNodeKind::Boolean(_) | JsonNodeKind::Null => {}
    }
}

fn should_validate_key(property: &JsonPropertyNode, visibility: ValidationVisibility) -> bool {
    visibility == ValidationVisibility::All || property.key_touched
}

fn should_validate_node(model: &JsonModel, visibility: ValidationVisibility) -> bool {
    visibility == ValidationVisibility::All || model.touched
}
