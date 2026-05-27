use crate::templates::demos::DemoComponent;
use crate::templates::demos::form::json_form_demo_helpers::{
    json_submit_handler, submitted_json_preview,
};
use serde_json::{Value, json};
use tailyew::form::{Form, JsonValueType, KeyValueInput};
use tailyew::organisms::table::Column;
use yew::prelude::*;

#[component(KeyValueInputDemoSection)]
pub fn key_value_input_demo_section() -> Html {
    let submitted_metadata = use_state(|| Value::Null);
    let submitted_query_params = use_state(|| Value::Null);
    let submitted_mixed_scalars = use_state(|| Value::Null);
    let submitted_required_map = use_state(|| Value::Null);
    let submitted_headers = use_state(|| Value::Null);

    let example = html! {
        <div class="grid gap-6 text-left xl:grid-cols-2">
            <div class="space-y-3">
                <Form
                    onsubmit_callback={json_submit_handler("metadata", submitted_metadata.clone())}
                    button_label={"Submit metadata".to_owned()}
                >
                    <KeyValueInput
                        id="demo_metadata"
                        name="metadata"
                        label="Metadata"
                        helper_text={Some("A string-only object map with editable keys and values.")}
                        initial_value={Some(json!({
                            "owner": "platform",
                            "source": "manual",
                        }))}
                        key_placeholder="Metadata key"
                        value_placeholder="Metadata value"
                        show_json_preview={true}
                    />
                </Form>
                { submitted_json_preview(&submitted_metadata) }
            </div>

            <div class="space-y-3">
                <Form
                    onsubmit_callback={json_submit_handler("demo_query_params", submitted_query_params.clone())}
                    button_label={"Submit parameters".to_owned()}
                >
                    <KeyValueInput
                        id="demo_query_params"
                        label="Query parameters"
                        helper_text={Some("Query parameters are usually a simple string map.")}
                        initial_value={Some(json!({
                            "limit": "50",
                            "sort": "created_at",
                        }))}
                        key_placeholder="Parameter"
                        value_placeholder="Value"
                        add_label="Add parameter"
                    />
                </Form>
                { submitted_json_preview(&submitted_query_params) }
            </div>

            <div class="space-y-3">
                <Form
                    onsubmit_callback={json_submit_handler("demo_mixed_scalars", submitted_mixed_scalars.clone())}
                    button_label={"Submit scalars".to_owned()}
                >
                    <KeyValueInput
                        id="demo_mixed_scalars"
                        label="Mixed scalar values"
                        helper_text={Some("Allow String, Number, and Boolean values without Object or Array.")}
                        initial_value={Some(json!({
                            "enabled": true,
                            "retry_count": 2,
                            "region": "us-east-1",
                        }))}
                        allowed_value_types={Some(vec![
                            JsonValueType::String,
                            JsonValueType::Number,
                            JsonValueType::Boolean,
                        ])}
                        show_json_preview={true}
                    />
                </Form>
                { submitted_json_preview(&submitted_mixed_scalars) }
            </div>

            <div class="space-y-3">
                <Form
                    onsubmit_callback={json_submit_handler("demo_required_map", submitted_required_map.clone())}
                    button_label={"Submit labels".to_owned()}
                >
                    <KeyValueInput
                        id="demo_required_map"
                        label="Required labels"
                        helper_text={Some("Submit-time validation can require at least one property.")}
                        initial_value={Some(json!({}))}
                        require_at_least_one={true}
                        key_placeholder="Label"
                        value_placeholder="Value"
                    />
                </Form>
                { submitted_json_preview(&submitted_required_map) }
            </div>

            <div class="space-y-3">
                <Form
                    onsubmit_callback={json_submit_handler("demo_required_headers", submitted_headers.clone())}
                    button_label={"Submit headers".to_owned()}
                >
                    <KeyValueInput
                        id="demo_required_headers"
                        label="Required header values"
                        helper_text={Some("Headers need both a name and a non-empty value. Blank values block submit.")}
                        initial_value={Some(json!({
                            "Authorization": "",
                            "X-Request-ID": "{{$request_id}}",
                        }))}
                        key_placeholder="Header name"
                        value_placeholder="Header value"
                        add_label="Add header"
                        require_values={true}
                        empty_value_message="Header value is required."
                        show_json_preview={true}
                    />
                </Form>
                { submitted_json_preview(&submitted_headers) }
            </div>
        </div>
    };

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "id",
                "name",
                "label",
                "helper_text",
                "initial_value",
                "value_type",
                "allowed_value_types",
                "key_placeholder",
                "value_placeholder",
                "add_label",
                "require_at_least_one",
                "require_values",
                "empty_value_message",
                "allow_remove",
                "show_json_preview",
                "block_form_submit_when_invalid",
                "on_json_change",
                "on_validity_change",
                "on_validation_report_change",
            ]
            .into_iter()
            .map(Html::from)
            .collect(),
        },
        Column {
            header: "Type".into(),
            values: vec![
                "AttrValue",
                "Option<AttrValue>",
                "AttrValue",
                "Option<AttrValue>",
                "Option<Value>",
                "JsonValueType",
                "Option<Vec<JsonValueType>>",
                "AttrValue",
                "AttrValue",
                "AttrValue",
                "bool",
                "bool",
                "AttrValue",
                "bool",
                "bool",
                "bool",
                "Option<Callback<Value>>",
                "Option<Callback<JsonInputValidity>>",
                "Option<Callback<JsonBackedValidationReport>>",
            ]
            .into_iter()
            .map(Html::from)
            .collect(),
        },
        Column {
            header: "Description".into(),
            values: vec![
                "DOM/accessibility ID.",
                "Submitted form key. Defaults to id.",
                "Field label.",
                "Optional helper copy.",
                "Initial JSON object; non-objects normalize to an empty map.",
                "Default value type for new properties.",
                "Optional set of value types; one type hides the Type column.",
                "Placeholder for property names.",
                "Placeholder for values.",
                "Add button label.",
                "Require one property through JsonInput validation.",
                "Require non-empty string values. Defaults to false.",
                "Message shown when require_values finds a blank string value.",
                "Controls property removal.",
                "Show the delegated JsonInput preview.",
                "Blocks native form submission while the JSON object draft is invalid.",
                "Emits the submitted JSON object.",
                "Emits delegated JsonInput validity.",
                "Emits structured validation issues for section-level summaries.",
            ]
            .into_iter()
            .map(Html::from)
            .collect(),
        },
    ];

    html! {
        <DemoComponent
            github_demo_path="form/key_value_input_demo_section.rs"
            github_source_path="form/key_value_input/mod.rs"
            title="KeyValueInput Component"
            description={Some(html! {
                <div class="space-y-2">
                    <p>{"KeyValueInput is the small API for JSON object maps. It preserves object submission while keeping common string-map and scalar-map cases free of path policy setup."}</p>
                    <p>{"Invalid map drafts block native form submission by default, so parent forms can read the hidden JSON input without separately wiring validity state."}</p>
                    <p>{"Use `require_values` for maps like headers, env vars, labels, or query params where blank string values should not submit."}</p>
                </div>
            })}
            example={example}
            usage_code={include_str!("key_value_input_usage.rs")}
            props_table={Some(props_table)}
        />
    }
}
