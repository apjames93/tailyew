use crate::templates::demos::DemoComponent;
use crate::templates::demos::form::json_form_demo_helpers::{
    json_submit_handler, submitted_json_preview,
};
use serde_json::{Value, json};
use tailyew::form::{Form, JsonValueType, StaticValuesInput};
use tailyew::organisms::table::Column;
use yew::prelude::*;

#[component(StaticValuesInputDemoSection)]
pub fn static_values_input_demo_section() -> Html {
    let submitted_tool_values = use_state(|| Value::Null);
    let submitted_required_values = use_state(|| Value::Null);
    let submitted_rich_values = use_state(|| Value::Null);

    let example = html! {
        <div class="grid gap-6 text-left xl:grid-cols-3">
            <div class="space-y-3">
                <Form
                    onsubmit_callback={json_submit_handler("static_values", submitted_tool_values.clone())}
                    button_label={"Submit static values".to_owned()}
                >
                    <StaticValuesInput
                        id="demo_tool_static_values"
                        name="static_values"
                        label="Tool static values"
                        helper_text={Some("String, number, and boolean values are available by default.")}
                        initial_value={Some(json!({
                            "workspace_id": "wrk_123",
                            "region": "us-east-1",
                            "dry_run": true,
                        }))}
                        show_json_preview={true}
                    />
                </Form>
                { submitted_json_preview(&submitted_tool_values) }
            </div>

            <div class="space-y-3">
                <Form
                    onsubmit_callback={json_submit_handler("demo_required_static_values", submitted_required_values.clone())}
                    button_label={"Submit required values".to_owned()}
                >
                    <StaticValuesInput
                        id="demo_required_static_values"
                        label="Required template values"
                        helper_text={Some("Blank string values are invalid when required values are enabled.")}
                        initial_value={Some(json!({
                            "customer_name": "Acme",
                            "support_email": "",
                        }))}
                        key_placeholder="Variable name"
                        value_placeholder="Value"
                        add_label="Add variable"
                        require_values={true}
                        empty_value_message="Static value is required."
                        show_json_preview={true}
                    />
                </Form>
                { submitted_json_preview(&submitted_required_values) }
            </div>

            <div class="space-y-3">
                <Form
                    onsubmit_callback={json_submit_handler("demo_rich_static_values", submitted_rich_values.clone())}
                    button_label={"Submit rich values".to_owned()}
                >
                    <StaticValuesInput
                        id="demo_rich_static_values"
                        label="Rich static values"
                        helper_text={Some("Static values may be scalars, null, arrays, or nested objects.")}
                        allowed_value_types={Some(vec![
                            JsonValueType::String,
                            JsonValueType::Number,
                            JsonValueType::Boolean,
                            JsonValueType::Null,
                            JsonValueType::Object,
                            JsonValueType::Array,
                        ])}
                        default_value_type={JsonValueType::String}
                        initial_value={Some(json!({
                            "workspace_id": "wrk_123",
                            "retry_count": 3,
                            "enabled": true,
                            "metadata": {
                                "source": "manual"
                            },
                            "tags": ["beta", "internal"],
                            "fallback": null
                        }))}
                        show_json_preview={true}
                    />
                </Form>
                { submitted_json_preview(&submitted_rich_values) }
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
                "key_placeholder",
                "value_placeholder",
                "add_label",
                "allowed_value_types",
                "default_value_type",
                "require_at_least_one",
                "require_values",
                "empty_value_message",
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
                "AttrValue",
                "AttrValue",
                "AttrValue",
                "Option<Vec<JsonValueType>>",
                "JsonValueType",
                "bool",
                "bool",
                "AttrValue",
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
                "Initial JSON object.",
                "Placeholder for variable names.",
                "Placeholder for values.",
                "Add button label.",
                "Allowed value types. Defaults to String, Number, and Boolean.",
                "Value type used for newly added values when it is allowed.",
                "Require one variable through JsonInput validation.",
                "Require non-empty string values.",
                "Inline error message for required blank string values.",
                "Show the delegated JsonInput preview.",
                "Blocks native form submission while the scalar map is invalid.",
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
            github_demo_path="form/static_values_input_demo_section.rs"
            github_source_path="form/static_values_input/mod.rs"
            title="StaticValuesInput Component"
            description={Some(html! {
                <div class="space-y-2">
                    <p>{"StaticValuesInput wraps KeyValueInput for template/static/config value maps. It defaults to scalar literals: String, Number, and Boolean."}</p>
                    <p>{"Use allowed_value_types when static values need richer JSON literals such as Null, Object, or Array. Use JsonInput for completely arbitrary JSON and KeyValueInput for generic object maps."}</p>
                    <p>{"Required and invalid scalar-map drafts block native form submission by default while still submitting one raw JSON object value when valid."}</p>
                </div>
            })}
            example={example}
            usage_code={include_str!("static_values_input_usage.rs")}
            props_table={Some(props_table)}
        />
    }
}
