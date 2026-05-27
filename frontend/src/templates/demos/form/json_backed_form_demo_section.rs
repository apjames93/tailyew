use crate::templates::demos::DemoComponent;
use crate::templates::demos::form::json_form_demo_helpers::submitted_json_preview;
use serde_json::{Value, json};
use tailyew::form::{
    FieldArray, FieldArrayObjectField, FieldArraySelectOption, Form, FormFieldSpec, Input,
    InputType, JsonBackedValidationSummaryEntry, JsonValueType, KeyValueInput, async_callback,
    e_form_json_object, summary_entries_from_reports, use_json_backed_form_reports_with_sections,
};
use tailyew::organisms::table::Column;
use web_sys::SubmitEvent;
use yew::prelude::*;

#[component(JsonBackedFormDemoSection)]
pub fn json_backed_form_demo_section() -> Html {
    let submitted_profile = use_state(|| Value::Null);
    let submitted_section_payload = use_state(|| Value::Null);
    let reports = use_json_backed_form_reports_with_sections(vec![
        ("section_parameters", "schema"),
        ("section_headers", "advanced"),
    ]);

    let onsubmit_callback = async_callback({
        let submitted_profile = submitted_profile.clone();
        move |e: SubmitEvent| {
            let submitted_profile = submitted_profile.clone();
            async move {
                let payload = e_form_json_object(
                    &e,
                    &[
                        FormFieldSpec::string("user_name"),
                        FormFieldSpec::number("age"),
                        FormFieldSpec::json("games_played"),
                    ],
                )?;
                submitted_profile.set(payload);
                Ok(None)
            }
        }
    });
    let sectioned_submit_callback = async_callback({
        let submitted_section_payload = submitted_section_payload.clone();
        move |e: SubmitEvent| {
            let submitted_section_payload = submitted_section_payload.clone();
            async move {
                let payload = e_form_json_object(
                    &e,
                    &[
                        FormFieldSpec::json("section_parameters"),
                        FormFieldSpec::json("section_headers"),
                    ],
                )?;
                submitted_section_payload.set(payload);
                Ok(None)
            }
        }
    });

    let summary_entries = summary_entries_from_reports(&reports.reports());

    let example = html! {
        <div class="space-y-8 text-left">
            <Form onsubmit_callback={onsubmit_callback} button_label={"Submit profile".to_owned()}>
                <div class="grid gap-4 md:grid-cols-2">
                    <Input
                        id="user_name"
                        name="user_name"
                        label="User name"
                        default_value="buddy guy"
                    />
                    <Input
                        id="age"
                        name="age"
                        label="Age"
                        input_type={InputType::Number}
                        default_value="30"
                    />
                </div>

                <FieldArray
                    id="games_played_editor"
                    name="games_played"
                    label="Games played"
                    helper_text={Some("The editor has a DOM id, but submits under the games_played form key.")}
                    initial_value={Some(json!([
                        {
                            "id": 1,
                            "name": "Resident Evil Requiem",
                            "hours_played": "3",
                            "beat": false
                        },
                        {
                            "id": 2,
                            "name": "Elden Ring",
                            "hours_played": "100",
                            "beat": true
                        }
                    ]))}
                    object_fields={Some(vec![
                        FieldArrayObjectField::hidden("id", JsonValueType::Number),
                        FieldArrayObjectField::string("name", "Game")
                            .placeholder("Game name")
                            .required(true),
                        FieldArrayObjectField::string("hours_played", "Hours played")
                            .placeholder("0"),
                        FieldArrayObjectField::boolean("beat", "Beat"),
                    ])}
                    add_label="Add game"
                    show_json_preview={true}
                />
            </Form>

            { submitted_json_preview(&submitted_profile) }

            <div class="rounded-lg border border-gray-200 bg-white p-4 dark:border-gray-700 dark:bg-gray-900">
                <div class="mb-4 flex flex-col gap-3 sm:flex-row sm:items-start sm:justify-between">
                    <div>
                        <h3 class="text-sm font-semibold text-gray-900 dark:text-gray-100">
                            { "Sectioned JSON-backed form" }
                        </h3>
                        <p class="mt-1 text-sm text-gray-600 dark:text-gray-400">
                            { "A shared report handle tracks each JSON-backed field and computes section badges without app-specific validity structs. Fix the highlighted fields and the badges clear from invalid to valid." }
                        </p>
                    </div>
                    <div class="flex flex-wrap gap-2">
                        { section_status_badge("Schema", reports.section_is_valid("schema"), reports.section_issue_count("schema")) }
                        { section_status_badge("Advanced", reports.section_is_valid("advanced"), reports.section_issue_count("advanced")) }
                    </div>
                </div>

                <Form onsubmit_callback={sectioned_submit_callback} button_label={"Submit sections".to_owned()}>
                    <FieldArray
                        id="section_parameters_editor"
                        name="section_parameters"
                        label="Parameters"
                        helper_text={Some("Reports from this field are grouped into the Schema section.")}
                        initial_value={Some(json!([
                            {
                                "name": "",
                                "type": "string",
                                "required": true
                            }
                        ]))}
                        object_fields={Some(vec![
                            FieldArrayObjectField::string("name", "Name")
                                .placeholder("order_id")
                                .required(true),
                            FieldArrayObjectField::select(
                                "type",
                                "Type",
                                vec![
                                    FieldArraySelectOption::same("string"),
                                    FieldArraySelectOption::same("number"),
                                    FieldArraySelectOption::same("boolean"),
                                ],
                            )
                            .required(true),
                            FieldArrayObjectField::boolean("required", "Required"),
                        ])}
                        add_label="Add parameter"
                        on_validation_report_change={reports.on_report("section_parameters")}
                    />

                    <KeyValueInput
                        id="section_headers_editor"
                        name="section_headers"
                        label="Headers"
                        helper_text={Some("Reports from this field are grouped into the Advanced section.")}
                        initial_value={Some(json!({
                            "Authorization": "",
                            "X-Request-ID": "{{$request_id}}",
                        }))}
                        key_placeholder="Header name"
                        value_placeholder="Header value"
                        require_values={true}
                        empty_value_message="Header value is required."
                        on_validation_report_change={reports.on_report("section_headers")}
                    />
                </Form>

                { validation_summary(&summary_entries) }
                { submitted_json_preview(&submitted_section_payload) }
            </div>
        </div>
    };

    let props_table = vec![
        Column {
            header: "Helper".into(),
            values: vec![
                Html::from("FormFieldSpec::string"),
                Html::from("FormFieldSpec::number"),
                Html::from("FormFieldSpec::boolean"),
                Html::from("FormFieldSpec::json"),
                Html::from("e_form_json_object"),
                Html::from("use_json_backed_form_reports_with_sections"),
                Html::from("summary_entries_from_reports"),
            ],
        },
        Column {
            header: "Purpose".into(),
            values: vec![
                Html::from("Reads a text input and inserts a JSON string."),
                Html::from("Parses an input value as a JSON number."),
                Html::from("Reads a checkbox as a JSON boolean."),
                Html::from("Parses a hidden JSON-backed field value."),
                Html::from("Builds one submitted JSON object from named form fields."),
                Html::from("Aggregates JSON-backed validation reports into section status."),
                Html::from("Builds user-facing summary rows from report issues."),
            ],
        },
    ];

    html! {
        <DemoComponent
            github_demo_path="form/json_backed_form_demo_section.rs"
            github_source_path="form/form_helpers/json_object.rs"
            title="JSON-Backed Mixed Form"
            description={Some(html! {
                <div class="space-y-2">
                    <p>{"Use `name` on JSON-backed fields when the DOM editor id differs from the submitted backend key. `e_form_json_object` combines normal inputs and JSON-backed hidden inputs into one JSON object."}</p>
                    <p>{"The FieldArray editor contributes native form validity through an unnamed proxy input, so invalid game rows block the profile submit before the mixed JSON payload is read."}</p>
                    <p>{"Use `use_json_backed_form_reports_with_sections` when accordions, tabs, or grouped forms need section-level issue counts from multiple JSON-backed fields."}</p>
                </div>
            })}
            example={example}
            usage_code={include_str!("json_backed_form_usage.rs")}
            props_table={Some(props_table)}
        />
    }
}

fn section_status_badge(label: &str, is_valid: bool, issue_count: usize) -> Html {
    let (class, copy) = if is_valid {
        (
            "rounded-full bg-emerald-50 px-2.5 py-1 text-xs font-medium text-emerald-700 ring-1 ring-emerald-200 dark:bg-emerald-950 dark:text-emerald-200 dark:ring-emerald-900",
            "valid".to_owned(),
        )
    } else {
        (
            "rounded-full bg-amber-50 px-2.5 py-1 text-xs font-medium text-amber-800 ring-1 ring-amber-200 dark:bg-amber-950 dark:text-amber-200 dark:ring-amber-900",
            format!("{issue_count} issues"),
        )
    };

    html! {
        <span class={class}>
            { format!("{label}: {copy}") }
        </span>
    }
}

fn validation_summary(entries: &[JsonBackedValidationSummaryEntry]) -> Html {
    if entries.is_empty() {
        return html! {
            <p class="mt-4 text-sm text-emerald-700 dark:text-emerald-300">
                { "No JSON-backed validation issues." }
            </p>
        };
    }

    html! {
        <div class="mt-4 rounded-md border border-amber-200 bg-amber-50 p-3 dark:border-amber-900 dark:bg-amber-950">
            <p class="text-sm font-medium text-amber-900 dark:text-amber-100">
                { "JSON-backed issues" }
            </p>
            <ul class="mt-2 space-y-1 text-sm text-amber-800 dark:text-amber-200">
                { for entries.iter().map(|entry| {
                    let location = entry
                        .location
                        .clone()
                        .or_else(|| entry.field_label.clone())
                        .unwrap_or_else(|| entry.field_name.clone());

                    html! {
                        <li>
                            { location }
                            { ": " }
                            { entry.message.clone() }
                        </li>
                    }
                })}
            </ul>
        </div>
    }
}
