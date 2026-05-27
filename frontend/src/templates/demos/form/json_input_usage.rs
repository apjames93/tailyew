let submitted_json = use_state(|| Value::Null);

let onsubmit_callback = async_callback({
    let submitted_json = submitted_json.clone();
    move |e: SubmitEvent| {
        let submitted_json = submitted_json.clone();
        async move {
            let payload = e_form_json_object(
                &e,
                &[
                    FormFieldSpec::json("json_payload"),
                    FormFieldSpec::json("json_payload_flat"),
                    FormFieldSpec::json("json_payload_tags"),
                    FormFieldSpec::json("mock_response"),
                ],
            )?;
            submitted_json.set(payload);
            Ok(None)
        }
    }
});

html! {
    <Form onsubmit_callback={onsubmit_callback}>
        <JsonInput
            id="json_payload_editor"
            name="json_payload"
            label="Request fields"
            helper_text={Some("Build the JSON object that will be submitted with this form.")}
            display_buttons={true}
            show_json_preview={true}
            initial_value={Some(json!({}))}
        />

        <JsonInput
            id="json_payload_flat"
            label="Request headers"
            helper_text={Some("Header names and values are submitted as a JSON object.")}
            placeholder_key={Some("Header name")}
            placeholder_value={Some("Header value")}
            display_buttons={true}
            allowed_types={Some(vec![JsonValueType::String])}
            default_new_type={JsonValueType::String}
            initial_value={Some(json!({
                "Accept": "application/json",
                "Content-Type": "application/json",
                "X-Request-Source": "tailyew-demo",
            }))}
        />

        <JsonInput
            id="json_payload_tags"
            label="Tags"
            helper_text={Some("The tags key and Array type are fixed; users edit string items.")}
            display_buttons={true}
            initial_value={Some(json!({
                "tags": ["beta", "internal"]
            }))}
            path_policies={Some(vec![
                JsonInputPathPolicy::for_key("tags")
                    .key_editable(false)
                    .type_editable(false)
                    .allowed_types(vec![JsonValueType::Array])
                    .removable(false),
                JsonInputPathPolicy::for_key("tags")
                    .any_index()
                    .type_editable(false)
                    .allowed_types(vec![JsonValueType::String])
                    .default_new_type(JsonValueType::String),
            ])}
        />

        <JsonInput
            id="mock_response"
            label="Mock response"
            helper_text={Some("Paste or build the JSON response returned during tests.")}
            allow_raw_json_paste={true}
            paste_label="Paste mock response"
            paste_helper_text={Some("Use any valid JSON object, array, string, number, boolean, or null.")}
            paste_placeholder={r#"{
  "status": "ok",
  "data": {
    "id": "ord_123",
    "total": 42.50
  }
}"#.into()}
            apply_paste_label="Use this JSON"
            show_json_preview={true}
            initial_value={Some(json!({
                "status": "ok",
                "data": {
                    "id": "ord_123",
                    "total": 42.50
                }
            }))}
        />
    </Form>
}
