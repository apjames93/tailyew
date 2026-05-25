use crate::templates::demos::DemoComponent;
use serde_json::{Value, json};
use tailyew::atoms::{TagType, Typo};
use tailyew::form::*;
use tailyew::organisms::table::Column;
use web_sys::SubmitEvent;
use yew::prelude::*;

fn tags_array_policies() -> Vec<JsonInputPathPolicy> {
    vec![
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
    ]
}

fn fixed_tool_settings_policies() -> Vec<JsonInputPathPolicy> {
    vec![
        JsonInputPathPolicy::for_path(JsonInputPath::root())
            .allow_add_children(false)
            .allow_remove_children(false),
        JsonInputPathPolicy::for_key("enabled")
            .key_editable(false)
            .type_editable(false)
            .allowed_types(vec![JsonValueType::Boolean])
            .removable(false),
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
        JsonInputPathPolicy::for_key("transport")
            .key_editable(false)
            .type_editable(false)
            .allowed_types(vec![JsonValueType::Object])
            .removable(false),
        JsonInputPathPolicy::keys(["transport", "timeout_ms"])
            .key_editable(false)
            .type_editable(false)
            .allowed_types(vec![JsonValueType::Number])
            .removable(false),
        JsonInputPathPolicy::keys(["transport", "retry_count"])
            .key_editable(false)
            .type_editable(false)
            .allowed_types(vec![JsonValueType::Number])
            .removable(false),
        JsonInputPathPolicy::keys(["transport", "headers"])
            .key_editable(false)
            .type_editable(false)
            .allowed_types(vec![JsonValueType::Object])
            .removable(false),
        JsonInputPathPolicy::for_path(JsonInputPath::keys(["transport", "headers"]).any_key())
            .type_editable(false)
            .allowed_types(vec![JsonValueType::String])
            .default_new_type(JsonValueType::String),
        JsonInputPathPolicy::for_key("metadata")
            .key_editable(false)
            .type_editable(false)
            .allowed_types(vec![JsonValueType::Null])
            .removable(false),
    ]
}

fn mixed_shape_policies() -> Vec<JsonInputPathPolicy> {
    vec![
        JsonInputPathPolicy::for_path(JsonInputPath::root())
            .allow_add_children(false)
            .allow_remove_children(false),
        JsonInputPathPolicy::for_key("name")
            .key_editable(false)
            .type_editable(false)
            .allowed_types(vec![JsonValueType::String])
            .removable(false),
        JsonInputPathPolicy::for_key("enabled")
            .key_editable(false)
            .type_editable(false)
            .allowed_types(vec![JsonValueType::Boolean])
            .removable(false),
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
        JsonInputPathPolicy::for_key("metadata")
            .key_editable(false)
            .type_editable(false)
            .allowed_types(vec![JsonValueType::Object])
            .removable(false),
    ]
}

#[component(JsonInputDemoSection)]
pub fn json_input_demo_section() -> Html {
    let submitted_json = use_state(|| Value::Null);
    let submitted_headers = use_state(|| Value::Null);
    let submitted_static_values = use_state(|| Value::Null);
    let submitted_config = use_state(|| Value::Null);
    let submitted_array = use_state(|| Value::Null);
    let submitted_paste = use_state(|| Value::Null);
    let submitted_paste_mock = use_state(|| Value::Null);
    let submitted_paste_array = use_state(|| Value::Null);
    let submitted_validation = use_state(|| Value::Null);
    let submitted_locked = use_state(|| Value::Null);
    let submitted_depth = use_state(|| Value::Null);
    let validation_state = use_state(|| None::<JsonInputValidity>);
    let validation_request_id = use_state(|| None::<u64>);

    // Shared handler generator
    let create_submit_handler = |id: &'static str, state: UseStateHandle<Value>| {
        async_callback({
            let state = state.clone();
            move |e: SubmitEvent| {
                let state = state.clone();
                async move {
                    let json_str = e_input_value(id, &e);
                    match serde_json::from_str(&json_str) {
                        Ok(val) => {
                            state.set(val);
                            Ok(None)
                        }
                        Err(err) => {
                            web_sys::console::error_1(&format!("Invalid JSON: {}", err).into());
                            Err("Invalid JSON".into())
                        }
                    }
                }
            }
        })
    };
    let validation_submit_handler = async_callback({
        let state = submitted_validation.clone();
        let validation_state = validation_state.clone();
        let validation_request_id = validation_request_id.clone();
        move |e: SubmitEvent| {
            let state = state.clone();
            let validation_state = validation_state.clone();
            let validation_request_id = validation_request_id.clone();
            async move {
                let next_request_id = (*validation_request_id).unwrap_or(0) + 1;
                validation_request_id.set(Some(next_request_id));

                if (*validation_state)
                    .as_ref()
                    .is_none_or(|validity| !validity.is_valid)
                {
                    return Err("Fix JSON validation errors before submitting.".into());
                }

                let json_str = e_input_value("json_payload_validation", &e);
                match serde_json::from_str(&json_str) {
                    Ok(val) => {
                        state.set(val);
                        Ok(None)
                    }
                    Err(err) => {
                        web_sys::console::error_1(&format!("Invalid JSON: {}", err).into());
                        Err("Invalid JSON".into())
                    }
                }
            }
        }
    });

    let example = html! {
        <div class="space-y-10">

            <section class="p-6 bg-white dark:bg-gray-800 rounded-lg shadow-lg space-y-4">
                <Typo tag={TagType::H2}>{ "1. Empty Object" }</Typo>
                <Form onsubmit_callback={create_submit_handler("json_payload_empty", submitted_json.clone())}>
                    <JsonInput
                        id="json_payload_empty"
                        label="Metadata"
                        helper_text={Some("Start with an empty object and add properties as needed.")}
                        display_buttons={true}
                        show_json_preview={true}
                        allow_raw_json_paste={true}
                        initial_value={Some(json!({}))}
                    />
                    if *submitted_json != Value::Null {
                        <pre class="text-sm text-gray-700 dark:text-gray-300 mt-4 whitespace-pre-wrap">
                            { serde_json::to_string_pretty(&*submitted_json).unwrap() }
                        </pre>
                    }
                </Form>
            </section>

            <section class="p-6 bg-white dark:bg-gray-800 rounded-lg shadow-lg space-y-4">
                <Typo tag={TagType::H2}>{ "2. Request Headers" }</Typo>
                <Form onsubmit_callback={create_submit_handler("json_payload_headers", submitted_headers.clone())}>
                    <JsonInput
                        id="json_payload_headers"
                        label="Headers"
                        helper_text={Some("String header values are preserved as a JSON object.")}
                        placeholder_key={Some("Header name")}
                        placeholder_value={Some("Header value")}
                        display_buttons={true}
                        require_at_least_one={true}
                        show_json_preview={true}
                        allowed_types={Some(vec![JsonValueType::String])}
                        default_new_type={JsonValueType::String}
                        initial_value={Some(json!({
                            "Accept": "application/json",
                            "Content-Type": "application/json",
                            "X-Request-Source": "tailyew-demo"
                        }))}
                    />

                    if *submitted_headers != Value::Null {
                        <pre class="text-sm text-gray-700 dark:text-gray-300 mt-4 whitespace-pre-wrap">
                            { serde_json::to_string_pretty(&*submitted_headers).unwrap() }
                        </pre>
                    }
                </Form>
            </section>

            <section class="p-6 bg-white dark:bg-gray-800 rounded-lg shadow-lg space-y-4">
                <Typo tag={TagType::H2}>{ "3. Tags Array" }</Typo>
                <Form onsubmit_callback={create_submit_handler("json_payload_static_values", submitted_static_values.clone())}>
                    <JsonInput
                        id="json_payload_static_values"
                        label="Release tags"
                        helper_text={Some("The tags property and array type are fixed; users edit the string items only.")}
                        placeholder_value={Some("Tag")}
                        display_buttons={true}
                        show_json_preview={true}
                        path_policies={Some(tags_array_policies())}
                        initial_value={Some(json!({
                            "tags": ["beta", "internal"]
                        }))}
                    />
                    if *submitted_static_values != Value::Null {
                        <pre class="text-sm text-gray-700 dark:text-gray-300 mt-4 whitespace-pre-wrap">
                            { serde_json::to_string_pretty(&*submitted_static_values).unwrap() }
                        </pre>
                    }
                </Form>
            </section>

            <section class="p-6 bg-white dark:bg-gray-800 rounded-lg shadow-lg space-y-4">
                <Typo tag={TagType::H2}>{ "4. Fixed Tool Settings" }</Typo>
                <Form onsubmit_callback={create_submit_handler("json_payload_config", submitted_config.clone())}>
                    <JsonInput
                        id="json_payload_config"
                        label="Tool configuration"
                        helper_text={Some("Known fields keep fixed names and types while selected nested values remain editable.")}
                        placeholder_key={Some("Config key")}
                        placeholder_value={Some("Config value")}
                        display_buttons={true}
                        show_json_preview={true}
                        path_policies={Some(fixed_tool_settings_policies())}
                        initial_value={Some(json!({
                            "enabled": true,
                            "tags": ["beta", "internal"],
                            "transport": {
                                "timeout_ms": 5000,
                                "retry_count": 2,
                                "headers": {
                                    "Accept": "application/json"
                                }
                            },
                            "metadata": null
                        }))}
                    />
                    if *submitted_config != Value::Null {
                        <pre class="text-sm text-gray-700 dark:text-gray-300 mt-4 whitespace-pre-wrap">
                            { serde_json::to_string_pretty(&*submitted_config).unwrap() }
                        </pre>
                    }
                </Form>
            </section>

            <section class="p-6 bg-white dark:bg-gray-800 rounded-lg shadow-lg space-y-4">
                <Typo tag={TagType::H2}>{ "5. Mixed Constrained + Freeform" }</Typo>
                <Form onsubmit_callback={create_submit_handler("json_payload_array", submitted_array.clone())}>
                    <JsonInput
                        id="json_payload_array"
                        label="Application settings"
                        helper_text={Some("Root fields are fixed, while metadata remains a freeform nested object.")}
                        placeholder_key={Some("Metadata key")}
                        placeholder_value={Some("Metadata value")}
                        display_buttons={true}
                        show_json_preview={true}
                        path_policies={Some(mixed_shape_policies())}
                        initial_value={Some(json!({
                            "name": "compile",
                            "enabled": true,
                            "tags": ["beta", "internal"],
                            "metadata": {
                                "source": "manual"
                            }
                        }))}
                    />
                    if *submitted_array != Value::Null {
                        <pre class="text-sm text-gray-700 dark:text-gray-300 mt-4 whitespace-pre-wrap">
                            { serde_json::to_string_pretty(&*submitted_array).unwrap() }
                        </pre>
                    }
                </Form>
            </section>

            <section class="p-6 bg-white dark:bg-gray-800 rounded-lg shadow-lg space-y-4">
                <Typo tag={TagType::H2}>{ "6. Paste JSON" }</Typo>
                <p class="text-sm text-gray-600 dark:text-gray-400">
                    { "The import panel works with any valid JSON shape. Use the default copy for generic editors, or customize the paste labels and placeholder for domain-specific payloads." }
                </p>
                <div class="grid gap-6 xl:grid-cols-3">
                    <Form onsubmit_callback={create_submit_handler("json_payload_paste", submitted_paste.clone())}>
                        <JsonInput
                            id="json_payload_paste"
                            label="Default JSON import"
                            helper_text={Some("Paste valid JSON to replace the structured editor, or paste invalid JSON to see a parse error.")}
                            display_buttons={true}
                            show_json_preview={true}
                            allow_raw_json_paste={true}
                            initial_value={Some(json!({
                                "source": "manual",
                                "enabled": true
                            }))}
                        />
                        if *submitted_paste != Value::Null {
                            <pre class="text-sm text-gray-700 dark:text-gray-300 mt-4 whitespace-pre-wrap">
                                { serde_json::to_string_pretty(&*submitted_paste).unwrap() }
                            </pre>
                        }
                    </Form>

                    <Form onsubmit_callback={create_submit_handler("mock_response", submitted_paste_mock.clone())}>
                        <JsonInput
                            id="mock_response"
                            label="Mock response"
                            helper_text={Some("Paste or build the JSON response returned during tests.")}
                            display_buttons={true}
                            allow_raw_json_paste={true}
                            paste_label="Paste mock response"
                            paste_helper_text={Some("Use any valid JSON object, array, string, number, boolean, or null.")}
                            paste_placeholder={AttrValue::from(r#"{
  "status": "ok",
  "data": {
    "id": "ord_123",
    "total": 42.50
  }
}"#)}
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
                        if *submitted_paste_mock != Value::Null {
                            <pre class="text-sm text-gray-700 dark:text-gray-300 mt-4 whitespace-pre-wrap">
                                { serde_json::to_string_pretty(&*submitted_paste_mock).unwrap() }
                            </pre>
                        }
                    </Form>

                    <Form onsubmit_callback={create_submit_handler("array_payload", submitted_paste_array.clone())}>
                        <JsonInput
                            id="array_payload"
                            label="Array payload"
                            helper_text={Some("Paste an array response or edit the rows manually.")}
                            display_buttons={true}
                            allow_raw_json_paste={true}
                            paste_label="Paste array payload"
                            paste_placeholder={AttrValue::from(r#"[
  { "id": 1, "name": "Example" }
]"#)}
                            apply_paste_label="Load array"
                            show_json_preview={true}
                            initial_value={Some(json!([
                                { "id": 1, "name": "Example" }
                            ]))}
                        />
                        if *submitted_paste_array != Value::Null {
                            <pre class="text-sm text-gray-700 dark:text-gray-300 mt-4 whitespace-pre-wrap">
                                { serde_json::to_string_pretty(&*submitted_paste_array).unwrap() }
                            </pre>
                        }
                    </Form>
                </div>
            </section>

            <section class="p-6 bg-white dark:bg-gray-800 rounded-lg shadow-lg space-y-4">
                <Typo tag={TagType::H2}>{ "7. Validation States" }</Typo>
                <Form onsubmit_callback={validation_submit_handler}>
                    <JsonInput
                        id="json_payload_validation"
                        label="Required configuration"
                        helper_text={Some("Submit the empty required object, or add duplicate keys and invalid number text to see row-level validation.")}
                        placeholder_key={Some("Property name")}
                        placeholder_value={Some("Property value")}
                        display_buttons={true}
                        require_at_least_one={true}
                        show_json_preview={true}
                        max_depth={Some(3)}
                        validation_request_id={*validation_request_id}
                        on_validity_change={Some({
                            let validation_state = validation_state.clone();
                            Callback::from(move |validity| validation_state.set(Some(validity)))
                        })}
                        initial_value={Some(json!({}))}
                    />
                    if let Some(validity) = &*validation_state {
                        <p class="text-sm text-gray-700 dark:text-gray-300">
                            { if validity.is_valid { "Current draft is valid and can be submitted." } else { "Submit is blocked until the JSON draft is valid." } }
                        </p>
                    }
                    if *submitted_validation != Value::Null {
                        <pre class="text-sm text-gray-700 dark:text-gray-300 mt-4 whitespace-pre-wrap">
                            { serde_json::to_string_pretty(&*submitted_validation).unwrap() }
                        </pre>
                    }
                </Form>
            </section>

            <section class="p-6 bg-white dark:bg-gray-800 rounded-lg shadow-lg space-y-4">
                <Typo tag={TagType::H2}>{ "8. Locked Keys" }</Typo>
                <Form onsubmit_callback={create_submit_handler("json_payload_locked", submitted_locked.clone())}>
                    <JsonInput
                        id="json_payload_locked"
                        label="System-provided fields"
                        helper_text={Some("Keys are locked while values remain editable.")}
                        placeholder_value={Some("Field value")}
                        display_buttons={true}
                        disable_keys={true}
                        show_json_preview={true}
                        allowed_types={Some(vec![JsonValueType::String])}
                        initial_value={Some(json!({
                            "tenant_id": "tnt_001",
                            "environment": "production"
                        }))}
                    />
                    if *submitted_locked != Value::Null {
                        <pre class="text-sm text-gray-700 dark:text-gray-300 mt-4 whitespace-pre-wrap">
                            { serde_json::to_string_pretty(&*submitted_locked).unwrap() }
                        </pre>
                    }
                </Form>
            </section>

            <section class="p-6 bg-white dark:bg-gray-800 rounded-lg shadow-lg space-y-4">
                <Typo tag={TagType::H2}>{ "9. Deep Object + Max Depth" }</Typo>
                <Form onsubmit_callback={create_submit_handler("json_payload_depth", submitted_depth.clone())}>
                    <JsonInput
                        id="json_payload_depth"
                        label="Depth-limited config"
                        helper_text={Some("Three nested levels verify the left-rail hierarchy while object and array add options stop at the configured nesting limit.")}
                        display_buttons={true}
                        show_json_preview={true}
                        max_depth={Some(2)}
                        initial_value={Some(json!({
                            "level_one": {
                                "level_two": {
                                    "level_three_value": "value",
                                    "enabled": true
                                }
                            }
                        }))}
                    />
                    if *submitted_depth != Value::Null {
                        <pre class="text-sm text-gray-700 dark:text-gray-300 mt-4 whitespace-pre-wrap">
                            { serde_json::to_string_pretty(&*submitted_depth).unwrap() }
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
                "name".into(),
                "label".into(),
                "initial_value".into(),
                "on_json_change".into(),
                "display_buttons".into(),
                "require_at_least_one".into(),
                "require_string_values".into(),
                "empty_string_value_message".into(),
                "disable_keys".into(),
                "disable_values".into(),
                "helper_text".into(),
                "placeholder_key".into(),
                "placeholder_value".into(),
                "show_json_preview".into(),
                "allow_raw_json_paste".into(),
                "paste_label".into(),
                "paste_helper_text".into(),
                "paste_placeholder".into(),
                "apply_paste_label".into(),
                "allowed_types".into(),
                "default_new_type".into(),
                "max_depth".into(),
                "on_validity_change".into(),
                "on_validation_report_change".into(),
                "validation_mode".into(),
                "validation_requested".into(),
                "validation_request_id".into(),
                "block_form_submit_when_invalid".into(),
                "density".into(),
                "path_policies".into(),
                "add_property_label".into(),
                "add_item_label".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "AttrValue".into(),
                "Option<AttrValue>".into(),
                "AttrValue".into(),
                "Option<Value>".into(),
                "Option<Callback<Value>>".into(),
                "bool".into(),
                "bool".into(),
                "bool".into(),
                "AttrValue".into(),
                "bool".into(),
                "bool".into(),
                "Option<AttrValue>".into(),
                "Option<AttrValue>".into(),
                "Option<AttrValue>".into(),
                "bool".into(),
                "bool".into(),
                "AttrValue".into(),
                "Option<AttrValue>".into(),
                "AttrValue".into(),
                "AttrValue".into(),
                "Option<Vec<JsonValueType>>".into(),
                "JsonValueType".into(),
                "Option<usize>".into(),
                "Option<Callback<JsonInputValidity>>".into(),
                "Option<Callback<JsonBackedValidationReport>>".into(),
                "JsonValidationMode".into(),
                "bool".into(),
                "Option<u64>".into(),
                "bool".into(),
                "JsonInputDensity".into(),
                "Option<Vec<JsonInputPathPolicy>>".into(),
                "AttrValue".into(),
                "AttrValue".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "DOM/accessibility ID for the editor.".into(),
                "Submitted form key. Defaults to id.".into(),
                "Label above the field.".into(),
                "Initial JSON value if any.".into(),
                "Callback when JSON changes.".into(),
                "Show add/remove buttons.".into(),
                "Require at least one entry.".into(),
                "Require non-empty string values.".into(),
                "Message shown for required blank string values.".into(),
                "Disable key input.".into(),
                "Disable value input.".into(),
                "Optional helper copy shown below the label.".into(),
                "Optional placeholder for key inputs.".into(),
                "Optional placeholder for value inputs.".into(),
                "Show a collapsible pretty JSON preview.".into(),
                "Show a simple paste-JSON import panel.".into(),
                "Button label for opening the paste panel.".into(),
                "Optional helper copy shown above the paste textarea.".into(),
                "Placeholder shown in the paste textarea.".into(),
                "Button label for applying pasted JSON.".into(),
                "Limit the value types users can select.".into(),
                "Value type used for new properties/items.".into(),
                "Optional maximum nesting depth.".into(),
                "Callback emitted when validity changes.".into(),
                "Callback emitted with structured validation issues for section-level summaries."
                    .into(),
                "Controls when validation errors become visible.".into(),
                "Legacy parent-controlled trigger for submit-time validation visibility.".into(),
                "Repeatable parent-controlled validation request id.".into(),
                "Blocks native form submission while the current JSON draft is invalid.".into(),
                "Row density for compact or comfortable JSON editing.".into(),
                "Path-aware constraints for locking keys, types, values, and add/remove behavior."
                    .into(),
                "Label for object add actions.".into(),
                "Label for array add actions.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            github_demo_path="form/json_input_demo_section.rs"
            github_source_path="form/json_input/mod.rs"
            title="JsonInput Component"
            description={Some(html! {
                <div class="space-y-2">
                    <p>{"The `JsonInput` component renders an accessible structured JSON builder with type-preserving values, nested objects and arrays, validation, optional import, and a single hidden form value."}</p>
                    <p>{"Invalid JSON drafts participate in native form validation and block submission by default. `on_validity_change`, `on_validation_report_change`, and `validation_request_id` remain available for custom parent-driven validation UI."}</p>
                    <p>{"Use global props for freeform JSON, or `path_policies` to lock known keys, fixed value types, array item types, add/remove permissions, and editable values at specific paths. `AnyIndex` and `AnyKey` policies cover repeated array items and dynamic object children."}</p>
                    <p>{"JsonInput is the advanced editor. Prefer FieldArray for arrays, TagsInput for string tag lists, KeyValueInput for object maps, and StaticValuesInput for scalar template/static values."}</p>
                    <p>{"UI text props use Yew `AttrValue` for cheap cloning; submitted JSON and user-entered values remain JSON/String based."}</p>
                </div>
            })}
            example={example}
            usage_code={usage_code}
            props_table={Some(props_table)}
        />
    }
}
