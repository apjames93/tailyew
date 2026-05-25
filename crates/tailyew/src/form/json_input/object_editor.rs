use super::controls::{
    JsonRowKind, add_row_action_class, add_row_container_class, editor_header_class,
    editor_surface_class, object_grid_columns_class, run_on_keyboard_click,
    run_on_mouse_down_before_blur, should_show_editor_header, show_type_column_for_paths,
};
use super::model::{count_children, is_at_max_depth, new_model_for_type, node_kind_mut_at_path};
use super::policy::{default_new_type_for_path, resolve_policy};
use super::property_row::JsonPropertyRow;
use super::types::*;
use crate::{AddIcon, Button, ButtonSize, ButtonType};
use uuid::Uuid;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub(super) struct JsonObjectEditorProps {
    pub node: JsonModel,
    pub path: Vec<JsonPathSegment>,
    pub policy_path: JsonInputPath,
    pub depth: usize,
    pub config: JsonInputConfig,
    pub issues: Vec<JsonInputIssue>,
    pub update_model: Callback<ModelUpdater>,
}

#[component(JsonObjectEditor)]
pub(super) fn json_object_editor(props: &JsonObjectEditorProps) -> Html {
    let properties = match &props.node.kind {
        JsonNodeKind::Object(properties) => properties.clone(),
        _ => Vec::new(),
    };
    let at_max_depth = is_at_max_depth(&props.config, props.depth);
    let node_policy = resolve_policy(&props.config, &props.policy_path);
    let focus_property_id = use_state(|| None::<Uuid>);
    let child_policy_paths = properties
        .iter()
        .map(|property| props.policy_path.child_key(property.key.clone()))
        .collect::<Vec<_>>();
    let show_type_column =
        show_type_column_for_paths(&props.config, &child_policy_paths, JsonRowKind::Object);
    let header_class =
        editor_header_class(props.depth, object_grid_columns_class(show_type_column));
    let empty_state_class = if props.depth == 0 {
        "rounded-lg border border-dashed border-gray-300 bg-gray-50 px-4 py-6 text-sm dark:border-gray-700 dark:bg-gray-800"
    } else {
        "px-3 py-3 text-sm text-gray-600 dark:text-gray-400"
    };

    let on_add_property = {
        let update_model = props.update_model.clone();
        let path = props.path.clone();
        let policy_path = props.policy_path.clone();
        let config = props.config.clone();
        let focus_property_id = focus_property_id.clone();

        Callback::from(move |_| {
            let path = path.clone();
            let property_id = Uuid::new_v4();
            let child_policy_path = policy_path.child_key("");
            let new_type = default_new_type_for_path(&config, &child_policy_path);
            focus_property_id.set(Some(property_id));
            update_model.emit(Box::new(move |root| {
                if let Some(JsonNodeKind::Object(properties)) = node_kind_mut_at_path(root, &path) {
                    properties.push(JsonPropertyNode {
                        id: property_id,
                        key: String::new(),
                        key_touched: false,
                        value: new_model_for_type(new_type, false),
                    });
                }
            }));
        })
    };
    let on_add_property_mouse_down = run_on_mouse_down_before_blur(on_add_property.clone());
    let on_add_property_keyboard_click = run_on_keyboard_click(on_add_property.clone());

    html! {
        <div class={editor_surface_class(props.depth)}>
            if properties.is_empty() {
                <div class={empty_state_class}>
                    <p class="font-medium text-gray-800 dark:text-gray-100">{ "No properties yet." }</p>
                    <p class="mt-1 text-gray-600 dark:text-gray-400">
                        { "Add properties to build the JSON object that will be submitted with this form." }
                    </p>
                    if props.config.display_buttons && node_policy.allow_add_children && !at_max_depth {
                        <div class="mt-4">
                            <Button
                                button_type={ButtonType::Ghost}
                                size={ButtonSize::Small}
                                on_click={on_add_property_keyboard_click.clone()}
                                on_mouse_down={on_add_property_mouse_down.clone()}
                                class={add_row_action_class(props.depth)}
                            >
                                <AddIcon size={16} decorative=true />
                                <span>{ props.config.add_property_label.clone() }</span>
                            </Button>
                        </div>
                    }
                </div>
            } else {
                if should_show_editor_header(props.depth, properties.len()) {
                    <div class={header_class}>
                        <div>{ "Property" }</div>
                        if show_type_column {
                            <div>{ "Type" }</div>
                        }
                        <div>{ "Value" }</div>
                        <div class="text-right">{ "Actions" }</div>
                    </div>
                }

                <div>
                    { for properties.into_iter().map(|property| {
                        let key = property.id.to_string();
                        let focus_key = *focus_property_id == Some(property.id);
                        let policy_path = props.policy_path.child_key(property.key.clone());
                        html! {
                            <JsonPropertyRow
                                key={key}
                                property={property}
                                sibling_count={count_children(&props.node)}
                                parent_path={props.path.clone()}
                                policy_path={policy_path}
                                depth={props.depth}
                                config={props.config.clone()}
                                issues={props.issues.clone()}
                                update_model={props.update_model.clone()}
                                focus_key={focus_key}
                            />
                        }
                    }) }
                </div>

                if props.config.display_buttons && node_policy.allow_add_children {
                    <div class={add_row_container_class(props.depth)}>
                        <Button
                            button_type={ButtonType::Ghost}
                            size={ButtonSize::Small}
                            on_click={on_add_property_keyboard_click}
                            on_mouse_down={on_add_property_mouse_down}
                            disabled={at_max_depth}
                            class={add_row_action_class(props.depth)}
                            aria_label={Some(props.config.add_property_label.clone())}
                        >
                            <AddIcon size={16} decorative=true />
                            <span>{ props.config.add_property_label.clone() }</span>
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
