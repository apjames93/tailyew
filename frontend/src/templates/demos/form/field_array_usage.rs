use serde_json::{Value, json};
use tailyew::form::{
    FieldArray, FieldArrayObjectField, FieldArrayRows, FieldArraySelectOption, FieldArrayText,
    FieldArrayValidator, Form, async_callback, e_input_value,
};
use web_sys::SubmitEvent;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
struct ModelInput {
    name: String,
    input_type: String,
    required: bool,
    description: String,
}

#[component(FieldArrayExample)]
pub fn field_array_example() -> Html {
    let submitted_json = use_state(|| Value::Null);
    let extracted_inputs = use_state(Vec::<ModelInput>::new);

    let onsubmit = async_callback({
        let submitted_json = submitted_json.clone();
        let extracted_inputs = extracted_inputs.clone();
        move |e: SubmitEvent| {
            let submitted_json = submitted_json.clone();
            let extracted_inputs = extracted_inputs.clone();
            async move {
                let raw_json = e_input_value("model_inputs", &e);
                let value: Value = serde_json::from_str(&raw_json)
                    .map_err(|_| "FieldArray submitted invalid JSON".to_owned())?;
                let rows = FieldArrayRows::from_value(&value).map_err(|err| err.to_string())?;
                let inputs = rows
                    .active()
                    .map(|row| {
                        let name = row.required_string("name")?;
                        let input_type = row.required_string_enum(
                            "type",
                            &["string", "number", "integer", "boolean"],
                        )?;
                        let required = row.optional_bool("required")?.unwrap_or(false);
                        let description = row.optional_string("description")?.unwrap_or_default();

                        Ok::<ModelInput, _>(ModelInput {
                            name,
                            input_type,
                            required,
                            description,
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|err| err.to_string())?;

                submitted_json.set(value);
                extracted_inputs.set(inputs);
                Ok(None)
            }
        }
    });
    html! {
        <Form onsubmit_callback={onsubmit} button_label={"Submit model inputs".to_owned()}>
            <FieldArray
                id="model_inputs_editor"
                name="model_inputs"
                label="Model inputs"
                helper_text={Some("Reusable validators enforce naming rules and duplicate checks.")}
                text={FieldArrayText {
                    item_label: "model input".into(),
                    item_label_plural: "model inputs".into(),
                    invalid_status: "Fix the highlighted input fields before submitting.".into(),
                    ..FieldArrayText::default()
                }}
                initial_value={Some(json!([
                    {
                        "name": "order_id",
                        "type": "string",
                        "required": true,
                        "description": "The order identifier to look up."
                    },
                    {
                        "name": "include_history",
                        "type": "boolean",
                        "required": false,
                        "description": "Whether to include historical records."
                    }
                ]))}
                object_fields={Some(vec![
                    FieldArrayObjectField::string("name", "Field name")
                        .placeholder("order_id")
                        .required_trimmed("Enter a field name.")
                        .pattern(
                            r"^[A-Za-z][A-Za-z0-9_]*$",
                            "Start with a letter and use only letters, numbers, or underscores.",
                        ),
                    FieldArrayObjectField::select(
                        "type",
                        "Type",
                        vec![
                            FieldArraySelectOption::same("string"),
                            FieldArraySelectOption::same("number"),
                            FieldArraySelectOption::same("integer"),
                            FieldArraySelectOption::same("boolean"),
                        ],
                    )
                    .required(true)
                    .default_value(json!("string")),
                    FieldArrayObjectField::boolean("required", "Required"),
                    FieldArrayObjectField::string("description", "Description")
                        .placeholder("The order identifier to look up.")
                        .required_trimmed("Enter a description."),
                ])}
                validators={vec![
                    FieldArrayValidator::unique_field_trimmed(
                        "name",
                        "Field names must be unique.",
                    ),
                ]}
                add_label="Add model input"
                show_json_preview={true}
            />

            if *submitted_json != Value::Null {
                <pre>{ serde_json::to_string_pretty(&*submitted_json).unwrap() }</pre>
                <ul>
                    { for extracted_inputs.iter().map(|input| html! {
                        <li>
                            {
                                format!(
                                    "{} ({}) · required: {} · {}",
                                    input.name,
                                    input.input_type,
                                    input.required,
                                    input.description
                                )
                            }
                        </li>
                    }) }
                </ul>
            }
        </Form>
    }
}
