use super::super::model::{
    TagAddOutcome, add_tag_to_list, can_add_more, can_remove_more, normalize_tags, remove_tag_at,
    tag_is_allowed, tag_validation_message, tags_to_value,
};
use super::super::{tags_validation_report, tags_validity};
use crate::form::JsonInputErrorKind;
use serde_json::json;
use yew::AttrValue;

#[test]
fn tags_normalize_and_serialize_to_json_array() {
    let tags = normalize_tags(
        vec![
            " beta ".to_owned(),
            "internal".to_owned(),
            "beta".to_owned(),
        ],
        false,
    );

    assert_eq!(tags, vec!["beta", "internal"]);
    assert_eq!(tags_to_value(&tags), json!(["beta", "internal"]));
}

#[test]
fn tags_enter_or_comma_adds_tag_value() {
    let mut tags = vec!["beta".to_owned()];

    assert_eq!(
        add_tag_to_list(&mut tags, "internal,", false, true, &[], None),
        Ok(TagAddOutcome::Added)
    );
    assert_eq!(tags, vec!["beta", "internal"]);
}

#[test]
fn tags_block_duplicates_by_default_and_allow_when_configured() {
    let mut tags = vec!["beta".to_owned()];

    assert!(add_tag_to_list(&mut tags, "beta", false, true, &[], None).is_err());
    assert_eq!(
        add_tag_to_list(&mut tags, "beta", true, true, &[], None),
        Ok(TagAddOutcome::Added)
    );
    assert_eq!(tags, vec!["beta", "beta"]);
}

#[test]
fn tags_max_and_min_helpers_gate_add_remove() {
    let mut tags = vec!["beta".to_owned(), "internal".to_owned()];

    assert!(!can_add_more(&tags, Some(2)));
    assert!(add_tag_to_list(&mut tags, "stable", false, true, &[], Some(2)).is_err());
    assert!(!can_remove_more(&tags, Some(2)));
    assert!(remove_tag_at(&mut tags, 1));
    assert_eq!(tags, vec!["beta"]);
}

#[test]
fn tags_suggestions_only_mode_rejects_custom_values() {
    let suggestions = vec!["beta".to_owned(), "internal".to_owned()];
    let mut tags = vec!["beta".to_owned()];

    assert!(tag_is_allowed("internal", false, &suggestions));
    assert!(!tag_is_allowed("custom", false, &suggestions));
    assert_eq!(
        add_tag_to_list(&mut tags, "custom", false, false, &suggestions, None),
        Err("Select a suggested tag.".to_owned())
    );
    assert_eq!(
        add_tag_to_list(&mut tags, "internal", false, false, &suggestions, None),
        Ok(TagAddOutcome::Added)
    );
    assert_eq!(tags, vec!["beta", "internal"]);
}

#[test]
fn tags_validation_message_reports_native_bridge_constraints() {
    assert_eq!(
        tag_validation_message(&["beta".to_owned()], Some(2), None),
        Some("Add at least 2 tags before submitting.".to_owned())
    );
    assert_eq!(
        tag_validation_message(&["beta".to_owned(), "internal".to_owned()], None, Some(1)),
        Some("Use at most 1 tag before submitting.".to_owned())
    );
    assert_eq!(
        tag_validation_message(&["beta".to_owned()], Some(1), Some(2)),
        None
    );
}

#[test]
fn tags_validation_report_maps_min_max_issue() {
    let report = tags_validation_report(
        &AttrValue::from("release_tags_editor"),
        &Some(AttrValue::from("release_tags")),
        &AttrValue::from("Release tags"),
        Some("Add at least 2 tags before submitting."),
    );

    assert!(!report.is_valid);
    assert_eq!(report.field_name, AttrValue::from("release_tags"));
    assert_eq!(
        report.issues[0].message,
        AttrValue::from("Add at least 2 tags before submitting.")
    );
    assert_eq!(report.issues[0].raw_path, Some(AttrValue::from("$")));
}

#[test]
fn tags_validity_maps_min_max_constraints() {
    let min_validity = tags_validity(Some("Add at least 2 tags before submitting."));

    assert!(!min_validity.is_valid);
    assert_eq!(
        min_validity.errors[0].kind,
        JsonInputErrorKind::RequiredObjectEmpty
    );

    let max_validity = tags_validity(Some("Use at most 2 tags before submitting."));

    assert!(!max_validity.is_valid);
    assert_eq!(
        max_validity.errors[0].kind,
        JsonInputErrorKind::UnsupportedType
    );

    assert!(tags_validity(None).is_valid);
}

#[test]
fn tags_validation_report_clears_when_valid() {
    let report = tags_validation_report(
        &AttrValue::from("release_tags_editor"),
        &None,
        &AttrValue::from("Release tags"),
        None,
    );

    assert!(report.is_valid);
    assert_eq!(report.field_name, AttrValue::from("release_tags_editor"));
    assert!(report.issues.is_empty());
}
