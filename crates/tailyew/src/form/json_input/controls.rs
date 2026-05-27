use super::model::{convert_node_type, node_mut_at_path, summarize_value, value_type_for_node};
use super::policy::{allowed_types_for_path, resolve_policy};
use super::types::*;
use crate::{ArrowDownIcon, ButtonSize, InputSize, Select, SelectOption, SelectSize};
use yew::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum JsonRowKind {
    Object,
    Array,
    RootScalar,
}

pub(super) fn show_type_column_for_path(
    config: &JsonInputConfig,
    policy_path: &JsonInputPath,
    row_kind: JsonRowKind,
) -> bool {
    let policy = resolve_policy(config, policy_path);
    if policy.allowed_types.len() > 1 {
        return true;
    }

    matches!(row_kind, JsonRowKind::Object) && !policy.type_editable
}

pub(super) fn show_type_column_for_paths(
    config: &JsonInputConfig,
    policy_paths: &[JsonInputPath],
    row_kind: JsonRowKind,
) -> bool {
    policy_paths
        .iter()
        .any(|path| show_type_column_for_path(config, path, row_kind))
}

pub(super) fn object_grid_columns_class(show_type_column: bool) -> &'static str {
    row_grid_columns_class(JsonRowKind::Object, show_type_column)
}

pub(super) fn array_grid_columns_class(show_type_column: bool) -> &'static str {
    row_grid_columns_class(JsonRowKind::Array, show_type_column)
}

pub(super) fn root_scalar_grid_columns_class(show_type_column: bool) -> &'static str {
    row_grid_columns_class(JsonRowKind::RootScalar, show_type_column)
}

pub(super) fn row_grid_columns_class(
    row_kind: JsonRowKind,
    show_type_column: bool,
) -> &'static str {
    match row_kind {
        JsonRowKind::Object => object_row_grid_columns_class(show_type_column),
        JsonRowKind::Array => array_row_grid_columns_class(show_type_column),
        JsonRowKind::RootScalar => root_scalar_row_grid_columns_class(show_type_column),
    }
}

fn object_row_grid_columns_class(show_type_column: bool) -> &'static str {
    if show_type_column {
        "md:grid-cols-[minmax(8rem,1fr)_7rem_minmax(9rem,2fr)_3rem]"
    } else {
        "md:grid-cols-[minmax(8rem,1fr)_minmax(9rem,2fr)_3rem]"
    }
}

fn array_row_grid_columns_class(show_type_column: bool) -> &'static str {
    if show_type_column {
        "md:grid-cols-[3rem_7rem_minmax(9rem,2fr)_3rem]"
    } else {
        "md:grid-cols-[3rem_minmax(9rem,2fr)_3rem]"
    }
}

fn root_scalar_row_grid_columns_class(show_type_column: bool) -> &'static str {
    if show_type_column {
        "md:grid-cols-[8rem_minmax(18rem,2fr)]"
    } else {
        "md:grid-cols-1"
    }
}

pub(super) fn json_row_control_height_class(density: JsonInputDensity) -> &'static str {
    match density {
        JsonInputDensity::Compact => "h-9",
        JsonInputDensity::Comfortable => "h-10",
    }
}

pub(super) fn control_height_class(density: JsonInputDensity) -> &'static str {
    json_row_control_height_class(density)
}

pub(super) fn control_square_class(density: JsonInputDensity) -> &'static str {
    match density {
        JsonInputDensity::Compact => "h-9 w-9",
        JsonInputDensity::Comfortable => "h-10 w-10",
    }
}

pub(super) fn select_size_for_density(density: JsonInputDensity) -> SelectSize {
    match density {
        JsonInputDensity::Compact => SelectSize::Small,
        JsonInputDensity::Comfortable => SelectSize::Medium,
    }
}

pub(super) fn input_size_for_density(density: JsonInputDensity) -> InputSize {
    match density {
        JsonInputDensity::Compact => InputSize::Small,
        JsonInputDensity::Comfortable => InputSize::Medium,
    }
}

pub(super) fn row_action_size_for_density(density: JsonInputDensity) -> ButtonSize {
    match density {
        JsonInputDensity::Compact => ButtonSize::IconSmall,
        JsonInputDensity::Comfortable => ButtonSize::IconMedium,
    }
}

pub(super) fn run_on_mouse_down_before_blur(
    callback: Callback<MouseEvent>,
) -> Callback<MouseEvent> {
    Callback::from(move |event: MouseEvent| {
        event.prevent_default();
        callback.emit(event);
    })
}

pub(super) fn run_on_keyboard_click(callback: Callback<MouseEvent>) -> Callback<MouseEvent> {
    Callback::from(move |event: MouseEvent| {
        if event.detail() == 0 {
            callback.emit(event);
        }
    })
}

pub(super) fn editor_surface_class(depth: usize) -> Classes {
    if depth == 0 {
        classes!(
            "overflow-hidden",
            "rounded-lg",
            "border",
            "border-gray-200",
            "bg-white",
            "dark:border-gray-700",
            "dark:bg-gray-900"
        )
    } else {
        classes!("space-y-0", "bg-transparent")
    }
}

pub(super) fn row_divider_class(depth: usize) -> &'static str {
    if depth == 0 {
        "border-gray-100 dark:border-gray-800"
    } else {
        "border-gray-100/80 dark:border-gray-800/70"
    }
}

pub(super) fn row_wrapper_class(depth: usize, is_expanded: bool) -> Classes {
    classes!(
        "border-b",
        row_divider_class(depth),
        "transition-colors",
        "last:border-b-0",
        "hover:bg-gray-50/70",
        "dark:hover:bg-gray-800/30",
        is_expanded.then_some("bg-gray-50/40"),
        is_expanded.then_some("dark:bg-gray-800/20")
    )
}

pub(super) fn row_main_grid_class(
    show_type_column: bool,
    _density: JsonInputDensity,
    row_kind: JsonRowKind,
) -> Classes {
    classes!(
        "grid",
        "grid-cols-1",
        "gap-2",
        "px-3",
        "py-2",
        row_grid_columns_class(row_kind, show_type_column),
        "md:items-center"
    )
}

pub(super) fn json_row_grid_class(
    show_type_column: bool,
    density: JsonInputDensity,
    row_kind: JsonRowKind,
) -> Classes {
    row_main_grid_class(show_type_column, density, row_kind)
}

pub(super) fn json_row_error_grid_class(show_type_column: bool, row_kind: JsonRowKind) -> Classes {
    classes!(
        "grid",
        "grid-cols-1",
        "gap-1",
        "px-3",
        "pb-2",
        row_grid_columns_class(row_kind, show_type_column),
        "md:gap-2"
    )
}

pub(super) fn should_show_editor_header(depth: usize, row_count: usize) -> bool {
    depth == 0 || row_count >= 3
}

pub(super) fn editor_header_class(depth: usize, grid_columns_class: &'static str) -> Classes {
    if depth == 0 {
        classes!(
            "hidden",
            "border-b",
            "border-gray-100",
            "bg-gray-50",
            "px-3",
            "py-2",
            "text-xs",
            "font-semibold",
            "uppercase",
            "tracking-wide",
            "text-gray-500",
            "dark:border-gray-800",
            "dark:bg-gray-800",
            "dark:text-gray-400",
            "md:grid",
            grid_columns_class,
            "md:gap-2"
        )
    } else {
        classes!(
            "hidden",
            "px-3",
            "pb-1",
            "pt-2",
            "text-xs",
            "font-semibold",
            "uppercase",
            "tracking-wide",
            "text-gray-400",
            "dark:text-gray-500",
            "md:grid",
            grid_columns_class,
            "md:gap-2"
        )
    }
}

pub(super) fn json_row_control_wrapper_class(density: JsonInputDensity) -> Classes {
    classes!(
        "flex",
        json_row_control_height_class(density),
        "items-center"
    )
}

pub(super) fn json_row_action_cell_class(density: JsonInputDensity) -> Classes {
    classes!(
        "flex",
        json_row_control_height_class(density),
        "items-center",
        "justify-start",
        "md:justify-end"
    )
}

pub(super) fn action_cell_class(config: &JsonInputConfig) -> Classes {
    json_row_action_cell_class(config.density)
}

pub(super) fn input_class(_config: &JsonInputConfig, has_error: bool) -> Classes {
    classes!("min-w-0", has_error.then_some("border-red-500"))
}

pub(super) fn readonly_key_class(config: &JsonInputConfig) -> Classes {
    classes!(
        "flex",
        control_height_class(config.density),
        "min-w-0",
        "items-center",
        "truncate",
        "rounded-md",
        "border",
        "border-gray-200",
        "bg-gray-50",
        "px-3",
        "text-sm",
        "font-medium",
        "text-gray-700",
        "dark:border-gray-700",
        "dark:bg-gray-800",
        "dark:text-gray-200"
    )
}

pub(super) fn json_row_error_text_class() -> Classes {
    classes!(
        "mt-1",
        "text-xs",
        "font-medium",
        "text-red-600",
        "dark:text-red-300"
    )
}

pub(super) fn value_issue_for_node(
    issues: &[JsonInputIssue],
    node: &JsonModel,
) -> Option<JsonInputIssue> {
    issues
        .iter()
        .find(|issue| issue.node_id == Some(node.id))
        .cloned()
}

pub(super) fn type_select_class(_config: &JsonInputConfig) -> Classes {
    classes!("min-w-0")
}

pub(super) fn type_badge_class(config: &JsonInputConfig) -> Classes {
    classes!(
        "inline-flex",
        control_height_class(config.density),
        "w-full",
        "items-center",
        "rounded-md",
        "border",
        "border-gray-200",
        "bg-gray-50",
        "px-3",
        "text-sm",
        "font-medium",
        "text-gray-600",
        "dark:border-gray-700",
        "dark:bg-gray-800",
        "dark:text-gray-300"
    )
}

pub(super) fn value_shell_class(config: &JsonInputConfig) -> Classes {
    classes!(
        "flex",
        control_height_class(config.density),
        "items-center",
        "rounded-md",
        "border",
        "border-gray-200",
        "bg-white",
        "px-3",
        "text-sm",
        "dark:border-gray-700",
        "dark:bg-gray-900"
    )
}

pub(super) fn null_value_class(config: &JsonInputConfig) -> Classes {
    classes!(
        "inline-flex",
        control_height_class(config.density),
        "items-center",
        "rounded-md",
        "border",
        "border-gray-200",
        "bg-gray-50",
        "px-3",
        "font-mono",
        "text-sm",
        "text-gray-600",
        "dark:border-gray-700",
        "dark:bg-gray-800",
        "dark:text-gray-300"
    )
}

pub(super) fn summary_badge_class(config: &JsonInputConfig) -> Classes {
    classes!(
        "inline-flex",
        "min-w-0",
        "items-center",
        "rounded-full",
        "bg-gray-100",
        "px-2.5",
        "text-xs",
        "font-medium",
        "text-gray-700",
        "dark:bg-gray-800",
        "dark:text-gray-300",
        match config.density {
            JsonInputDensity::Compact => "h-7",
            JsonInputDensity::Comfortable => "h-8",
        }
    )
}

pub(super) fn disclosure_button_class(config: &JsonInputConfig) -> Classes {
    classes!(
        "inline-flex",
        control_square_class(config.density),
        "shrink-0",
        "items-center",
        "justify-center",
        "rounded-md",
        "border",
        "border-gray-200",
        "bg-white",
        "text-gray-600",
        "transition-colors",
        "hover:bg-gray-50",
        "focus:outline-none",
        "focus:ring-2",
        "focus:ring-primary",
        "dark:border-gray-700",
        "dark:bg-gray-900",
        "dark:text-gray-300",
        "dark:hover:bg-gray-800",
        "dark:focus:ring-primary-dark"
    )
}

pub(super) fn nested_panel_class(depth: usize) -> &'static str {
    if depth <= 1 {
        "ml-4 border-l border-gray-200 pl-4 dark:border-gray-700"
    } else {
        "ml-3 border-l border-gray-200 pl-3 dark:border-gray-700"
    }
}

pub(super) fn add_row_action_class(_depth: usize) -> Classes {
    classes!(
        "inline-flex",
        "h-9",
        "items-center",
        "gap-2",
        "border",
        "border-transparent",
        "px-2",
        "text-gray-600",
        "shadow-none",
        "hover:border-gray-200",
        "hover:bg-gray-50",
        "dark:text-gray-300",
        "dark:hover:border-gray-700",
        "dark:hover:bg-gray-800"
    )
}

pub(super) fn add_row_container_class(depth: usize) -> Classes {
    classes!(
        "flex",
        "flex-wrap",
        "items-center",
        "gap-2",
        "px-3",
        "py-2",
        if depth == 0 {
            "bg-white dark:bg-gray-900"
        } else {
            "bg-transparent"
        }
    )
}

pub(super) fn row_action_button_class() -> Classes {
    classes!(
        "inline-flex",
        "items-center",
        "justify-center",
        "shadow-none"
    )
}

#[derive(Properties, PartialEq, Clone)]
pub(super) struct JsonCompositeDisclosureProps {
    pub node: JsonModel,
    pub label: AttrValue,
    pub panel_id: String,
    pub path: Vec<JsonPathSegment>,
    pub config: JsonInputConfig,
    pub update_model: Callback<ModelUpdater>,
}

#[component(JsonCompositeDisclosure)]
pub(super) fn json_composite_disclosure(props: &JsonCompositeDisclosureProps) -> Html {
    let is_expanded = props.node.expanded;
    let action_label = if is_expanded {
        format!("Collapse {}", props.label.as_str())
    } else {
        format!("Expand {}", props.label.as_str())
    };
    let on_toggle = {
        let update_model = props.update_model.clone();
        let path = props.path.clone();

        Callback::from(move |_| {
            let path = path.clone();
            update_model.emit(Box::new(move |root| {
                if let Some(node) = node_mut_at_path(root, &path) {
                    node.expanded = !node.expanded;
                }
            }));
        })
    };

    html! {
        <div class={classes!(json_row_control_wrapper_class(props.config.density), "min-w-0", "gap-2")}>
            <button
                type="button"
                class={disclosure_button_class(&props.config)}
                aria-label={action_label}
                aria-expanded={AttrValue::from(is_expanded.to_string())}
                aria-controls={AttrValue::from(props.panel_id.clone())}
                onclick={on_toggle}
            >
                <ArrowDownIcon
                    size={14}
                    decorative={true}
                    class={classes!(
                        "transition-transform",
                        if is_expanded { "rotate-0" } else { "-rotate-90" }
                    )}
                />
            </button>
            <span class={summary_badge_class(&props.config)}>
                { summarize_value(&props.node) }
            </span>
        </div>
    }
}

pub(super) fn render_type_select(
    node: &JsonModel,
    path: &[JsonPathSegment],
    policy_path: &JsonInputPath,
    config: &JsonInputConfig,
    update_model: Callback<ModelUpdater>,
    label: &str,
) -> Html {
    let policy = resolve_policy(config, policy_path);
    if policy.allowed_types.len() <= 1 && policy.type_editable {
        return Html::default();
    }

    let current_type = value_type_for_node(node);
    if !policy.type_editable || policy.allowed_types.len() <= 1 {
        return html! {
            <div class={json_row_control_wrapper_class(config.density)}>
                <span class={type_badge_class(config)}>
                    { current_type.label() }
                </span>
            </div>
        };
    }

    let allowed_types = allowed_types_for_path(config, policy_path);
    let on_change = {
        let update_model = update_model.clone();
        let path = path.to_vec();

        Callback::from(move |value: String| {
            if let Some(next_type) = JsonValueType::from_str(&value) {
                let path = path.clone();
                update_model.emit(Box::new(move |root| {
                    if let Some(node) = node_mut_at_path(root, &path) {
                        convert_node_type(node, next_type);
                    }
                }));
            }
        })
    };

    html! {
        <Select
            id={format!("json-type-{}", node.id)}
            label={label.to_owned()}
            value={Some(AttrValue::from(current_type.as_str()))}
            options={allowed_types
                .into_iter()
                .map(|value_type| SelectOption {
                    label: value_type.label().to_owned(),
                    value: value_type.as_str().to_owned(),
                })
                .collect::<Vec<_>>()}
            on_change={Some(on_change)}
            disabled={config.disable_values}
            required={true}
            size={select_size_for_density(config.density)}
            visually_hidden_label={true}
            container_class="mb-0"
            class={type_select_class(config)}
            aria_label={Some(AttrValue::from(label.to_owned()))}
        />
    }
}
