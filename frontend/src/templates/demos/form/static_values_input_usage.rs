use serde_json::{Value, json};
use tailyew::form::{
    Form, FormFieldSpec, JsonValueType, StaticValuesInput, async_callback, e_form_json_object,
};
use web_sys::SubmitEvent;
use yew::prelude::*;

#[component(StaticValuesInputExample)]
pub fn static_values_input_example() -> Html {
    let submitted_json = use_state(|| Value::Null);

    let onsubmit = async_callback({
        let submitted_json = submitted_json.clone();
        move |e: SubmitEvent| {
            let submitted_json = submitted_json.clone();
            async move {
                let payload = e_form_json_object(
                    &e,
                    &[
                        FormFieldSpec::json("static_values"),
                        FormFieldSpec::json("rich_static_values"),
                    ],
                )?;
                submitted_json.set(payload);
                Ok(None)
            }
        }
    });

    html! {
        <Form onsubmit_callback={onsubmit} button_label={"Submit static values".to_owned()}>
            <StaticValuesInput
                id="static_values_editor"
                name="static_values"
                label="Tool static values"
                helper_text={Some("Values are strings, numbers, or booleans by default. Add allowed_value_types when richer JSON values are needed.")}
                initial_value={Some(json!({
                    "workspace_id": "wrk_123",
                    "region": "us-east-1",
                    "dry_run": true,
                }))}
                show_json_preview={true}
            />

            <StaticValuesInput
                id="rich_static_values"
                label="Rich static values"
                helper_text={Some("Allow null, arrays, and nested objects for config fragments.")}
                allowed_value_types={Some(vec![
                    JsonValueType::String,
                    JsonValueType::Number,
                    JsonValueType::Boolean,
                    JsonValueType::Null,
                    JsonValueType::Object,
                    JsonValueType::Array,
                ])}
                initial_value={Some(json!({
                    "metadata": { "source": "manual" },
                    "tags": ["beta", "internal"],
                    "fallback": null,
                }))}
            />

            if *submitted_json != Value::Null {
                <pre>{ serde_json::to_string_pretty(&*submitted_json).unwrap() }</pre>
            }
        </Form>
    }
}
