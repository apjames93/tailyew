mod array_editor_test;
mod controls_test;
mod form_bridge_test;
mod mod_test;
mod model_test;
mod object_editor_test;
mod policy_test;
mod presets_test;
mod validation_report_test;
mod validation_test;
mod value_editor_test;

use super::types::*;
use yew::AttrValue;

fn test_config() -> JsonInputConfig {
    JsonInputConfig {
        display_buttons: true,
        require_at_least_one: false,
        require_string_values: false,
        empty_string_value_message: AttrValue::from("Enter a value."),
        disable_keys: false,
        disable_values: false,
        placeholder_key: AttrValue::from(DEFAULT_KEY_PLACEHOLDER),
        placeholder_value: AttrValue::from(DEFAULT_VALUE_PLACEHOLDER),
        allowed_types: JsonValueType::all(),
        default_new_type: JsonValueType::String,
        max_depth: None,
        density: JsonInputDensity::Compact,
        path_policies: Vec::new(),
        add_property_label: AttrValue::from("Add property"),
        add_item_label: AttrValue::from("Add item"),
    }
}

fn first_property_mut<'a>(model: &'a mut JsonModel, key: &str) -> &'a mut JsonPropertyNode {
    match &mut model.kind {
        JsonNodeKind::Object(properties) => properties
            .iter_mut()
            .find(|property| property.key == key)
            .expect("property exists"),
        _ => panic!("expected object"),
    }
}
