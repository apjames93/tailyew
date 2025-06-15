use crate::RenderFieldProps;
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlFormElement, HtmlInputElement};
use yew::events::SubmitEvent;

pub fn e_input_value(id: &str, e: &SubmitEvent) -> String {
    let target: EventTarget = e.target().expect("Event should have a target.");
    let form: HtmlFormElement = target.unchecked_into();
    if let Some(input) = form.get_with_name(id) {
        let input: HtmlInputElement = input.unchecked_into();
        input.value()
    } else {
        web_sys::console::error_1(
            &format!("Input element with name '{}' not found in form.", id).into(),
        );
        String::new()
    }
}

pub fn e_checkbox_checked(id: &str, e: &SubmitEvent) -> bool {
    let target = e.target().expect("Event should have a target.");
    let form: HtmlFormElement = target.unchecked_into();
    if let Some(input) = form.get_with_name(id) {
        let input: HtmlInputElement = input.unchecked_into();
        input.checked()
    } else {
        false
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
    let _form: HtmlFormElement = e
        .target()
        .expect("submit event should have a target")
        .unchecked_into();

    let mut values = HashMap::new();

    for field in fields {
        // pick out the field’s id/name
        let name: String = if let Some(chk) = &field.checkbox {
            chk.id.to_string()
        } else if let Some(input) = &field.input {
            input.id.to_string()
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
