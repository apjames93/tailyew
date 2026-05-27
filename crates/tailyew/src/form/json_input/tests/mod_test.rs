use super::super::form_bridge::{
    validation_bridge_aria_label, validation_bridge_id, validation_bridge_message,
};
use super::super::model::{model_from_value, value_from_model};
use super::super::types::{
    JsonInputError, JsonInputErrorKind, JsonInputValidity, JsonValidationMode,
    default_apply_paste_label, default_paste_label, default_paste_placeholder,
};
use super::super::validation::validate_model_report;
use super::super::{
    hidden_json_value, parse_pasted_json, preview_status_copy, should_show_all_validation,
};
use super::{first_property_mut, test_config};
use crate::form::JsonInput;
use serde_json::json;
use yew::{AttrValue, html};

#[test]
fn json_input_public_text_props_accept_string_literals() {
    let _ = html! {
        <JsonInput
            id="json_payload_editor"
            name="json_payload"
            label="Request fields"
            helper_text={Some("Build the JSON object.")}
            placeholder_key={Some("Key")}
            placeholder_value={Some("Value")}
            allow_raw_json_paste={true}
            paste_label="Paste request JSON"
            paste_helper_text={Some("Paste any valid JSON snippet.")}
            paste_placeholder={"{\n  \"ok\": true\n}"}
            apply_paste_label="Use pasted JSON"
            add_property_label="Add field"
            add_item_label="Add item"
        />
    };
}

#[test]
fn json_input_paste_copy_defaults_are_generic() {
    let placeholder = default_paste_placeholder();

    assert_eq!(default_paste_label().as_str(), "Paste JSON");
    assert_eq!(default_apply_paste_label().as_str(), "Apply JSON");
    assert!(placeholder.as_str().contains("\"example\": true"));
    assert!(!placeholder.as_str().to_lowercase().contains("headers"));
    assert!(!placeholder.as_str().to_lowercase().contains("config"));
}

#[test]
fn valid_pasted_json_parses_to_editor_model() {
    let model = parse_pasted_json(r#"{ "status": "ok", "count": 2 }"#).expect("valid JSON parses");

    assert_eq!(
        value_from_model(&model).expect("model serializes"),
        json!({ "status": "ok", "count": 2 })
    );
}

#[test]
fn invalid_pasted_json_reports_error_without_model_replacement() {
    let error = parse_pasted_json(r#"{ "status": "ok" "#).expect_err("invalid JSON returns error");

    assert!(error.starts_with("Invalid JSON:"));
}

#[test]
fn validation_bridge_uses_stable_proxy_id_without_form_name() {
    let id = AttrValue::from("json_payload");

    assert_eq!(validation_bridge_id(&id).as_str(), "json_payload__validity");
}

#[test]
fn validation_bridge_message_uses_custom_or_label_copy() {
    let label = AttrValue::from("Games played");
    let custom = AttrValue::from("Fix the games list.");

    assert_eq!(
        validation_bridge_message(&label, None).as_str(),
        "Fix validation errors in Games played before submitting."
    );
    assert_eq!(
        validation_bridge_message(&label, Some(&custom)).as_str(),
        "Fix the games list."
    );
    assert_eq!(
        validation_bridge_message(&AttrValue::from(""), None).as_str(),
        "Fix validation errors before submitting."
    );
}

#[test]
fn validation_bridge_aria_label_names_the_proxy_control() {
    assert_eq!(
        validation_bridge_aria_label(&AttrValue::from("Games played")).as_str(),
        "Games played validation status"
    );
    assert_eq!(
        validation_bridge_aria_label(&AttrValue::from("")).as_str(),
        "JSON field validation status"
    );
}

#[test]
fn hidden_input_preserves_last_valid_json_when_draft_is_invalid() {
    let last_valid_json = json!({ "name": "old" });
    let mut model = model_from_value(&last_valid_json);
    let property = first_property_mut(&mut model, "name");
    property.key.clear();
    property.key_touched = true;

    let report = validate_model_report(
        &model,
        &test_config(),
        super::super::types::ValidationVisibility::All,
        "$".to_owned(),
    );

    assert!(!report.validity.is_valid);
    assert!(value_from_model(&model).is_err());
    assert_eq!(
        hidden_json_value(&model, &last_valid_json, &report.validity),
        last_valid_json
    );
}

#[test]
fn invalid_draft_preview_copy_matches_visible_validation_state() {
    let full_invalid = JsonInputValidity {
        is_valid: false,
        errors: vec![JsonInputError {
            path: "headers.(empty key)".into(),
            message: "Enter a property name.".into(),
            kind: JsonInputErrorKind::EmptyKey,
        }],
    };
    let visible_valid = JsonInputValidity {
        is_valid: true,
        errors: Vec::new(),
    };
    let visible_invalid = full_invalid.clone();

    assert_eq!(
        preview_status_copy(&full_invalid, &visible_valid),
        "Preview will update after incomplete rows are fixed."
    );
    assert_eq!(
        preview_status_copy(&full_invalid, &visible_invalid),
        "Preview will update after validation errors are fixed."
    );
}

#[test]
fn validation_request_id_enables_visible_validation() {
    assert!(should_show_all_validation(
        JsonValidationMode::OnBlurOrSubmit,
        false,
        Some(1),
        false
    ));
    assert!(!should_show_all_validation(
        JsonValidationMode::OnBlurOrSubmit,
        false,
        None,
        false
    ));
}
