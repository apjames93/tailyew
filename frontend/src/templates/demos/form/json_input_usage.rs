let submitted_json = use_state(|| Value::Null);

let onsubmit_callback = {
    let submitted_json = submitted_json.clone();
    Callback::from(move |e: SubmitEvent| {
        e.prevent_default();
        let json_str = e_input_value("json_payload", &e);
        if let Ok(val) = serde_json::from_str(&json_str) {
            submitted_json.set(val);
        }
    })
};

html! {
    <Form onsubmit_callback={onsubmit_callback}>
        <JsonInput id="json_payload" label="Enter JSON" display_buttons={true} />

        <JsonInput
            id="json_payload_flat"
            label="Flat JSON"
            display_buttons={true}
            initial_value={Some(json!({
                "name": "buddy",
                "email": "guy@example.com",
                "role": "admin"
            }))}
        />
    </Form>
}
