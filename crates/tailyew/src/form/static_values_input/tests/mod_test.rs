use super::super::{
    effective_default_value_type, effective_static_value_types, static_value_types,
};
use crate::form::{JsonBackedValidationReport, JsonValueType, StaticValuesInput};
use yew::{Callback, html};

#[test]
fn static_values_input_public_props_accept_string_literals_for_ui_text() {
    let _ = html! {
        <StaticValuesInput
            id="static_values_editor"
            name="static_values"
            allowed_value_types={Some(vec![
                JsonValueType::String,
                JsonValueType::Number,
                JsonValueType::Boolean,
                JsonValueType::Null,
                JsonValueType::Object,
                JsonValueType::Array,
            ])}
            default_value_type={JsonValueType::Object}
            require_values={true}
            empty_value_message="Static value is required."
            on_validation_report_change={Some(Callback::from(|_: JsonBackedValidationReport| {}))}
        />
    };
}

#[test]
fn static_values_allow_only_string_number_boolean() {
    assert_eq!(
        static_value_types(),
        vec![
            JsonValueType::String,
            JsonValueType::Number,
            JsonValueType::Boolean
        ]
    );
    assert!(!static_value_types().contains(&JsonValueType::Object));
    assert!(!static_value_types().contains(&JsonValueType::Array));
    assert!(!static_value_types().contains(&JsonValueType::Null));
}

#[test]
fn static_values_default_allowed_types_are_string_number_boolean() {
    assert_eq!(effective_static_value_types(&None), static_value_types());
}

#[test]
fn static_values_custom_allowed_types_are_used_when_provided() {
    let allowed = vec![
        JsonValueType::String,
        JsonValueType::Null,
        JsonValueType::Object,
        JsonValueType::Array,
    ];

    assert_eq!(
        effective_static_value_types(&Some(allowed.clone())),
        allowed
    );
}

#[test]
fn static_values_empty_custom_allowed_types_fall_back_to_defaults() {
    assert_eq!(
        effective_static_value_types(&Some(Vec::new())),
        static_value_types()
    );
}

#[test]
fn static_values_default_value_type_uses_requested_type_when_allowed() {
    assert_eq!(
        effective_default_value_type(JsonValueType::Boolean, &static_value_types()),
        JsonValueType::Boolean
    );
}

#[test]
fn static_values_default_value_type_falls_back_to_first_allowed_type() {
    assert_eq!(
        effective_default_value_type(JsonValueType::Object, &static_value_types()),
        JsonValueType::String
    );
}

#[test]
fn static_values_rich_json_types_can_be_allowed_when_configured() {
    let allowed = effective_static_value_types(&Some(vec![
        JsonValueType::String,
        JsonValueType::Number,
        JsonValueType::Boolean,
        JsonValueType::Null,
        JsonValueType::Object,
        JsonValueType::Array,
    ]));

    assert!(allowed.contains(&JsonValueType::Null));
    assert!(allowed.contains(&JsonValueType::Object));
    assert!(allowed.contains(&JsonValueType::Array));
}
