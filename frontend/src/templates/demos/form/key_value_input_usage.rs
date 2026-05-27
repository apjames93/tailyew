use serde_json::{Value, json};
use tailyew::form::{Form, FormFieldSpec, KeyValueInput, async_callback, e_form_json_object};
use web_sys::SubmitEvent;
use yew::prelude::*;

#[component(KeyValueInputExample)]
pub fn key_value_input_example() -> Html {
    let submitted_json = use_state(|| Value::Null);

    let onsubmit = async_callback({
        let submitted_json = submitted_json.clone();
        move |e: SubmitEvent| {
            let submitted_json = submitted_json.clone();
            async move {
                let payload = e_form_json_object(
                    &e,
                    &[
                        FormFieldSpec::json("metadata"),
                        FormFieldSpec::json("headers"),
                    ],
                )?;
                submitted_json.set(payload);
                Ok(None)
            }
        }
    });

    html! {
        <Form onsubmit_callback={onsubmit} button_label={"Submit metadata".to_owned()}>
            <KeyValueInput
                id="metadata_editor"
                name="metadata"
                label="Metadata"
                helper_text={Some("A string-only JSON object map.")}
                initial_value={Some(json!({
                    "owner": "platform",
                    "source": "manual",
                }))}
                key_placeholder="Metadata key"
                value_placeholder="Metadata value"
                show_json_preview={true}
            />

            <KeyValueInput
                id="headers_editor"
                name="headers"
                label="Required header values"
                helper_text={Some("Headers need both a name and a non-empty value.")}
                initial_value={Some(json!({
                    "Authorization": "",
                    "X-Request-ID": "{{$request_id}}",
                }))}
                key_placeholder="Header name"
                value_placeholder="Header value"
                require_values={true}
                empty_value_message="Header value is required."
            />

            if *submitted_json != Value::Null {
                <pre>{ serde_json::to_string_pretty(&*submitted_json).unwrap() }</pre>
            }
        </Form>
    }
}
