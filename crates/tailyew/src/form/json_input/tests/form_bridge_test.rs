use super::super::form_bridge::{
    validation_bridge_aria_label, validation_bridge_id, validation_bridge_message,
};
use crate::form::submitted_name;
use yew::AttrValue;

#[test]
fn json_backed_components_default_submitted_name_to_id() {
    let id = AttrValue::from("field_id");

    assert_eq!(submitted_name(&id, &None).as_str(), "field_id");
}

#[test]
fn json_backed_components_can_submit_under_distinct_name() {
    let id = AttrValue::from("field_editor");
    let name = Some(AttrValue::from("payload_key"));

    assert_eq!(submitted_name(&id, &name).as_str(), "payload_key");
}

#[test]
fn validation_bridge_id_is_derived_from_dom_id() {
    assert_eq!(
        validation_bridge_id(&AttrValue::from("payload")).as_str(),
        "payload__validity"
    );
}

#[test]
fn validation_bridge_message_uses_field_label_by_default() {
    assert_eq!(
        validation_bridge_message(&AttrValue::from("Games played"), None).as_str(),
        "Fix validation errors in Games played before submitting."
    );
}

#[test]
fn validation_bridge_message_prefers_explicit_message() {
    assert_eq!(
        validation_bridge_message(
            &AttrValue::from("Games played"),
            Some(&AttrValue::from("Fix games."))
        )
        .as_str(),
        "Fix games."
    );
}

#[test]
fn validation_bridge_aria_label_uses_field_label() {
    assert_eq!(
        validation_bridge_aria_label(&AttrValue::from("Games played")).as_str(),
        "Games played validation status"
    );
}
