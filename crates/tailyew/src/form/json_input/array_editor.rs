use super::array_item_row::JsonArrayItemRow;
use super::controls::{
    JsonRowKind, add_row_action_class, add_row_container_class, array_grid_columns_class,
    editor_header_class, editor_surface_class, run_on_keyboard_click,
    run_on_mouse_down_before_blur, should_show_editor_header, show_type_column_for_paths,
};
use super::model::{is_at_max_depth, new_model_for_type, node_kind_mut_at_path};
use super::policy::{default_new_type_for_path, resolve_policy};
use super::types::*;
use crate::{AddIcon, Button, ButtonSize, ButtonType};
use uuid::Uuid;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub(super) struct JsonArrayEditorProps {
    pub node: JsonModel,
    pub path: Vec<JsonPathSegment>,
    pub policy_path: JsonInputPath,
    pub depth: usize,
    pub config: JsonInputConfig,
    pub issues: Vec<JsonInputIssue>,
    pub update_model: Callback<ModelUpdater>,
}

#[component(JsonArrayEditor)]
pub(super) fn json_array_editor(props: &JsonArrayEditorProps) -> Html {
    let items = match &props.node.kind {
        JsonNodeKind::Array(items) => items.clone(),
        _ => Vec::new(),
    };
    let at_max_depth = is_at_max_depth(&props.config, props.depth);
    let node_policy = resolve_policy(&props.config, &props.policy_path);
    let child_policy_paths = items
        .iter()
        .enumerate()
        .map(|(index, _)| props.policy_path.child_index(index))
        .collect::<Vec<_>>();
    let show_type_column =
        show_type_column_for_paths(&props.config, &child_policy_paths, JsonRowKind::Array);
    let header_class = editor_header_class(props.depth, array_grid_columns_class(show_type_column));
    let empty_state_class = if props.depth == 0 {
        "rounded-lg border border-dashed border-gray-300 bg-gray-50 px-4 py-5 text-sm dark:border-gray-700 dark:bg-gray-800"
    } else {
        "px-3 py-3 text-sm text-gray-600 dark:text-gray-400"
    };

    let on_add_item = {
        let update_model = props.update_model.clone();
        let path = props.path.clone();
        let policy_path = props.policy_path.clone();
        let config = props.config.clone();
        let next_index = items.len();

        Callback::from(move |_| {
            let path = path.clone();
            let child_policy_path = policy_path.child_index(next_index);
            let new_type = default_new_type_for_path(&config, &child_policy_path);
            update_model.emit(Box::new(move |root| {
                if let Some(JsonNodeKind::Array(items)) = node_kind_mut_at_path(root, &path) {
                    items.push(JsonArrayItemNode {
                        id: Uuid::new_v4(),
                        value: new_model_for_type(new_type, false),
                    });
                }
            }));
        })
    };
    let on_add_item_mouse_down = run_on_mouse_down_before_blur(on_add_item.clone());
    let on_add_item_keyboard_click = run_on_keyboard_click(on_add_item.clone());

    html! {
        <div class={editor_surface_class(props.depth)}>
            if items.is_empty() {
                <div class={empty_state_class}>
                    <p class="font-medium text-gray-800 dark:text-gray-100">{ "No items yet." }</p>
                    <p class="mt-1 text-gray-600 dark:text-gray-400">
                        { "Add items to build this JSON array." }
                    </p>
                    if props.config.display_buttons && node_policy.allow_add_children && !at_max_depth {
                        <div class="mt-4">
                            <Button
                                button_type={ButtonType::Ghost}
                                size={ButtonSize::Small}
                                on_click={on_add_item_keyboard_click.clone()}
                                on_mouse_down={on_add_item_mouse_down.clone()}
                                class={add_row_action_class(props.depth)}
                            >
                                <AddIcon size={16} decorative=true />
                                <span>{ props.config.add_item_label.clone() }</span>
                            </Button>
                        </div>
                    }
                </div>
            } else {
                if should_show_editor_header(props.depth, items.len()) {
                    <div class={header_class}>
                        <div>{ "Item" }</div>
                        if show_type_column {
                            <div>{ "Type" }</div>
                        }
                        <div>{ "Value" }</div>
                        <div class="text-right">{ "Actions" }</div>
                    </div>
                }

                <div>
                    { for items.into_iter().enumerate().map(|(index, item)| {
                        let key = item.id.to_string();
                        html! {
                                <JsonArrayItemRow
                                    key={key}
                                    item={item}
                                    index={index}
                                    parent_path={props.path.clone()}
                                    policy_path={props.policy_path.child_index(index)}
                                    depth={props.depth}
                                config={props.config.clone()}
                                issues={props.issues.clone()}
                                update_model={props.update_model.clone()}
                            />
                        }
                    }) }
                </div>

                if props.config.display_buttons && node_policy.allow_add_children {
                    <div class={add_row_container_class(props.depth)}>
                        <Button
                            button_type={ButtonType::Ghost}
                            size={ButtonSize::Small}
                            on_click={on_add_item_keyboard_click}
                            on_mouse_down={on_add_item_mouse_down}
                            disabled={at_max_depth}
                            class={add_row_action_class(props.depth)}
                            aria_label={Some(props.config.add_item_label.clone())}
                        >
                            <AddIcon size={16} decorative=true />
                            <span>{ props.config.add_item_label.clone() }</span>
                        </Button>
                        if at_max_depth {
                            <span class="text-xs text-gray-500 dark:text-gray-400">
                                { "Maximum nesting depth reached." }
                            </span>
                        }
                    </div>
                }
            }
        </div>
    }
}
