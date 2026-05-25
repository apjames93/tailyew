use super::super::controls::{
    JsonRowKind, add_row_action_class, array_grid_columns_class, control_height_class,
    editor_header_class, editor_surface_class, input_size_for_density, json_row_action_cell_class,
    json_row_control_wrapper_class, json_row_error_grid_class, json_row_grid_class,
    nested_panel_class, object_grid_columns_class, row_action_size_for_density, row_wrapper_class,
    select_size_for_density, should_show_editor_header, show_type_column_for_path,
};
use super::super::policy::{allowed_types_for_path, default_new_type_for_path};
use super::super::types::{JsonInputDensity, JsonInputPath, JsonInputPathPolicy, JsonValueType};
use super::test_config;
use crate::{ButtonSize, InputSize, SelectSize};

#[test]
fn string_only_allowed_types_hides_type_selector() {
    let mut config = test_config();
    config.allowed_types = vec![JsonValueType::String];

    assert!(!show_type_column_for_path(
        &config,
        &JsonInputPath::key("header"),
        JsonRowKind::Object
    ));
    assert!(!object_grid_columns_class(false).contains("_8rem_"));
}

#[test]
fn multi_type_mode_shows_type_selector() {
    let config = test_config();

    assert!(show_type_column_for_path(
        &config,
        &JsonInputPath::key("field"),
        JsonRowKind::Object
    ));
    assert!(object_grid_columns_class(true).contains("_7rem_"));
}

#[test]
fn array_item_layout_hides_type_column_in_single_type_mode() {
    assert!(!array_grid_columns_class(false).contains("_8rem_"));
}

#[test]
fn string_locked_array_items_hide_type_column() {
    let mut config = test_config();
    config.path_policies = vec![
        JsonInputPathPolicy::for_key("tags")
            .any_index()
            .type_editable(false)
            .allowed_types(vec![JsonValueType::String]),
    ];

    assert!(!show_type_column_for_path(
        &config,
        &JsonInputPath::key("tags").index(0),
        JsonRowKind::Array
    ));
}

#[test]
fn fixed_object_property_keeps_quiet_type_badge_column() {
    let mut config = test_config();
    config.path_policies = vec![
        JsonInputPathPolicy::for_key("enabled")
            .type_editable(false)
            .allowed_types(vec![JsonValueType::Boolean]),
    ];

    assert!(show_type_column_for_path(
        &config,
        &JsonInputPath::key("enabled"),
        JsonRowKind::Object
    ));
}

#[test]
fn object_header_and_row_share_grid_columns() {
    let typed_columns = object_grid_columns_class(true);
    let string_only_columns = object_grid_columns_class(false);

    assert_eq!(
        typed_columns,
        "md:grid-cols-[minmax(8rem,1fr)_7rem_minmax(9rem,2fr)_3rem]"
    );
    assert_eq!(
        string_only_columns,
        "md:grid-cols-[minmax(8rem,1fr)_minmax(9rem,2fr)_3rem]"
    );
    assert!(
        json_row_grid_class(true, JsonInputDensity::Compact, JsonRowKind::Object)
            .to_string()
            .contains(typed_columns)
    );
    assert!(
        json_row_error_grid_class(false, JsonRowKind::Object)
            .to_string()
            .contains(string_only_columns)
    );
}

#[test]
fn array_header_and_row_share_grid_columns() {
    let typed_columns = array_grid_columns_class(true);
    let single_type_columns = array_grid_columns_class(false);

    assert_eq!(
        typed_columns,
        "md:grid-cols-[3rem_7rem_minmax(9rem,2fr)_3rem]"
    );
    assert_eq!(
        single_type_columns,
        "md:grid-cols-[3rem_minmax(9rem,2fr)_3rem]"
    );
    assert!(
        json_row_grid_class(true, JsonInputDensity::Compact, JsonRowKind::Array)
            .to_string()
            .contains(typed_columns)
    );
    assert!(
        json_row_error_grid_class(false, JsonRowKind::Array)
            .to_string()
            .contains(single_type_columns)
    );
}

#[test]
fn density_controls_return_expected_heights() {
    assert_eq!(control_height_class(JsonInputDensity::Compact), "h-9");
    assert_eq!(control_height_class(JsonInputDensity::Comfortable), "h-10");
    assert_eq!(
        input_size_for_density(JsonInputDensity::Compact),
        InputSize::Small
    );
    assert_eq!(
        select_size_for_density(JsonInputDensity::Compact),
        SelectSize::Small
    );
    assert!(matches!(
        row_action_size_for_density(JsonInputDensity::Compact),
        ButtonSize::IconSmall
    ));
    assert!(matches!(
        row_action_size_for_density(JsonInputDensity::Comfortable),
        ButtonSize::IconMedium
    ));
    assert!(
        json_row_control_wrapper_class(JsonInputDensity::Compact)
            .to_string()
            .contains("h-9")
    );
    assert!(
        json_row_action_cell_class(JsonInputDensity::Comfortable)
            .to_string()
            .contains("h-10")
    );
}

#[test]
fn default_new_type_falls_back_to_allowed_type() {
    let mut config = test_config();
    config.allowed_types = vec![JsonValueType::Boolean];
    config.default_new_type = JsonValueType::Object;

    assert_eq!(
        default_new_type_for_path(&config, &JsonInputPath::key("field")),
        JsonValueType::Boolean
    );
}

#[test]
fn max_depth_removes_composite_types_from_allowed_types() {
    let mut config = test_config();
    config.max_depth = Some(1);

    let allowed = allowed_types_for_path(&config, &JsonInputPath::key("field"));

    assert!(!allowed.contains(&JsonValueType::Object));
    assert!(!allowed.contains(&JsonValueType::Array));
    assert!(allowed.contains(&JsonValueType::String));
}

#[test]
fn depth_zero_editor_surface_is_the_root_table_surface() {
    let class = editor_surface_class(0).to_string();

    assert!(class.contains("rounded-lg"));
    assert!(class.contains("border"));
    assert!(class.contains("bg-white"));
}

#[test]
fn nested_panel_uses_left_rail_without_card_chrome() {
    let depth_one = nested_panel_class(1);
    let deeper = nested_panel_class(3);

    assert!(depth_one.contains("border-l"));
    assert!(depth_one.contains("pl-4"));
    assert!(deeper.contains("border-l"));
    assert!(deeper.contains("pl-3"));
    assert!(!depth_one.contains("rounded"));
    assert!(!depth_one.contains("shadow"));
}

#[test]
fn editor_header_visibility_is_depth_and_row_count_aware() {
    assert!(should_show_editor_header(0, 0));
    assert!(should_show_editor_header(0, 1));
    assert!(!should_show_editor_header(1, 2));
    assert!(should_show_editor_header(1, 3));
}

#[test]
fn nested_header_is_quiet_when_shown() {
    let root_class = editor_header_class(0, object_grid_columns_class(true)).to_string();
    let nested_class = editor_header_class(2, object_grid_columns_class(true)).to_string();

    assert!(root_class.contains("bg-gray-50"));
    assert!(root_class.contains("border-b"));
    assert!(!nested_class.contains("bg-gray-50"));
    assert!(!nested_class.contains("rounded"));
    assert!(nested_class.contains("text-gray-400"));
}

#[test]
fn row_wrapper_uses_dividers_instead_of_boxed_cards() {
    let class = row_wrapper_class(1, false).to_string();

    assert!(class.contains("border-b"));
    assert!(class.contains("hover:bg-gray-50"));
    assert!(!class.contains("rounded-md"));
    assert!(!class.contains("shadow"));
}

#[test]
fn add_row_action_is_low_emphasis() {
    let class = add_row_action_class(1).to_string();

    assert!(class.contains("border-transparent"));
    assert!(class.contains("shadow-none"));
    assert!(class.contains("hover:bg-gray-50"));
}
