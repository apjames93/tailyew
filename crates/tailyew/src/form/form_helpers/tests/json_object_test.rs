use super::super::json_object::form_json_object_from_values;
use crate::form::FormFieldSpec;
use serde_json::Value;

fn build_json_object(
    fields: &[FormFieldSpec],
    values: &[(&str, &str)],
    checks: &[(&str, bool)],
) -> Result<Value, String> {
    form_json_object_from_values(
        fields,
        |name| {
            values
                .iter()
                .find(|(field, _)| *field == name)
                .map(|(_, value)| (*value).to_owned())
                .ok_or_else(|| format!("Form field '{name}' was not found."))
        },
        |name| {
            checks
                .iter()
                .find(|(field, _)| *field == name)
                .map(|(_, value)| *value)
                .ok_or_else(|| format!("Form field '{name}' was not found."))
        },
    )
}

#[test]
fn form_json_object_builds_mixed_payload() {
    let payload = build_json_object(
        &[
            FormFieldSpec::string("user_name"),
            FormFieldSpec::number("age"),
            FormFieldSpec::boolean("active"),
            FormFieldSpec::json("games_played"),
        ],
        &[
            ("user_name", "buddy guy"),
            ("age", "30"),
            (
                "games_played",
                r#"[{"id":1,"name":"Resident Evil Requiem"}]"#,
            ),
        ],
        &[("active", true)],
    )
    .unwrap();

    assert_eq!(
        payload,
        serde_json::json!({
            "user_name": "buddy guy",
            "age": 30,
            "active": true,
            "games_played": [
                { "id": 1, "name": "Resident Evil Requiem" }
            ]
        })
    );
}

#[test]
fn form_json_object_rejects_invalid_number() {
    let err =
        build_json_object(&[FormFieldSpec::number("age")], &[("age", "bad")], &[]).unwrap_err();

    assert_eq!(err, "Field 'age' must be a valid JSON number.");
}

#[test]
fn form_json_object_rejects_invalid_json() {
    let err =
        build_json_object(&[FormFieldSpec::json("payload")], &[("payload", "{")], &[]).unwrap_err();

    assert!(err.starts_with("Field 'payload' must contain valid JSON:"));
}

#[test]
fn form_json_object_reports_missing_field() {
    let err = build_json_object(&[FormFieldSpec::string("missing")], &[], &[]).unwrap_err();

    assert_eq!(err, "Form field 'missing' was not found.");
}
