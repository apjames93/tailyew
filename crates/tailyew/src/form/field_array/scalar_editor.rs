use super::FieldArrayModeProps;
use super::props::FieldArrayProps;
use super::styles::field_array_scalar_wrapper_class;
use crate::form::form_helpers::json_field_support::{helper_with_warning, normalize_array_initial};
use crate::form::json_input::presets::{
    JsonInputPreset, default_type_for_allowed, non_empty_types,
};
use crate::form::{JsonInput, JsonInputPath, JsonInputPathPolicy, JsonValueType};
use serde_json::Value;
use yew::prelude::*;

#[component(ScalarFieldArray)]
pub(super) fn scalar_field_array(props: &FieldArrayModeProps) -> Html {
    let props = &props.props;
    let (normalized_initial, warning) = normalize_array_initial(props.initial_value.clone());
    let current_value = use_state(|| normalized_initial.clone());
    let current_len = array_len(&current_value);
    let allowed_item_types = resolved_item_types(props);
    let default_item_type = default_type_for_allowed(&allowed_item_types, props.item_type);
    let preset = field_array_preset(
        normalized_initial,
        allowed_item_types,
        default_item_type,
        current_len,
        props.min_items,
        props.max_items,
        props.allow_remove,
    )
    .with_placeholders(None, props.placeholder.clone());

    let on_json_change = {
        let current_value = current_value.clone();
        let parent_on_change = props.on_json_change.clone();
        Callback::from(move |value: Value| {
            current_value.set(value.clone());
            if let Some(parent_on_change) = &parent_on_change {
                parent_on_change.emit(value);
            }
        })
    };

    let editor = html! {
        <JsonInput
            id={props.id.clone()}
            name={props.name.clone()}
            label={props.label.clone()}
            helper_text={helper_with_warning(&props.helper_text, warning)}
            initial_value={preset.initial_value}
            allowed_types={preset.allowed_types}
            default_new_type={preset.default_new_type}
            path_policies={preset.path_policies}
            placeholder_value={preset.placeholder_value}
            add_item_label={props.add_label.clone()}
            display_buttons={true}
            show_json_preview={props.show_json_preview}
            block_form_submit_when_invalid={props.block_form_submit_when_invalid}
            on_json_change={Some(on_json_change)}
            on_validity_change={props.on_validity_change.clone()}
            on_validation_report_change={props.on_validation_report_change.clone()}
        />
    };

    html! {
        <div class={field_array_scalar_wrapper_class()}>
            { editor }
        </div>
    }
}

pub(crate) fn field_array_preset(
    initial_value: Value,
    allowed_item_types: Vec<JsonValueType>,
    default_item_type: JsonValueType,
    current_len: usize,
    min_items: Option<usize>,
    max_items: Option<usize>,
    allow_remove: bool,
) -> JsonInputPreset {
    let allowed_item_types = non_empty_types(allowed_item_types, vec![JsonValueType::String]);
    let default_item_type = default_type_for_allowed(&allowed_item_types, default_item_type);
    let item_type_editable = allowed_item_types.len() > 1;
    let allow_add_children = max_items.is_none_or(|max_items| current_len < max_items);
    let allow_remove_children =
        allow_remove && min_items.is_none_or(|min_items| current_len > min_items);

    let base_preset = if allowed_item_types.len() == 1 && allowed_item_types[0] == default_item_type
    {
        JsonInputPreset::root_array(default_item_type)
    } else {
        JsonInputPreset::root_array_with_types(allowed_item_types.clone(), default_item_type)
    };

    base_preset
        .with_initial_value(initial_value)
        .with_path_policies(vec![
            JsonInputPathPolicy::for_path(JsonInputPath::root())
                .allowed_types(vec![JsonValueType::Array])
                .default_new_type(JsonValueType::Array)
                .type_editable(false)
                .allow_add_children(allow_add_children)
                .allow_remove_children(allow_remove_children),
            JsonInputPathPolicy::for_path(JsonInputPath::root().any_index())
                .allowed_types(allowed_item_types)
                .default_new_type(default_item_type)
                .type_editable(item_type_editable)
                .removable(allow_remove_children),
        ])
}

pub(crate) fn resolved_item_types(props: &FieldArrayProps) -> Vec<JsonValueType> {
    props
        .allowed_item_types
        .clone()
        .filter(|types| !types.is_empty())
        .unwrap_or_else(|| vec![props.item_type])
}

pub(crate) fn array_len(value: &Value) -> usize {
    value.as_array().map_or(0, Vec::len)
}
