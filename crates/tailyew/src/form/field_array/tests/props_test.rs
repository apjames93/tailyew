use super::super::object_field::{FieldArrayObjectField, FieldArraySelectOption};
use crate::form::{
    FieldArray, FieldArrayCustomIssue, FieldArrayDeleteBehavior, FieldArrayText,
    FieldArrayValidationContext, FieldArrayValidator, JsonBackedValidationReport, JsonValueType,
};
use std::collections::BTreeMap;
use yew::Callback;
use yew::html;

#[test]
fn field_array_public_props_accept_string_literals_for_ui_text() {
    let _ = html! {
        <>
            <FieldArray id="scopes" label="OAuth scopes" add_label="Add scope" />
            <FieldArray
                id="games_editor"
                name="games"
                label="Games"
                text={FieldArrayText {
                    item_label: "game".into(),
                    item_label_plural: "games".into(),
                    ..FieldArrayText::default()
                }}
                delete_behavior={FieldArrayDeleteBehavior::mark_deleted()}
                object_fields={Some(vec![
                    FieldArrayObjectField::hidden("id", JsonValueType::Number),
                    FieldArrayObjectField::string("name", "Game"),
                    FieldArrayObjectField::select(
                        "type",
                        "Type",
                        vec![FieldArraySelectOption::same("string")],
                    ),
                ])}
                custom_validate={Some(Callback::from(|context: FieldArrayValidationContext| {
                    let mut seen = BTreeMap::<String, usize>::new();
                    let mut issues = Vec::new();
                    for row in context.rows.iter().filter(|row| !row.deleted) {
                        let Some(name) = row.get_string("name") else {
                            continue;
                        };
                        if let Some(first_index) = seen.insert(name.to_owned(), row.index) {
                            issues.push(FieldArrayCustomIssue::field(
                                first_index,
                                "name",
                                "Names must be unique.",
                            ));
                            issues.push(FieldArrayCustomIssue::field(
                                row.index,
                                "name",
                                "Names must be unique.",
                            ));
                        }
                    }
                    issues
                }))}
                validators={vec![
                    FieldArrayValidator::unique_field_trimmed(
                        "name",
                        "Names must be unique.",
                    ),
                ]}
                on_validation_report_change={Some(Callback::from(|_: JsonBackedValidationReport| {}))}
            />
        </>
    };
}

#[test]
fn field_array_default_text_avoids_developer_facing_copy() {
    let text = FieldArrayText::default();
    let deleted_description = text.deleted_rows_description.to_string();
    let invalid_status = text.invalid_status.to_string();

    assert!(!deleted_description.contains("submitted JSON"));
    assert!(!deleted_description.contains("deletion marker"));
    assert!(!invalid_status.contains("Last valid JSON"));
}
