use super::model::{TagAddOutcome, add_tag_to_list, tags_to_value};
use serde_json::Value;
use yew::prelude::*;

pub(super) fn tag_change_callback(
    on_change: Option<Callback<Vec<String>>>,
    on_json_change: Option<Callback<Value>>,
) -> Callback<Vec<String>> {
    Callback::from(move |next_tags: Vec<String>| {
        if let Some(on_change) = &on_change {
            on_change.emit(next_tags.clone());
        }
        if let Some(on_json_change) = &on_json_change {
            on_json_change.emit(tags_to_value(&next_tags));
        }
    })
}

pub(super) struct TagAddRequest<'a> {
    pub tags: &'a UseStateHandle<Vec<String>>,
    pub error: &'a UseStateHandle<Option<String>>,
    pub emit_changes: &'a Callback<Vec<String>>,
    pub raw_tag: &'a str,
    pub allow_duplicates: bool,
    pub allow_custom_tags: bool,
    pub suggestions: &'a [String],
    pub max_tags: Option<usize>,
}

pub(super) fn commit_tag_add(request: TagAddRequest<'_>) -> bool {
    let mut next_tags = (**request.tags).clone();
    match add_tag_to_list(
        &mut next_tags,
        request.raw_tag,
        request.allow_duplicates,
        request.allow_custom_tags,
        request.suggestions,
        request.max_tags,
    ) {
        Ok(TagAddOutcome::Added) => {
            request.tags.set(next_tags.clone());
            request.error.set(None);
            request.emit_changes.emit(next_tags);
            true
        }
        Ok(TagAddOutcome::IgnoredEmpty) => {
            request.error.set(None);
            true
        }
        Err(message) => {
            request.error.set(Some(message));
            false
        }
    }
}
