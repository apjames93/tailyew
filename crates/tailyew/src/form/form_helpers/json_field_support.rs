use serde_json::{Map, Value};
use yew::AttrValue;

pub(crate) fn helper_with_warning(
    helper_text: &Option<AttrValue>,
    warning: Option<&str>,
) -> Option<AttrValue> {
    match (helper_text.as_ref(), warning) {
        (Some(helper), Some(warning)) => {
            Some(AttrValue::from(format!("{} {warning}", helper.as_str())))
        }
        (Some(helper), None) => Some(helper.clone()),
        (None, Some(warning)) => Some(AttrValue::from(warning)),
        (None, None) => None,
    }
}

pub(crate) fn normalize_array_initial(value: Option<Value>) -> (Value, Option<&'static str>) {
    match value {
        Some(Value::Array(items)) => (Value::Array(items), None),
        Some(_) => (
            Value::Array(Vec::new()),
            Some("Initial value was not an array and was normalized to an empty list."),
        ),
        None => (Value::Array(Vec::new()), None),
    }
}

pub(crate) fn normalize_object_initial(value: Option<Value>) -> (Value, Option<&'static str>) {
    match value {
        Some(Value::Object(object)) => (Value::Object(object), None),
        Some(_) => (
            Value::Object(Map::new()),
            Some("Initial value was not an object and was normalized to an empty map."),
        ),
        None => (Value::Object(Map::new()), None),
    }
}
