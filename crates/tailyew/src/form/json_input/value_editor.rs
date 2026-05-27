use super::controls::{
    input_class, input_size_for_density, null_value_class, summary_badge_class, value_shell_class,
};
use super::model::{node_mut_at_path, summarize_value};
use super::policy::resolve_policy;
use super::types::*;
use crate::{Checkbox, Input, InputType};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub(super) struct JsonValueEditorProps {
    pub node: JsonModel,
    pub path: Vec<JsonPathSegment>,
    pub depth: usize,
    pub config: JsonInputConfig,
    pub issues: Vec<JsonInputIssue>,
    pub aria_label_prefix: AttrValue,
    pub error_id: Option<String>,
    pub policy_path: JsonInputPath,
    pub update_model: Callback<ModelUpdater>,
}

#[component(JsonValueEditor)]
pub(super) fn json_value_editor(props: &JsonValueEditorProps) -> Html {
    match &props.node.kind {
        JsonNodeKind::String(value) => render_string_editor(props, value),
        JsonNodeKind::Number { raw } => render_number_editor(props, raw),
        JsonNodeKind::Boolean(value) => render_boolean_editor(props, *value),
        JsonNodeKind::Null => html! {
            <div class={null_value_class(&props.config)}>
                { "null" }
            </div>
        },
        JsonNodeKind::Object(_) | JsonNodeKind::Array(_) => html! {
            <span class={summary_badge_class(&props.config)}>
                { summarize_value(&props.node) }
            </span>
        },
    }
}

fn render_string_editor(props: &JsonValueEditorProps, value: &str) -> Html {
    let issue = props
        .issues
        .iter()
        .find(|issue| {
            issue.node_id == Some(props.node.id) && issue.kind == JsonInputErrorKind::EmptyValue
        })
        .cloned();
    let on_change = {
        let update_model = props.update_model.clone();
        let path = props.path.clone();

        Callback::from(move |next_value: String| {
            let path = path.clone();
            update_model.emit(Box::new(move |root| {
                if let Some(node) = node_mut_at_path(root, &path) {
                    node.kind = JsonNodeKind::String(next_value);
                    node.touched = true;
                }
            }));
        })
    };
    let policy = resolve_policy(&props.config, &props.policy_path);

    html! {
        <Input
            id={format!("json-string-{}", props.node.id)}
            label={props.aria_label_prefix.clone()}
            value={Some(AttrValue::from(value.to_owned()))}
            placeholder={props.config.placeholder_value.clone()}
            input_type={InputType::Text}
            on_change={Some(on_change)}
            disabled={!policy.value_editable}
            size={input_size_for_density(props.config.density)}
            visually_hidden_label={true}
            marginless={true}
            container_class="mb-0"
            class={input_class(&props.config, issue.is_some())}
            aria_invalid={Some(issue.is_some())}
            aria_describedby={props.error_id.as_ref().map(|id| AttrValue::from(id.clone()))}
        />
    }
}

fn render_number_editor(props: &JsonValueEditorProps, raw: &str) -> Html {
    let issue = props
        .issues
        .iter()
        .find(|issue| {
            issue.node_id == Some(props.node.id) && issue.kind == JsonInputErrorKind::InvalidNumber
        })
        .cloned();
    let on_change = {
        let update_model = props.update_model.clone();
        let path = props.path.clone();

        Callback::from(move |next_value: String| {
            let path = path.clone();
            update_model.emit(Box::new(move |root| {
                if let Some(node) = node_mut_at_path(root, &path) {
                    node.kind = JsonNodeKind::Number { raw: next_value };
                    node.touched = true;
                }
            }));
        })
    };
    let policy = resolve_policy(&props.config, &props.policy_path);

    html! {
        <Input
            id={format!("json-number-{}", props.node.id)}
            label={props.aria_label_prefix.clone()}
            value={Some(AttrValue::from(raw.to_owned()))}
            placeholder="0"
            input_type={InputType::Text}
            on_change={Some(on_change)}
            disabled={!policy.value_editable}
            size={input_size_for_density(props.config.density)}
            visually_hidden_label={true}
            marginless={true}
            container_class="mb-0"
            class={input_class(&props.config, issue.is_some())}
            aria_invalid={Some(issue.is_some())}
            aria_describedby={props.error_id.as_ref().map(|id| AttrValue::from(id.clone()))}
        />
    }
}

fn render_boolean_editor(props: &JsonValueEditorProps, value: bool) -> Html {
    let on_change = {
        let update_model = props.update_model.clone();
        let path = props.path.clone();

        Callback::from(move |next_value: bool| {
            let path = path.clone();
            update_model.emit(Box::new(move |root| {
                if let Some(node) = node_mut_at_path(root, &path) {
                    node.kind = JsonNodeKind::Boolean(next_value);
                    node.touched = true;
                }
            }));
        })
    };
    let policy = resolve_policy(&props.config, &props.policy_path);

    html! {
        <div class={value_shell_class(&props.config)}>
            <Checkbox
                id={format!("json-bool-{}", props.node.id)}
                label={boolean_display_text(value)}
                checked={value}
                on_change={Some(on_change)}
                disabled={!policy.value_editable}
                aria_label={Some(props.aria_label_prefix.clone())}
            />
        </div>
    }
}

pub(super) fn boolean_display_text(value: bool) -> &'static str {
    if value { "true" } else { "false" }
}
