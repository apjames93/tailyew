use crate::RenderFieldProps;
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{
    Element, EventTarget, HtmlFormElement, HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement,
};
use yew::events::SubmitEvent;
use yew::prelude::*;

pub type FormSubmitCallback =
    Callback<SubmitEvent, Pin<Box<dyn Future<Output = Result<Option<String>, String>>>>>;

pub type FormSubmitFuture = Pin<Box<dyn Future<Output = Result<Option<String>, String>>>>;

pub fn async_callback<F, Fut>(f: F) -> FormSubmitCallback
where
    F: Fn(SubmitEvent) -> Fut + 'static,
    Fut: Future<Output = Result<Option<String>, String>> + 'static,
{
    Callback::from(move |e| {
        let fut = f(e);
        let boxed: Pin<Box<dyn Future<Output = Result<Option<String>, String>>>> = Box::pin(fut);
        boxed
    })
}

pub fn e_input_value(name: &str, e: &SubmitEvent) -> String {
    match form_from_submit_event(e).and_then(|form| input_value_from_form(&form, name)) {
        Ok(value) => value,
        Err(message) => {
            web_sys::console::error_1(&message.into());
            String::new()
        }
    }
}

pub fn e_checkbox_checked(name: &str, e: &SubmitEvent) -> bool {
    match form_from_submit_event(e).and_then(|form| checkbox_checked_from_form(&form, name)) {
        Ok(value) => value,
        Err(message) => {
            web_sys::console::error_1(&message.into());
            false
        }
    }
}

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

fn form_from_submit_event(e: &SubmitEvent) -> Result<HtmlFormElement, String> {
    if let Some(form) = e.current_target().and_then(event_target_as_form) {
        return Ok(form);
    }

    let target = e
        .target()
        .ok_or_else(|| "Submit event did not include a form target.".to_owned())?;

    if let Some(form) = event_target_as_form(target.clone()) {
        return Ok(form);
    }

    let target_element: Element = target
        .dyn_into()
        .map_err(|_| "Submit event target was not a form element.".to_owned())?;

    target_element
        .closest("form")
        .map_err(|_| "Unable to resolve the submitted form element.".to_owned())?
        .ok_or_else(|| "Submit event target was not inside a form element.".to_owned())?
        .dyn_into::<HtmlFormElement>()
        .map_err(|_| "Resolved submit target form was not a form element.".to_owned())
}

fn event_target_as_form(target: EventTarget) -> Option<HtmlFormElement> {
    target.dyn_into::<HtmlFormElement>().ok()
}

fn input_value_from_form(form: &HtmlFormElement, name: &str) -> Result<String, String> {
    let control = form
        .get_with_name(name)
        .ok_or_else(|| format!("Form field '{name}' was not found."))?;

    value_from_form_control(&control)
        .ok_or_else(|| format!("Form field '{name}' is not a supported value control."))
}

fn value_from_form_control(control: &js_sys::Object) -> Option<String> {
    if let Some(input) = control.dyn_ref::<HtmlInputElement>() {
        return Some(input.value());
    }

    if let Some(textarea) = control.dyn_ref::<HtmlTextAreaElement>() {
        return Some(textarea.value());
    }

    if let Some(select) = control.dyn_ref::<HtmlSelectElement>() {
        return Some(select.value());
    }

    // Radio groups are returned by HtmlFormElement::get_with_name as a
    // RadioNodeList. Read its standard value property without requiring a
    // separate web-sys feature gate.
    js_sys::Reflect::get(control.as_ref(), &JsValue::from_str("value"))
        .ok()
        .and_then(|value| value.as_string())
}

fn checkbox_checked_from_form(form: &HtmlFormElement, name: &str) -> Result<bool, String> {
    let input = form
        .get_with_name(name)
        .ok_or_else(|| format!("Form field '{name}' was not found."))?;
    let input: HtmlInputElement = input
        .dyn_into()
        .map_err(|_| format!("Form field '{name}' is not a checkbox input element."))?;
    Ok(input.checked())
}

fn form_json_object_from_values<T, B>(
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

#[derive(Debug, Clone)]
pub enum FormValue {
    Text(String),
    Checked(bool),
}

/// Walks all the `RenderFieldProps` you passed into your `FormBuilder`,
/// and returns a map of `field.id -> FormValue`.
pub fn e_form_builder_values(
    e: &SubmitEvent,
    fields: &[RenderFieldProps],
) -> HashMap<String, FormValue> {
    // just to assert that we really do have a form
    let _form: HtmlFormElement = form_from_submit_event(e)
        .expect("submit event should resolve to the submitted form element");

    let mut values = HashMap::new();

    for field in fields {
        // pick out the field’s id/name
        let name: String = if let Some(chk) = &field.checkbox {
            chk.name
                .clone()
                .unwrap_or_else(|| chk.id.clone())
                .to_string()
        } else if let Some(input) = &field.input {
            input
                .name
                .clone()
                .unwrap_or_else(|| input.id.clone())
                .to_string()
        } else if let Some(textarea) = &field.textarea {
            textarea.id.to_string()
        } else if let Some(select) = &field.select {
            select.id.to_string()
        } else if let Some(radio) = &field.radio {
            radio.id.to_string()
        } else if let Some(color) = &field.color {
            color.id.to_string()
        } else if let Some(file) = &field.file {
            file.id.to_string()
        } else if let Some(phone) = &field.phone {
            phone.id.to_string()
        } else if let Some(range) = &field.range {
            range.id.to_string()
        } else if let Some(state) = &field.state {
            state.id.to_string()
        } else if let Some(search) = &field.search {
            search.id.to_string()
        } else {
            // unknown field type; skip it
            continue;
        };

        // now extract
        let value = if field.checkbox.is_some() {
            FormValue::Checked(e_checkbox_checked(&name, e))
        } else {
            FormValue::Text(e_input_value(&name, e))
        };

        values.insert(name, value);
    }

    values
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_json_object(
        fields: &[FormFieldSpec],
        values: &[(&str, &str)],
        checks: &[(&str, bool)],
    ) -> Result<Value, String> {
        form_json_object_from_values(
            fields,
            |name| {
                values
                    .iter()
                    .find(|(field, _)| *field == name)
                    .map(|(_, value)| (*value).to_owned())
                    .ok_or_else(|| format!("Form field '{name}' was not found."))
            },
            |name| {
                checks
                    .iter()
                    .find(|(field, _)| *field == name)
                    .map(|(_, value)| *value)
                    .ok_or_else(|| format!("Form field '{name}' was not found."))
            },
        )
    }

    #[test]
    fn form_json_object_builds_mixed_payload() {
        let payload = build_json_object(
            &[
                FormFieldSpec::string("user_name"),
                FormFieldSpec::number("age"),
                FormFieldSpec::boolean("active"),
                FormFieldSpec::json("games_played"),
            ],
            &[
                ("user_name", "buddy guy"),
                ("age", "30"),
                (
                    "games_played",
                    r#"[{"id":1,"name":"Resident Evil Requiem"}]"#,
                ),
            ],
            &[("active", true)],
        )
        .unwrap();

        assert_eq!(
            payload,
            serde_json::json!({
                "user_name": "buddy guy",
                "age": 30,
                "active": true,
                "games_played": [
                    { "id": 1, "name": "Resident Evil Requiem" }
                ]
            })
        );
    }

    #[test]
    fn form_json_object_rejects_invalid_number() {
        let err =
            build_json_object(&[FormFieldSpec::number("age")], &[("age", "bad")], &[]).unwrap_err();

        assert_eq!(err, "Field 'age' must be a valid JSON number.");
    }

    #[test]
    fn form_json_object_rejects_invalid_json() {
        let err = build_json_object(&[FormFieldSpec::json("payload")], &[("payload", "{")], &[])
            .unwrap_err();

        assert!(err.starts_with("Field 'payload' must contain valid JSON:"));
    }

    #[test]
    fn form_json_object_reports_missing_field() {
        let err = build_json_object(&[FormFieldSpec::string("missing")], &[], &[]).unwrap_err();

        assert_eq!(err, "Form field 'missing' was not found.");
    }
}
