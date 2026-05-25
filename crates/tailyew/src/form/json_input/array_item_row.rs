use super::array_editor::JsonArrayEditor;
use super::controls::{
    JsonCompositeDisclosure, JsonRowKind, action_cell_class, json_row_control_wrapper_class,
    json_row_error_grid_class, json_row_error_text_class, json_row_grid_class, nested_panel_class,
    render_type_select, row_action_button_class, row_action_size_for_density, row_wrapper_class,
    run_on_keyboard_click, run_on_mouse_down_before_blur, value_issue_for_node,
};
use super::model::{
    append_array_path, node_kind_mut_at_path, node_mut_at_path, restore_replaced_kind,
};
use super::object_editor::JsonObjectEditor;
use super::policy::resolve_policy;
use super::types::*;
use super::value_editor::JsonValueEditor;
use crate::{Button, ButtonSize, ButtonType, DeleteIcon};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub(super) struct JsonArrayItemRowProps {
    pub item: JsonArrayItemNode,
    pub index: usize,
    pub parent_path: Vec<JsonPathSegment>,
    pub policy_path: JsonInputPath,
    pub depth: usize,
    pub config: JsonInputConfig,
    pub issues: Vec<JsonInputIssue>,
    pub update_model: Callback<ModelUpdater>,
}

#[component(JsonArrayItemRow)]
pub(super) fn json_array_item_row(props: &JsonArrayItemRowProps) -> Html {
    let item_path = append_array_path(&props.parent_path, props.item.id);
    let is_composite = matches!(
        props.item.value.kind,
        JsonNodeKind::Object(_) | JsonNodeKind::Array(_)
    );
    let item_policy = resolve_policy(&props.config, &props.policy_path);
    let parent_policy = resolve_policy(&props.config, &props.policy_path.parent());
    let show_type_column = super::controls::show_type_column_for_path(
        &props.config,
        &props.policy_path,
        JsonRowKind::Array,
    );
    let panel_id = format!("json-array-item-panel-{}", props.item.id);
    let item_label = format!("item {}", props.index + 1);
    let value_issue = value_issue_for_node(&props.issues, &props.item.value);
    let value_error_id = value_issue
        .as_ref()
        .map(|_| format!("json-value-{}-error", props.item.value.id));
    let on_remove = {
        let update_model = props.update_model.clone();
        let parent_path = props.parent_path.clone();
        let item_id = props.item.id;

        Callback::from(move |_| {
            let parent_path = parent_path.clone();
            update_model.emit(Box::new(move |root| {
                if let Some(JsonNodeKind::Array(items)) = node_kind_mut_at_path(root, &parent_path)
                {
                    items.retain(|item| item.id != item_id);
                }
            }));
        })
    };
    let on_remove_mouse_down = run_on_mouse_down_before_blur(on_remove.clone());
    let on_remove_keyboard_click = run_on_keyboard_click(on_remove);
    let on_undo_type_change = {
        let update_model = props.update_model.clone();
        let path = item_path.clone();

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
        json_row_grid_class(show_type_column, props.config.density, JsonRowKind::Array);
    let error_grid_class = json_row_error_grid_class(show_type_column, JsonRowKind::Array);

    html! {
        <div class={row_wrapper_class(props.depth, is_composite && props.item.value.expanded)}>
            <div class={row_grid_class}>
                <div class={json_row_control_wrapper_class(props.config.density)}>
                    <span class="sr-only">
                        { format!("Item {}", props.index + 1) }
                    </span>
                    <span class="inline-flex h-7 min-w-[2rem] items-center justify-center rounded-full bg-gray-100 px-2 text-xs font-medium text-gray-700 dark:bg-gray-800 dark:text-gray-300">
                        { format!("#{}", props.index + 1) }
                    </span>
                </div>

                if show_type_column {
                    { render_type_select(
                        &props.item.value,
                        &item_path,
                        &props.policy_path,
                        &props.config,
                        props.update_model.clone(),
                        "Array item type",
                    ) }
                }

                if is_composite {
                    <JsonCompositeDisclosure
                        node={props.item.value.clone()}
                        label={item_label.clone()}
                        panel_id={panel_id.clone()}
                        path={item_path.clone()}
                        config={props.config.clone()}
                        update_model={props.update_model.clone()}
                    />
                } else {
                    <JsonValueEditor
                        node={props.item.value.clone()}
                        path={item_path.clone()}
                        depth={props.depth + 1}
                        config={props.config.clone()}
                        issues={props.issues.clone()}
                        aria_label_prefix={format!("Array item {}", props.index + 1)}
                        error_id={value_error_id.clone()}
                        policy_path={props.policy_path.clone()}
                        update_model={props.update_model.clone()}
                    />
                }

                <div class={action_cell_class(&props.config)}>
                    if props.config.display_buttons && parent_policy.allow_remove_children && item_policy.removable {
                        <Button
                            button_type={ButtonType::DangerGhost}
                            size={row_action_size_for_density(props.config.density)}
                            on_click={on_remove_keyboard_click}
                            on_mouse_down={on_remove_mouse_down}
                            class={row_action_button_class()}
                            aria_label={Some(AttrValue::from(format!("Remove array item {}", props.index + 1)))}
                            title={Some(AttrValue::from("Remove item"))}
                        >
                            <DeleteIcon size={14} decorative=true />
                        </Button>
                    }
                </div>
            </div>

            if value_issue.is_some() {
                <div class={error_grid_class}>
                    <div class="hidden md:block"></div>

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

            if props.item.value.replaced_kind.is_some() {
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

            if is_composite && props.item.value.expanded {
                <div id={panel_id} class="px-3 pb-3">
                    <div class={nested_panel_class(props.depth + 1)}>
                        {
                            match &props.item.value.kind {
                                JsonNodeKind::Object(_) => html! {
                                    <JsonObjectEditor
                                        node={props.item.value.clone()}
                                        path={item_path.clone()}
                                        policy_path={props.policy_path.clone()}
                                        depth={props.depth + 1}
                                        config={props.config.clone()}
                                        issues={props.issues.clone()}
                                        update_model={props.update_model.clone()}
                                    />
                                },
                                JsonNodeKind::Array(_) => html! {
                                    <JsonArrayEditor
                                        node={props.item.value.clone()}
                                        path={item_path.clone()}
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
