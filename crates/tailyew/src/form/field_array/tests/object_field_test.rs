use super::super::object_field::{
    FieldArrayFieldValidator, FieldArrayObjectField, FieldArrayObjectFieldEditor,
    FieldArraySelectOption, default_value_for_field, visible_object_fields,
};
use super::game_fields;
use crate::form::JsonValueType;
use serde_json::json;

#[test]
fn field_array_object_mode_hides_configured_hidden_fields() {
    let visible = visible_object_fields(&game_fields());

    assert_eq!(
        visible
            .iter()
            .map(|field| field.key.to_string())
            .collect::<Vec<_>>(),
        vec!["name", "hours_played", "beat"]
    );
}

#[test]
fn field_array_select_field_is_string_backed() {
    let field =
        FieldArrayObjectField::select("type", "Type", vec![FieldArraySelectOption::same("string")]);

    assert_eq!(field.value_type, JsonValueType::String);
    assert!(field.editable);
    assert!(!field.hidden);
    assert!(matches!(
        field.editor,
        FieldArrayObjectFieldEditor::Select { .. }
    ));
}

#[test]
fn field_array_select_option_constructors_are_ergonomic() {
    assert_eq!(
        FieldArraySelectOption::same("string"),
        FieldArraySelectOption::new("string", "string")
    );
    assert!(
        FieldArraySelectOption::new("Integer", "integer")
            .disabled(true)
            .disabled
    );
}

#[test]
fn field_array_select_default_uses_first_option_when_missing() {
    let field = FieldArrayObjectField::select(
        "type",
        "Type",
        vec![
            FieldArraySelectOption::same("string"),
            FieldArraySelectOption::same("boolean"),
        ],
    );

    assert_eq!(default_value_for_field(&field), json!("string"));
}

#[test]
fn field_array_select_explicit_default_overrides_first_option() {
    let field = FieldArrayObjectField::select(
        "type",
        "Type",
        vec![
            FieldArraySelectOption::same("string"),
            FieldArraySelectOption::same("boolean"),
        ],
    )
    .default_value(json!("boolean"));

    assert_eq!(default_value_for_field(&field), json!("boolean"));
}

#[test]
fn field_array_object_field_builders_attach_field_validators() {
    let field = FieldArrayObjectField::string("name", "Name")
        .required_trimmed("Enter a name.")
        .pattern(r"^[A-Za-z]+$", "Use letters.");

    assert_eq!(field.validators.len(), 2);
    assert!(matches!(
        field.validators[0],
        FieldArrayFieldValidator::RequiredTrimmed { .. }
    ));
    assert!(matches!(
        field.validators[1],
        FieldArrayFieldValidator::Pattern { .. }
    ));
}
