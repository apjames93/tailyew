use super::super::model::{model_from_value, new_model_for_type};
use super::super::types::{
    JsonInputErrorKind, JsonModel, JsonNodeKind, JsonPropertyNode, JsonValueType,
    ValidationVisibility,
};
use super::super::validation::validate_model_report;
use super::{first_property_mut, test_config};
use serde_json::json;
use uuid::Uuid;
use yew::AttrValue;

#[test]
fn invalid_number_creates_validation_error_without_panicking() {
    let mut model = model_from_value(&json!({ "limit": 1 }));
    let property = first_property_mut(&mut model, "limit");
    property.value.kind = JsonNodeKind::Number { raw: "nope".into() };
    property.value.touched = true;

    let report = validate_model_report(
        &model,
        &test_config(),
        ValidationVisibility::All,
        "$".to_owned(),
    );

    assert!(!report.validity.is_valid);
    assert!(
        report
            .validity
            .errors
            .iter()
            .any(|error| error.kind == JsonInputErrorKind::InvalidNumber)
    );
}

#[test]
fn empty_key_creates_validation_error_after_validation_requested() {
    let model = JsonModel {
        id: Uuid::new_v4(),
        kind: JsonNodeKind::Object(vec![JsonPropertyNode {
            id: Uuid::new_v4(),
            key: String::new(),
            key_touched: false,
            value: new_model_for_type(JsonValueType::String, false),
        }]),
        touched: false,
        expanded: true,
        replaced_kind: None,
    };

    let report = validate_model_report(
        &model,
        &test_config(),
        ValidationVisibility::All,
        "$".to_owned(),
    );

    assert!(
        report
            .validity
            .errors
            .iter()
            .any(|error| error.kind == JsonInputErrorKind::EmptyKey)
    );
}

#[test]
fn duplicate_sibling_keys_create_validation_error() {
    let mut model = model_from_value(&json!({ "name": "one" }));
    if let JsonNodeKind::Object(properties) = &mut model.kind {
        properties.push(JsonPropertyNode {
            id: Uuid::new_v4(),
            key: "name".into(),
            key_touched: true,
            value: new_model_for_type(JsonValueType::String, false),
        });
    }

    let report = validate_model_report(
        &model,
        &test_config(),
        ValidationVisibility::All,
        "$".to_owned(),
    );

    assert!(
        report
            .validity
            .errors
            .iter()
            .any(|error| error.kind == JsonInputErrorKind::DuplicateKey)
    );
}

#[test]
fn untouched_invalid_draft_is_observable_in_full_validity() {
    let model = JsonModel {
        id: Uuid::new_v4(),
        kind: JsonNodeKind::Object(vec![JsonPropertyNode {
            id: Uuid::new_v4(),
            key: String::new(),
            key_touched: false,
            value: new_model_for_type(JsonValueType::String, false),
        }]),
        touched: false,
        expanded: true,
        replaced_kind: None,
    };

    let full_report = validate_model_report(
        &model,
        &test_config(),
        ValidationVisibility::All,
        "$".to_owned(),
    );
    let visible_report = validate_model_report(
        &model,
        &test_config(),
        ValidationVisibility::Touched,
        "$".to_owned(),
    );

    assert!(!full_report.validity.is_valid);
    assert!(visible_report.validity.is_valid);
}

#[test]
fn touched_empty_key_is_visible_without_submit_request() {
    let model = JsonModel {
        id: Uuid::new_v4(),
        kind: JsonNodeKind::Object(vec![JsonPropertyNode {
            id: Uuid::new_v4(),
            key: String::new(),
            key_touched: true,
            value: new_model_for_type(JsonValueType::String, false),
        }]),
        touched: false,
        expanded: true,
        replaced_kind: None,
    };

    let report = validate_model_report(
        &model,
        &test_config(),
        ValidationVisibility::Touched,
        "$".to_owned(),
    );

    assert!(
        report
            .validity
            .errors
            .iter()
            .any(|error| error.kind == JsonInputErrorKind::EmptyKey)
    );
}

#[test]
fn blank_string_values_are_allowed_by_default() {
    let model = model_from_value(&json!({ "authorization": "" }));

    let report = validate_model_report(
        &model,
        &test_config(),
        ValidationVisibility::All,
        "$".to_owned(),
    );

    assert!(report.validity.is_valid);
}

#[test]
fn required_string_values_reject_blank_and_whitespace_values() {
    let mut config = test_config();
    config.require_string_values = true;

    for value in ["", "   "] {
        let model = model_from_value(&json!({ "authorization": value }));
        let report =
            validate_model_report(&model, &config, ValidationVisibility::All, "$".to_owned());

        assert!(!report.validity.is_valid);
        assert!(report.validity.errors.iter().any(|error| {
            error.kind == JsonInputErrorKind::EmptyValue
                && error.path == "authorization"
                && error.message == "Enter a value."
        }));
        assert!(report.issues.iter().any(|issue| {
            issue.kind == JsonInputErrorKind::EmptyValue && issue.node_id.is_some()
        }));
    }
}

#[test]
fn required_string_values_allow_non_empty_values() {
    let mut config = test_config();
    config.require_string_values = true;
    let model = model_from_value(&json!({ "authorization": "Bearer token" }));

    let report = validate_model_report(&model, &config, ValidationVisibility::All, "$".to_owned());

    assert!(report.validity.is_valid);
}

#[test]
fn required_string_values_use_custom_message() {
    let mut config = test_config();
    config.require_string_values = true;
    config.empty_string_value_message = AttrValue::from("Header value is required.");
    let model = model_from_value(&json!({ "authorization": "" }));

    let report = validate_model_report(&model, &config, ValidationVisibility::All, "$".to_owned());

    assert!(
        report
            .validity
            .errors
            .iter()
            .any(|error| error.message == "Header value is required.")
    );
}

#[test]
fn required_string_value_errors_follow_visibility_rules() {
    let mut config = test_config();
    config.require_string_values = true;
    let mut model = model_from_value(&json!({ "authorization": "" }));

    let untouched_report = validate_model_report(
        &model,
        &config,
        ValidationVisibility::Touched,
        "$".to_owned(),
    );
    assert!(untouched_report.validity.is_valid);

    first_property_mut(&mut model, "authorization")
        .value
        .touched = true;
    let touched_report = validate_model_report(
        &model,
        &config,
        ValidationVisibility::Touched,
        "$".to_owned(),
    );
    assert!(!touched_report.validity.is_valid);
    assert!(
        touched_report
            .validity
            .errors
            .iter()
            .any(|error| error.kind == JsonInputErrorKind::EmptyValue)
    );
}
