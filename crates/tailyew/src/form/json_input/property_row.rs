use super::array_editor::JsonArrayEditor;
use super::controls::{
    JsonCompositeDisclosure, JsonRowKind, action_cell_class, input_class, input_size_for_density,
    json_row_error_grid_class, json_row_error_text_class, json_row_grid_class, nested_panel_class,
    readonly_key_class, render_type_select, row_action_button_class, row_action_size_for_density,
    row_wrapper_class, run_on_keyboard_click, run_on_mouse_down_before_blur,
    show_type_column_for_path, value_issue_for_node,
};
use super::model::{
    append_property_path, display_key, is_key_error, node_kind_mut_at_path, node_mut_at_path,
    property_mut_at_path, restore_replaced_kind,
};
use super::object_editor::JsonObjectEditor;
use super::policy::resolve_policy;
use super::types::*;
use super::value_editor::JsonValueEditor;
use crate::{Button, ButtonSize, ButtonType, DeleteIcon, Input, InputType};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub(super) struct JsonPropertyRowProps {
    pub property: JsonPropertyNode,
    pub sibling_count: usize,
    pub parent_path: Vec<JsonPathSegment>,
    pub policy_path: JsonInputPath,
    pub depth: usize,
    pub config: JsonInputConfig,
    pub issues: Vec<JsonInputIssue>,
    pub update_model: Callback<ModelUpdater>,
    pub focus_key: bool,
}

#[component(JsonPropertyRow)]
pub(super) fn json_property_row(props: &JsonPropertyRowProps) -> Html {
    let property_path = append_property_path(&props.parent_path, props.property.id);
    let property_name = display_key(&props.property.key);
    let is_composite = matches!(
        props.property.value.kind,
        JsonNodeKind::Object(_) | JsonNodeKind::Array(_)
    );
    let property_policy = resolve_policy(&props.config, &props.policy_path);
    let parent_policy = resolve_policy(&props.config, &props.policy_path.parent());
    let show_type_column =
        show_type_column_for_path(&props.config, &props.policy_path, JsonRowKind::Object);
    let panel_id = format!("json-property-panel-{}", props.property.id);
    let key_ref = use_node_ref();
    let key_issue = props
        .issues
        .iter()
        .find(|issue| issue.node_id == Some(props.property.id) && is_key_error(&issue.kind))
        .cloned();
    let value_issue = value_issue_for_node(&props.issues, &props.property.value);
    let key_error_id = key_issue
        .as_ref()
        .map(|_| format!("json-key-{}-error", props.property.id));
    let value_error_id = value_issue
        .as_ref()
        .map(|_| format!("json-value-{}-error", props.property.value.id));
    let show_remove = props.config.display_buttons
        && parent_policy.allow_remove_children
        && property_policy.removable
        && (!props.config.require_at_least_one || props.sibling_count > 1);

    {
        let key_ref = key_ref.clone();
        let focus_key = props.focus_key;

        use_effect_with(focus_key, move |focus_key| {
            if *focus_key && let Some(input) = key_ref.cast::<HtmlInputElement>() {
                let _ = input.focus();
            }
        });
    }

    let on_key_change = {
        let update_model = props.update_model.clone();
        let parent_path = props.parent_path.clone();
        let property_id = props.property.id;

        Callback::from(move |value: String| {
            let parent_path = parent_path.clone();
            update_model.emit(Box::new(move |root| {
                if let Some(property) = property_mut_at_path(root, &parent_path, property_id) {
                    property.key = value;
                    property.key_touched = true;
                }
            }));
        })
    };
    let on_key_blur = {
        let update_model = props.update_model.clone();
        let parent_path = props.parent_path.clone();
        let property_id = props.property.id;

        Callback::from(move |_| {
            let parent_path = parent_path.clone();
            update_model.emit(Box::new(move |root| {
                if let Some(property) = property_mut_at_path(root, &parent_path, property_id) {
                    property.key_touched = true;
                }
            }));
        })
    };
    let on_remove = {
        let update_model = props.update_model.clone();
        let parent_path = props.parent_path.clone();
        let property_id = props.property.id;

        Callback::from(move |_| {
            let parent_path = parent_path.clone();
            update_model.emit(Box::new(move |root| {
                if let Some(JsonNodeKind::Object(properties)) =
                    node_kind_mut_at_path(root, &parent_path)
                {
                    properties.retain(|property| property.id != property_id);
                }
            }));
        })
    };
    let on_remove_mouse_down = run_on_mouse_down_before_blur(on_remove.clone());
    let on_remove_keyboard_click = run_on_keyboard_click(on_remove);
    let on_undo_type_change = {
        let update_model = props.update_model.clone();
        let path = property_path.clone();

        Callback::from(move |_| {
            let path = path.clone();
            update_model.emit(Box::new(move |root| {
                if let Some(node) = node_mut_at_path(root, &path) {
                    restore_replaced_kind(node);
                }
            }));
        })
    };

    let row_grid_class =
        json_row_grid_class(show_type_column, props.config.density, JsonRowKind::Object);
    let error_grid_class = json_row_error_grid_class(show_type_column, JsonRowKind::Object);
    let has_row_error = key_issue.is_some() || value_issue.is_some();

    html! {
        <div class={row_wrapper_class(props.depth, is_composite && props.property.value.expanded)}>
            <div class={row_grid_class}>
                if property_policy.key_editable {
                    <Input
                        id={format!("json-key-{}", props.property.id)}
                        label="Property"
                        value={Some(AttrValue::from(props.property.key.clone()))}
                        placeholder={props.config.placeholder_key.clone()}
                        input_type={InputType::Text}
                        on_change={Some(on_key_change)}
                        on_blur={Some(on_key_blur)}
                        disabled={!property_policy.key_editable}
                        size={input_size_for_density(props.config.density)}
                        visually_hidden_label={true}
                        marginless={true}
                        container_class="mb-0"
                        class={input_class(&props.config, key_issue.is_some())}
                        aria_invalid={Some(key_issue.is_some())}
                        aria_describedby={key_error_id.as_ref().map(|id| AttrValue::from(id.clone()))}
                        node_ref={key_ref}
                    />
                } else {
                    <div class={readonly_key_class(&props.config)} title={props.property.key.clone()}>
                        { props.property.key.clone() }
                    </div>
                }

                if show_type_column {
                    { render_type_select(
                        &props.property.value,
                        &property_path,
                        &props.policy_path,
                        &props.config,
                        props.update_model.clone(),
                        "Property value type",
                    ) }
                }

                if is_composite {
                    <JsonCompositeDisclosure
                        node={props.property.value.clone()}
                        label={property_name.clone()}
                        panel_id={panel_id.clone()}
                        path={property_path.clone()}
                        config={props.config.clone()}
                        update_model={props.update_model.clone()}
                    />
                } else {
                    <JsonValueEditor
                        node={props.property.value.clone()}
                        path={property_path.clone()}
                        depth={props.depth + 1}
                        config={props.config.clone()}
                        issues={props.issues.clone()}
                        aria_label_prefix={format!("Value for {property_name}")}
                        error_id={value_error_id.clone()}
                        policy_path={props.policy_path.clone()}
                        update_model={props.update_model.clone()}
                    />
                }

                <div class={action_cell_class(&props.config)}>
                    if show_remove {
                        <Button
                            button_type={ButtonType::DangerGhost}
                            size={row_action_size_for_density(props.config.density)}
                            on_click={on_remove_keyboard_click}
                            on_mouse_down={on_remove_mouse_down}
                            class={row_action_button_class()}
                            aria_label={Some(AttrValue::from(format!("Remove property {property_name}")))}
                            title={Some(AttrValue::from("Remove property"))}
                        >
                            <DeleteIcon size={14} decorative=true />
                        </Button>
                    }
                </div>
            </div>

            if has_row_error {
                <div class={error_grid_class}>
                    {
                        if let (Some(issue), Some(error_id)) = (&key_issue, &key_error_id) {
                            html! {
                                <p id={error_id.clone()} class={json_row_error_text_class()}>
                                    { issue.message.clone() }
                                </p>
                            }
                        } else {
                            html! { <div class="hidden md:block"></div> }
                        }
                    }

                    if show_type_column {
                        <div class="hidden md:block"></div>
                    }

                    {
                        if let (Some(issue), Some(error_id)) = (&value_issue, &value_error_id) {
                            html! {
                                <p id={error_id.clone()} class={json_row_error_text_class()}>
                                    { issue.message.clone() }
                                </p>
                            }
                        } else {
                            html! { <div class="hidden md:block"></div> }
                        }
                    }

                    <div class="hidden md:block"></div>
                </div>
            }

            if props.property.value.replaced_kind.is_some() {
                <div class="flex flex-wrap items-center gap-2 px-3 pb-2 text-xs text-amber-800 dark:text-amber-200">
                    <span>{ "Type changed." }</span>
                    <Button
                        button_type={ButtonType::Ghost}
                        size={ButtonSize::Small}
                        on_click={on_undo_type_change}
                        class="py-1 px-2 text-xs"
                    >
                        { "Undo" }
                    </Button>
                </div>
            }

            if is_composite && props.property.value.expanded {
                <div id={panel_id} class="px-3 pb-3">
                    <div class={nested_panel_class(props.depth + 1)}>
                        {
                            match &props.property.value.kind {
                                JsonNodeKind::Object(_) => html! {
                                    <JsonObjectEditor
                                        node={props.property.value.clone()}
                                        path={property_path.clone()}
                                        policy_path={props.policy_path.clone()}
                                        depth={props.depth + 1}
                                        config={props.config.clone()}
                                        issues={props.issues.clone()}
                                        update_model={props.update_model.clone()}
                                    />
                                },
                                JsonNodeKind::Array(_) => html! {
                                    <JsonArrayEditor
                                        node={props.property.value.clone()}
                                        path={property_path.clone()}
                                        policy_path={props.policy_path.clone()}
                                        depth={props.depth + 1}
                                        config={props.config.clone()}
                                        issues={props.issues.clone()}
                                        update_model={props.update_model.clone()}
                                    />
                                },
                                _ => Html::default(),
                            }
                        }
                    </div>
                </div>
            }
        </div>
    }
}
