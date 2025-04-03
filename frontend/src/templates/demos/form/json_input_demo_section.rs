use crate::templates::demos::DemoComponent;
use serde_json::{json, Value};
use tailyew::atoms::{TagType, Typo};
use tailyew::form::*;
use tailyew::organisms::table::Column;
use web_sys::SubmitEvent;
use yew::prelude::*;

#[function_component(JsonInputDemoSection)]
pub fn json_input_demo_section() -> Html {
    let submitted_json = use_state(|| Value::Null);
    let submitted_json_flat = use_state(|| Value::Null);
    let submitted_json_nested = use_state(|| Value::Null);

    // Shared handler generator
    let create_submit_handler = |id: &'static str, state: UseStateHandle<Value>| {
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let json_str = e_input_value(id, &e);
            match serde_json::from_str(&json_str) {
                Ok(val) => state.set(val),
                Err(err) => web_sys::console::error_1(&format!("Invalid JSON: {}", err).into()),
            }
        })
    };

    let example = html! {
        <div class="space-y-10">

            // 🔹 Example 1: Empty
            <section class="p-6 bg-white dark:bg-gray-800 rounded-lg shadow-lg space-y-4">
                <Typo tag={TagType::H2}>{ "1. Empty JSON Input" }</Typo>
                <Form onsubmit_callback={create_submit_handler("json_payload_empty", submitted_json.clone())}>
                    <JsonInput id="json_payload_empty" label="Empty JSON" display_buttons={true} />
                    if *submitted_json != Value::Null {
                        <pre class="text-sm text-gray-700 dark:text-gray-300 mt-4 whitespace-pre-wrap">
                            { serde_json::to_string_pretty(&*submitted_json).unwrap() }
                        </pre>
                    }
                </Form>
            </section>

            // 🔹 Example 2: Flat
            <section class="p-6 bg-white dark:bg-gray-800 rounded-lg shadow-lg space-y-4">
                <Typo tag={TagType::H2}>{ "2. Pre-filled Flat JSON" }</Typo>
                <Form onsubmit_callback={create_submit_handler("json_payload_flat", submitted_json_flat.clone())}>
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

                    if *submitted_json_flat != Value::Null {
                        <pre class="text-sm text-gray-700 dark:text-gray-300 mt-4 whitespace-pre-wrap">
                            { serde_json::to_string_pretty(&*submitted_json_flat).unwrap() }
                        </pre>
                    }
                </Form>
            </section>

            // 🔹 Example 3: Nested
            <section class="p-6 bg-white dark:bg-gray-800 rounded-lg shadow-lg space-y-4">
                <Typo tag={TagType::H2}>{ "3. Pre-filled Nested JSON" }</Typo>
                <Form onsubmit_callback={create_submit_handler("json_payload_nested", submitted_json_nested.clone())}>
                    <JsonInput
                        id="json_payload_nested"
                        label="Nested JSON"
                        display_buttons={true}
                        initial_value={Some(json!({
                            "config": {
                                "timeout": 30,
                                "retries": 5,
                                "headers": {
                                    "Authorization": "Bearer token",
                                    "Accept": "application/json"
                                }
                            },
                            "enabled": true
                        }))}
                    />
                    if *submitted_json_nested != Value::Null {
                        <pre class="text-sm text-gray-700 dark:text-gray-300 mt-4 whitespace-pre-wrap">
                            { serde_json::to_string_pretty(&*submitted_json_nested).unwrap() }
                        </pre>
                    }
                </Form>
            </section>

        </div>
    };

    let usage_code = include_str!("json_input_usage.rs");

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "id".into(),
                "label".into(),
                "initial_value".into(),
                "on_json_change".into(),
                "display_buttons".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "String".into(),
                "String".into(),
                "Option<Value>".into(),
                "Option<Callback<Value>>".into(),
                "bool".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "The input's DOM ID and name.".into(),
                "Label above the field.".into(),
                "Initial JSON value if any.".into(),
                "Callback when JSON changes.".into(),
                "Show add/remove buttons.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            title="JsonInput Component"
            description={Some(html! {
                <p>{"WIP: The `JsonInput` component renders editable key-value pairs and nested JSON structures. Below are examples showing empty, flat, and nested inputs."}</p>
            })}
            example={example}
            usage_code={usage_code}
            props_table={Some(props_table)}
        />
    }
}
