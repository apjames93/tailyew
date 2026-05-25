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


fn parse_form_number(name: &str, raw: &str) -> Result<Value, String> {
    match serde_json::from_str::<Value>(raw.trim()) {
        Ok(Value::Number(number)) => Ok(Value::Number(number)),
        _ => Err(format!("Field '{name}' must be a valid JSON number.")),
    }
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
