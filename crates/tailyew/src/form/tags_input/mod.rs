mod model;
mod props;
mod state;
mod suggestions;

#[cfg(test)]
mod tests;

pub use props::TagsInputProps;

use crate::form::json_input::form_bridge::{JsonBackedHiddenInput, JsonFormValidityBridge};
use crate::form::{
    JsonBackedValidationIssue, JsonBackedValidationReport, JsonInputError, JsonInputErrorKind,
    JsonInputValidity, submitted_name,
};
use crate::{AddIcon, Button, ButtonSize, ButtonType, Chip, ChipSize, ChipVariant, Label};
use model::{
    can_add_more, can_remove_more, normalize_tags, remove_tag_at, sanitize_tag, tag_is_allowed,
    tag_validation_message,
};
use state::{TagAddRequest, commit_tag_add, tag_change_callback};
use suggestions::TagsSuggestions;
use web_sys::HtmlInputElement;
use yew::prelude::*;

/// Edits tags as removable chips and submits them as a JSON string array.
#[component(TagsInput)]
pub fn tags_input(props: &TagsInputProps) -> Html {
    let tags = use_state(|| normalize_tags(props.initial_tags.clone(), props.allow_duplicates));
    let draft = use_state(String::new);
    let error = use_state(|| None::<String>);
    let helper_id = props
        .helper_text
        .as_ref()
        .map(|_| format!("{}-helper", props.id.as_str()));
    let error_id = (*error)
        .is_some()
        .then(|| format!("{}-error", props.id.as_str()));
    let input_id = format!("{}-tag-input", props.id.as_str());
    let hidden_json = serde_json::to_string(&*tags).unwrap_or_else(|_| "[]".to_owned());
    let can_add = can_add_more(&tags, props.max_tags);
    let can_remove = can_remove_more(&tags, props.min_tags);
    let validation_error = tag_validation_message(&tags, props.min_tags, props.max_tags);
    let validity = tags_validity(validation_error.as_deref());
    let validation_report = tags_validation_report(
        &props.id,
        &props.name,
        &props.label,
        validation_error.as_deref(),
    );
    let draft_can_be_added = sanitize_tag(draft.as_str()).is_some_and(|tag| {
        can_add && tag_is_allowed(&tag, props.allow_custom_tags, &props.suggestions)
    });
    let label_for_id = if props.allow_custom_tags {
        AttrValue::from(input_id.clone())
    } else {
        AttrValue::default()
    };

    let emit_changes = tag_change_callback(props.on_change.clone(), props.on_json_change.clone());

    {
        let on_validity_change = props.on_validity_change.clone();
        let on_validation_report_change = props.on_validation_report_change.clone();
        let validity = validity.clone();
        let validation_report = validation_report.clone();

        use_effect_with((validity, validation_report), move |(validity, report)| {
            if let Some(on_validity_change) = &on_validity_change {
                on_validity_change.emit(validity.clone());
            }
            if let Some(on_validation_report_change) = &on_validation_report_change {
                on_validation_report_change.emit(report.clone());
            }
        });
    }

    let add_current_draft = {
        let tags = tags.clone();
        let draft = draft.clone();
        let error = error.clone();
        let emit_changes = emit_changes.clone();
        let allow_duplicates = props.allow_duplicates;
        let allow_custom_tags = props.allow_custom_tags;
        let suggestions = props.suggestions.clone();
        let max_tags = props.max_tags;

        Callback::from(move |_| {
            if commit_tag_add(TagAddRequest {
                tags: &tags,
                error: &error,
                emit_changes: &emit_changes,
                raw_tag: draft.as_str(),
                allow_duplicates,
                allow_custom_tags,
                suggestions: &suggestions,
                max_tags,
            }) {
                draft.set(String::new());
            }
        })
    };

    let on_input = {
        let draft = draft.clone();
        let error = error.clone();

        Callback::from(move |event: InputEvent| {
            let input: HtmlInputElement = event.target_unchecked_into();
            draft.set(input.value());
            error.set(None);
        })
    };

    let on_key_down = {
        let tags = tags.clone();
        let draft = draft.clone();
        let error = error.clone();
        let emit_changes = emit_changes.clone();
        let allow_duplicates = props.allow_duplicates;
        let allow_custom_tags = props.allow_custom_tags;
        let suggestions = props.suggestions.clone();
        let max_tags = props.max_tags;
        let min_tags = props.min_tags;

        Callback::from(move |event: KeyboardEvent| {
            let key = event.key();
            if key == "Enter" || key == "," {
                event.prevent_default();
                if commit_tag_add(TagAddRequest {
                    tags: &tags,
                    error: &error,
                    emit_changes: &emit_changes,
                    raw_tag: draft.as_str(),
                    allow_duplicates,
                    allow_custom_tags,
                    suggestions: &suggestions,
                    max_tags,
                }) {
                    draft.set(String::new());
                }
            } else if key == "Backspace" && draft.is_empty() && can_remove_more(&tags, min_tags) {
                let mut next_tags = (*tags).clone();
                next_tags.pop();
                tags.set(next_tags.clone());
                error.set(None);
                emit_changes.emit(next_tags);
            }
        })
    };

    let on_suggestion_add = {
        let tags = tags.clone();
        let error = error.clone();
        let emit_changes = emit_changes.clone();
        let allow_duplicates = props.allow_duplicates;
        let allow_custom_tags = props.allow_custom_tags;
        let suggestions = props.suggestions.clone();
        let max_tags = props.max_tags;

        Callback::from(move |suggestion: String| {
            commit_tag_add(TagAddRequest {
                tags: &tags,
                error: &error,
                emit_changes: &emit_changes,
                raw_tag: &suggestion,
                allow_duplicates,
                allow_custom_tags,
                suggestions: &suggestions,
                max_tags,
            });
        })
    };

    html! {
        <div class="space-y-2 text-left">
            <Label for_id={label_for_id} text={props.label.clone()} />
            if let Some(helper_text) = &props.helper_text {
                <p id={helper_id.clone()} class="text-sm text-gray-600 dark:text-gray-400">
                    { helper_text }
                </p>
            }

            <div
                class={classes!(
                    "flex",
                    "min-h-10",
                    "w-full",
                    "flex-wrap",
                    "items-center",
                    "gap-2",
                    "rounded-lg",
                    "border",
                    if (*error).is_some() { "border-red-500" } else { "border-gray-300" },
                    "bg-white",
                    "px-3",
                    "py-2",
                    "shadow-sm",
                    "transition",
                    "focus-within:border-primary",
                    "focus-within:ring-2",
                    "focus-within:ring-primary",
                    "dark:border-gray-600",
                    "dark:bg-gray-800",
                    "dark:focus-within:border-primary-dark",
                    "dark:focus-within:ring-primary-dark",
                )}
            >
                { for tags.iter().enumerate().map(|(index, tag)| {
                    let tags = tags.clone();
                    let error = error.clone();
                    let emit_changes = emit_changes.clone();
                    let remove_disabled = !can_remove;
                    let on_remove = Callback::from(move |_| {
                        if remove_disabled {
                            error.set(Some("Minimum tag count reached.".to_owned()));
                            return;
                        }
                        let mut next_tags = (*tags).clone();
                        if remove_tag_at(&mut next_tags, index) {
                            tags.set(next_tags.clone());
                            error.set(None);
                            emit_changes.emit(next_tags);
                        }
                    });

                    html! {
                        <Chip
                            variant={ChipVariant::Neutral}
                            size={ChipSize::Small}
                            removable={true}
                            disabled={remove_disabled}
                            on_remove={Some(on_remove)}
                            remove_aria_label={Some(AttrValue::from(format!("Remove tag {tag}")))}
                            remove_title="Remove tag"
                        >
                            <span>{ tag.clone() }</span>
                        </Chip>
                    }
                }) }

                if props.allow_custom_tags {
                    <span class="inline-flex min-w-56 flex-1 items-center gap-2">
                        <input
                            id={input_id}
                            type="text"
                            value={(*draft).clone()}
                            placeholder={tag_input_placeholder(can_add, &props.placeholder)}
                            class="min-w-0 flex-1 border-0 bg-transparent p-0 text-sm text-gray-900 outline-none placeholder:text-gray-400 focus:ring-0 disabled:cursor-not-allowed dark:text-gray-100"
                            oninput={on_input}
                            onkeydown={on_key_down}
                            disabled={!can_add}
                            aria-invalid={AttrValue::from((*error).is_some().to_string())}
                            aria-describedby={join_ids(helper_id.clone(), error_id.clone())}
                        />

                        <Button
                            button_type={ButtonType::Ghost}
                            size={ButtonSize::Small}
                            disabled={!draft_can_be_added}
                            on_click={add_current_draft}
                            class="h-8 shrink-0 shadow-none"
                            aria_label={Some(AttrValue::from("Add tag"))}
                        >
                            <AddIcon size={14} decorative=true />
                            <span>{ "Add" }</span>
                        </Button>
                    </span>
                }
            </div>

            if let Some(error_message) = (*error).as_ref() {
                <p id={error_id} class="text-xs font-medium text-red-600 dark:text-red-300" role="alert">
                    { error_message.clone() }
                </p>
            }

            <TagsSuggestions
                suggestions={props.suggestions.clone()}
                tags={(*tags).clone()}
                can_add={can_add}
                allow_duplicates={props.allow_duplicates}
                on_add={on_suggestion_add}
            />

            <JsonBackedHiddenInput
                id={props.id.clone()}
                name={props.name.clone()}
                value={AttrValue::from(hidden_json)}
            />
            if props.block_form_submit_when_invalid {
                <JsonFormValidityBridge
                    id={props.id.clone()}
                    label={props.label.clone()}
                    is_valid={validation_error.is_none()}
                    validation_message={validation_error.clone().map(AttrValue::from)}
                    on_validation_requested={Some({
                        let error = error.clone();
                        let validation_error = validation_error.clone();
                        Callback::from(move |_| {
                            if let Some(validation_error) = &validation_error {
                                error.set(Some(validation_error.clone()));
                            }
                        })
                    })}
                />
            }
        </div>
    }
}

pub(crate) fn tags_validity(validation_error: Option<&str>) -> JsonInputValidity {
    JsonInputValidity {
        is_valid: validation_error.is_none(),
        errors: validation_error
            .map(|message| JsonInputError {
                path: "$".to_owned(),
                message: message.to_owned(),
                kind: tags_error_kind(message),
            })
            .into_iter()
            .collect(),
    }
}

fn tags_error_kind(message: &str) -> JsonInputErrorKind {
    if message.starts_with("Add at least") {
        JsonInputErrorKind::RequiredObjectEmpty
    } else {
        JsonInputErrorKind::UnsupportedType
    }
}

pub(crate) fn tags_validation_report(
    id: &AttrValue,
    name: &Option<AttrValue>,
    label: &AttrValue,
    validation_error: Option<&str>,
) -> JsonBackedValidationReport {
    JsonBackedValidationReport {
        is_valid: validation_error.is_none(),
        field_id: id.clone(),
        field_name: submitted_name(id, name),
        label: (!label.as_str().trim().is_empty()).then(|| label.clone()),
        issues: validation_error
            .map(|message| JsonBackedValidationIssue {
                message: AttrValue::from(message.to_owned()),
                label: (!label.as_str().trim().is_empty()).then(|| label.clone()),
                location: None,
                path: Some(AttrValue::from("$")),
                row_index: None,
                key: None,
                raw_path: Some(AttrValue::from("$")),
            })
            .into_iter()
            .collect(),
    }
}

fn join_ids(first: Option<String>, second: Option<String>) -> Option<AttrValue> {
    let ids = [first, second]
        .into_iter()
        .flatten()
        .filter(|id| !id.is_empty())
        .collect::<Vec<_>>();

    (!ids.is_empty()).then(|| AttrValue::from(ids.join(" ")))
}

fn tag_input_placeholder(can_add: bool, placeholder: &AttrValue) -> AttrValue {
    if !can_add {
        AttrValue::from("Maximum tags reached")
    } else {
        placeholder.clone()
    }
}
