use super::values::{checkbox_checked_from_form, form_from_submit_event, input_value_from_form};
use serde_json::{Map, Value};
use yew::AttrValue;
use yew::events::SubmitEvent;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormFieldValueKind {
    String,
    Number,
    Boolean,
    Json,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormFieldSpec {
    pub name: AttrValue,
    pub kind: FormFieldValueKind,
}

impl FormFieldSpec {
    pub fn string(name: impl Into<AttrValue>) -> Self {
        Self {
            name: name.into(),
            kind: FormFieldValueKind::String,
        }
    }

    pub fn number(name: impl Into<AttrValue>) -> Self {
        Self {
            name: name.into(),
            kind: FormFieldValueKind::Number,
        }
    }

    pub fn boolean(name: impl Into<AttrValue>) -> Self {
        Self {
            name: name.into(),
            kind: FormFieldValueKind::Boolean,
        }
    }

    pub fn json(name: impl Into<AttrValue>) -> Self {
        Self {
            name: name.into(),
            kind: FormFieldValueKind::Json,
        }
    }
}

pub fn e_form_json_object(e: &SubmitEvent, fields: &[FormFieldSpec]) -> Result<Value, String> {
    let form = form_from_submit_event(e)?;

    form_json_object_from_values(
        fields,
        |name| input_value_from_form(&form, name),
        |name| checkbox_checked_from_form(&form, name),
    )
}

pub(super) fn form_json_object_from_values<T, B>(
    fields: &[FormFieldSpec],
    mut text_value: T,
    mut bool_value: B,
) -> Result<Value, String>
where
    T: FnMut(&str) -> Result<String, String>,
    B: FnMut(&str) -> Result<bool, String>,
{
    let mut object = Map::new();

    for field in fields {
        let name = field.name.as_str();
        let value = match field.kind {
            FormFieldValueKind::String => Value::String(text_value(name)?),
            FormFieldValueKind::Number => parse_form_number(name, &text_value(name)?)?,
            FormFieldValueKind::Boolean => Value::Bool(bool_value(name)?),
            FormFieldValueKind::Json => parse_form_json(name, &text_value(name)?)?,
        };
        object.insert(name.to_owned(), value);
    }

    Ok(Value::Object(object))
}

fn parse_form_number(name: &str, raw: &str) -> Result<Value, String> {
    match serde_json::from_str::<Value>(raw.trim()) {
        Ok(Value::Number(number)) => Ok(Value::Number(number)),
        _ => Err(format!("Field '{name}' must be a valid JSON number.")),
    }
}

fn parse_form_json(name: &str, raw: &str) -> Result<Value, String> {
    serde_json::from_str::<Value>(raw)
        .map_err(|err| format!("Field '{name}' must contain valid JSON: {err}"))
}
