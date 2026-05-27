use super::types::{JsonInputPath, JsonInputPathPolicy, JsonValueType};
use serde_json::Value;
use yew::AttrValue;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct JsonInputPreset {
    pub initial_value: Option<Value>,
    pub allowed_types: Option<Vec<JsonValueType>>,
    pub default_new_type: JsonValueType,
    pub path_policies: Option<Vec<JsonInputPathPolicy>>,
    pub placeholder_key: Option<AttrValue>,
    pub placeholder_value: Option<AttrValue>,
}

impl JsonInputPreset {
    pub(crate) fn string_map() -> Self {
        Self::scalar_map(vec![JsonValueType::String], JsonValueType::String)
    }

    pub(crate) fn scalar_map(types: Vec<JsonValueType>, default_type: JsonValueType) -> Self {
        let allowed_types = non_empty_types(types, vec![JsonValueType::String]);
        let default_new_type = default_type_for_allowed(&allowed_types, default_type);

        Self {
            initial_value: None,
            allowed_types: Some(allowed_types),
            default_new_type,
            path_policies: None,
            placeholder_key: None,
            placeholder_value: None,
        }
    }

    pub(crate) fn root_array(item_type: JsonValueType) -> Self {
        Self::root_array_with_types(vec![item_type], item_type)
    }

    pub(crate) fn root_array_with_types(
        types: Vec<JsonValueType>,
        default_type: JsonValueType,
    ) -> Self {
        let allowed_types = non_empty_types(types, vec![JsonValueType::String]);
        let default_new_type = default_type_for_allowed(&allowed_types, default_type);
        let item_type_editable = allowed_types.len() > 1;

        Self {
            initial_value: None,
            allowed_types: Some(allowed_types.clone()),
            default_new_type,
            path_policies: Some(vec![
                JsonInputPathPolicy::for_path(JsonInputPath::root())
                    .allowed_types(vec![JsonValueType::Array])
                    .default_new_type(JsonValueType::Array)
                    .type_editable(false),
                JsonInputPathPolicy::for_path(JsonInputPath::root().any_index())
                    .allowed_types(allowed_types)
                    .default_new_type(default_new_type)
                    .type_editable(item_type_editable),
            ]),
            placeholder_key: None,
            placeholder_value: None,
        }
    }

    pub(crate) fn with_initial_value(mut self, value: Value) -> Self {
        self.initial_value = Some(value);
        self
    }

    pub(crate) fn with_placeholders(
        mut self,
        key: Option<AttrValue>,
        value: Option<AttrValue>,
    ) -> Self {
        self.placeholder_key = key;
        self.placeholder_value = value;
        self
    }

    pub(crate) fn with_path_policies(mut self, path_policies: Vec<JsonInputPathPolicy>) -> Self {
        self.path_policies = Some(path_policies);
        self
    }
}

pub(crate) fn default_type_for_allowed(
    allowed_types: &[JsonValueType],
    requested_default: JsonValueType,
) -> JsonValueType {
    if allowed_types.contains(&requested_default) {
        requested_default
    } else {
        allowed_types
            .first()
            .copied()
            .unwrap_or(JsonValueType::String)
    }
}

pub(crate) fn non_empty_types(
    types: Vec<JsonValueType>,
    fallback: Vec<JsonValueType>,
) -> Vec<JsonValueType> {
    if types.is_empty() { fallback } else { types }
}
