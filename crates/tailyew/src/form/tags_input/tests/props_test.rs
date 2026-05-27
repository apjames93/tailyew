use crate::form::{JsonBackedValidationReport, JsonInputValidity, TagsInput};
use yew::{Callback, html};

#[test]
fn tags_input_public_props_accept_string_literals_for_ui_text() {
    let _ = html! {
        <TagsInput
            id="tags_editor"
            name="tags"
            label="Tags"
            placeholder="Add tag"
            on_validity_change={Some(Callback::from(|_: JsonInputValidity| {}))}
            on_validation_report_change={Some(Callback::from(|_: JsonBackedValidationReport| {}))}
        />
    };
}

#[test]
fn tags_input_public_props_accept_suggestions_only_mode() {
    let _ = html! {
        <TagsInput
            id="capabilities"
            label="Capabilities"
            allow_custom_tags={false}
            suggestions={vec!["rag".to_owned(), "evals".to_owned()]}
        />
    };
}
