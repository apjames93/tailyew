use serde_json::{Value, json};
use tailyew::form::{
    FieldArray, FieldArrayObjectField, FieldArraySelectOption, Form, FormFieldSpec, Input,
    InputType, JsonValueType, KeyValueInput, async_callback, e_form_json_object,
    summary_entries_from_reports, use_json_backed_form_reports_with_sections,
};
use web_sys::SubmitEvent;
use yew::prelude::*;

#[component(JsonBackedFormExample)]
pub fn json_backed_form_example() -> Html {
    let submitted_payload = use_state(|| Value::Null);
    let reports = use_json_backed_form_reports_with_sections(vec![
        ("parameters", "schema"),
        ("headers", "advanced"),
    ]);
    let summary_entries = summary_entries_from_reports(&reports.reports());

    let onsubmit_callback = async_callback({
        let submitted_payload = submitted_payload.clone();
        move |e: SubmitEvent| {
            let submitted_payload = submitted_payload.clone();
            async move {
                let payload = e_form_json_object(
                    &e,
                    &[
                        FormFieldSpec::string("user_name"),
                        FormFieldSpec::number("age"),
                        FormFieldSpec::json("games_played"),
                        FormFieldSpec::json("parameters"),
                        FormFieldSpec::json("headers"),
                    ],
                )?;
                submitted_payload.set(payload);
                Ok(None)
            }
        }
    });

    html! {
        <Form onsubmit_callback={onsubmit_callback} button_label={"Submit profile".to_owned()}>
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

            <FieldArray
                id="games_played_editor"
                name="games_played"
                label="Games played"
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
                    FieldArrayObjectField::string("name", "Game"),
                    FieldArrayObjectField::string("hours_played", "Hours played"),
                    FieldArrayObjectField::boolean("beat", "Beat"),
                ])}
                add_label="Add game"
            />

            <FieldArray
                id="parameters_editor"
                name="parameters"
                label="Parameters"
                initial_value={Some(json!([
                    { "name": "", "type": "string", "required": true }
                ]))}
                object_fields={Some(vec![
                    FieldArrayObjectField::string("name", "Name").required(true),
                    FieldArrayObjectField::select(
                        "type",
                        "Type",
                        vec![
                            FieldArraySelectOption::same("string"),
                            FieldArraySelectOption::same("number"),
                            FieldArraySelectOption::same("boolean"),
                        ],
                    ),
                    FieldArrayObjectField::boolean("required", "Required"),
                ])}
                on_validation_report_change={reports.on_report("parameters")}
            />

            <KeyValueInput
                id="headers_editor"
                name="headers"
                label="Headers"
                initial_value={Some(json!({
                    "Authorization": "",
                    "X-Request-ID": "{{$request_id}}",
                }))}
                require_values={true}
                empty_value_message="Header value is required."
                on_validation_report_change={reports.on_report("headers")}
            />

            if !reports.all_valid() {
                <ul>
                    { for summary_entries.iter().map(|entry| html! {
                        <li>{ entry.message.clone() }</li>
                    }) }
                </ul>
            }

            if *submitted_payload != Value::Null {
                <pre>{ serde_json::to_string_pretty(&*submitted_payload).unwrap() }</pre>
            }
        </Form>
    }
}
