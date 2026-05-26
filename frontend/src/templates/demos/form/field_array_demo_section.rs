use crate::templates::demos::DemoComponent;
use crate::templates::demos::form::json_form_demo_helpers::{
    json_submit_handler, submitted_json_preview,
};
use serde_json::{Value, json};
use tailyew::form::{
    FieldArray, FieldArrayDeleteBehavior, FieldArrayObjectField, FieldArraySelectOption,
    FieldArrayText, FieldArrayValidator, Form, JsonBackedValidationReport, JsonInputValidity,
    JsonValueType,
};
use tailyew::organisms::table::Column;
use yew::prelude::*;

#[component(FieldArrayDemoSection)]
pub fn field_array_demo_section() -> Html {
    let submitted_saved_games = use_state(|| Value::Null);
    let submitted_model_inputs = use_state(|| Value::Null);
    let model_inputs_report = use_state(|| None::<JsonBackedValidationReport>);
    let submitted_scopes = use_state(|| Value::Null);
    let submitted_batch_sizes = use_state(|| Value::Null);
    let submitted_scalar_options = use_state(|| Value::Null);
    let live_scalar_options = use_state(|| Value::Null);
    let scalar_options_validity = use_state(|| None::<JsonInputValidity>);
    let submitted_validation_rows = use_state(|| Value::Null);

    let on_scalar_options_change = {
        let live_scalar_options = live_scalar_options.clone();
        Callback::from(move |value: Value| {
            live_scalar_options.set(value);
        })
    };
    let on_scalar_options_validity = {
        let scalar_options_validity = scalar_options_validity.clone();
        Callback::from(move |validity: JsonInputValidity| {
            scalar_options_validity.set(Some(validity));
        })
    };
    let on_model_inputs_report = {
        let model_inputs_report = model_inputs_report.clone();
        Callback::from(move |report: JsonBackedValidationReport| {
            model_inputs_report.set(Some(report));
        })
    };
    let example = html! {
        <div class="space-y-8 text-left">
            <div class="rounded-lg border border-gray-200 bg-gray-50 p-4 text-sm text-gray-700 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-300">
                <h3 class="text-sm font-semibold text-gray-900 dark:text-gray-100">
                    { "FieldArray configuration choices" }
                </h3>
                <ul class="mt-2 list-disc space-y-1 pl-5">
                    <li>{ "Use object_fields for repeated records with labeled controls, hidden fields, select fields, reusable validators, and custom validation." }</li>
                    <li>{ "Use item_type for simple scalar arrays such as strings or numbers." }</li>
                    <li>{ "Use allowed_item_types when each scalar item may choose from a small set of JSON scalar types." }</li>
                    <li>{ "Use delete_behavior when saved database rows should be marked for removal instead of immediately disappearing from the submitted value." }</li>
                </ul>
            </div>

            <div class="space-y-3">
                { demo_heading(
                    "DB-backed object records",
                    "Each record has a subtle grouped surface. Hidden IDs and unknown backend fields are preserved, and removed saved rows can be reviewed before submit.",
                ) }
                <Form
                    onsubmit_callback={json_submit_handler("games_played", submitted_saved_games.clone())}
                    button_label={"Submit games".to_owned()}
                >
                    <FieldArray
                        id="demo_user_games"
                        name="games_played"
                        label="User games"
                        helper_text={Some("Edit the visible fields. IDs stay hidden and preserved.")}
                        text={FieldArrayText {
                            item_label: "game".into(),
                            item_label_plural: "games".into(),
                            invalid_status: "Fix the highlighted game fields before submitting.".into(),
                            deleted_rows_description: "Removed games will be deleted when you save.".into(),
                            ..FieldArrayText::default()
                        }}
                        initial_value={Some(json!([
                            {
                                "id": 1,
                                "name": "Resident Evil Requiem",
                                "hours_played": "3",
                                "beat": false,
                                "source": "steam"
                            },
                            {
                                "id": 2,
                                "name": "Elden Ring",
                                "hours_played": "100",
                                "beat": true,
                                "source": "psn"
                            }
                        ]))}
                        preserve_unknown_fields={true}
                        object_fields={Some(vec![
                            FieldArrayObjectField::hidden("id", JsonValueType::Number),
                            FieldArrayObjectField::string("name", "Game")
                                .placeholder("Game name")
                                .required(true),
                            FieldArrayObjectField::string("hours_played", "Hours played")
                                .placeholder("0")
                                .default_value(json!("0")),
                            FieldArrayObjectField::boolean("beat", "Beat"),
                        ])}
                        delete_behavior={FieldArrayDeleteBehavior::mark_deleted()}
                        add_label="Add game"
                        show_json_preview={true}
                    />
                </Form>
                { submitted_json_preview(&submitted_saved_games) }
            </div>

            <div class="space-y-3">
                { demo_heading(
                    "Model input records with reusable validation",
                    "Grouped records work with select fields, field validators, and array validators. Select options serialize as JSON strings.",
                ) }
                { section_validation_status((*model_inputs_report).as_ref()) }
                <Form
                    onsubmit_callback={json_submit_handler("model_inputs", submitted_model_inputs.clone())}
                    button_label={"Submit model inputs".to_owned()}
                >
                    <FieldArray
                        id="demo_model_inputs"
                        name="model_inputs"
                        label="Model inputs"
                        helper_text={Some("Try duplicate names or a name that starts with a number to see reusable field errors.")}
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
                                "description": "The order identifier to look up.",
                                "source": "Dropped because preserve_unknown_fields is false."
                            },
                            {
                                "name": "include_history",
                                "type": "boolean",
                                "required": false,
                                "description": "Whether to include historical records."
                            }
                        ]))}
                        preserve_unknown_fields={false}
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
                        on_validation_report_change={Some(on_model_inputs_report.clone())}
                        add_label="Add model input"
                        show_json_preview={true}
                    />
                </Form>
                { submitted_json_preview(&submitted_model_inputs) }
            </div>

            <div class="grid gap-6 xl:grid-cols-2">
                <div class="space-y-3">
                    { demo_heading(
                        "String scalar array",
                        "Use the default string item type for simple lists such as scopes, URLs, and email allow-lists.",
                    ) }
                    <Form
                        onsubmit_callback={json_submit_handler("demo_oauth_scopes", submitted_scopes.clone())}
                        button_label={"Submit scopes".to_owned()}
                    >
                        <FieldArray
                            id="demo_oauth_scopes"
                            label="OAuth scopes"
                            helper_text={Some("Submitted as a JSON array of strings.")}
                            initial_value={Some(json!(["read:users", "write:users"]))}
                            placeholder={Some("read:projects")}
                            add_label="Add scope"
                            show_json_preview={true}
                        />
                    </Form>
                    { submitted_json_preview(&submitted_scopes) }
                </div>

                <div class="space-y-3">
                    { demo_heading(
                        "Number scalar array",
                        "Set item_type to Number when the submitted array must preserve JSON number values.",
                    ) }
                    <Form
                        onsubmit_callback={json_submit_handler("demo_batch_sizes", submitted_batch_sizes.clone())}
                        button_label={"Submit sizes".to_owned()}
                    >
                        <FieldArray
                            id="demo_batch_sizes"
                            label="Batch sizes"
                            helper_text={Some("Scalar arrays use compact item rows. Values submit as JSON numbers.")}
                            initial_value={Some(json!([100, 250, 500]))}
                            item_type={JsonValueType::Number}
                            add_label="Add size"
                            show_json_preview={true}
                        />
                    </Form>
                    { submitted_json_preview(&submitted_batch_sizes) }
                </div>

                <div class="space-y-3">
                    { demo_heading(
                        "Constrained scalar values",
                        "allowed_item_types exposes a small type picker. min_items, max_items, callbacks, and native submit blocking still apply.",
                    ) }
                    <Form
                        onsubmit_callback={json_submit_handler("demo_scalar_options", submitted_scalar_options.clone())}
                        button_label={"Submit values".to_owned()}
                    >
                        <FieldArray
                            id="demo_scalar_options"
                            name="demo_scalar_options"
                            label="Scalar options"
                            helper_text={Some("Use callbacks when a parent needs live JSON or validity state before submit.")}
                            initial_value={Some(json!(["small", 3, true]))}
                            item_type={JsonValueType::String}
                            allowed_item_types={Some(vec![
                                JsonValueType::String,
                                JsonValueType::Number,
                                JsonValueType::Boolean,
                            ])}
                            placeholder={Some("value")}
                            min_items={Some(1)}
                            max_items={Some(5)}
                            block_form_submit_when_invalid={true}
                            add_label="Add value"
                            show_json_preview={true}
                            on_json_change={Some(on_scalar_options_change.clone())}
                            on_validity_change={Some(on_scalar_options_validity.clone())}
                        />
                    </Form>
                    <div class="rounded-md border border-gray-200 bg-gray-50 px-3 py-2 text-xs text-gray-700 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-300">
                        <p class="font-medium">
                            {
                                (*scalar_options_validity)
                                    .as_ref()
                                    .map(|validity| {
                                        if validity.is_valid {
                                            "Live validity: valid"
                                        } else {
                                            "Live validity: invalid"
                                        }
                                    })
                                    .unwrap_or("Live validity: unchanged")
                            }
                        </p>
                        if *live_scalar_options != Value::Null {
                            <pre class="mt-2 overflow-x-auto font-mono">
                                { serde_json::to_string_pretty(&*live_scalar_options).unwrap_or_default() }
                            </pre>
                        }
                    </div>
                    { submitted_json_preview(&submitted_scalar_options) }
                </div>

                <div class="space-y-3">
                    { demo_heading(
                        "Built-in object validation",
                        "Required fields and number fields are part of FieldArray validation and block form submit by default.",
                    ) }
                    <Form
                        onsubmit_callback={json_submit_handler("demo_validation_rows", submitted_validation_rows.clone())}
                        button_label={"Submit rows".to_owned()}
                    >
                        <FieldArray
                        id="demo_validation_rows"
                        label="Validation rows"
                        helper_text={Some("Fix the required game name and numeric score before submitting.")}
                        text={FieldArrayText {
                            item_label: "game".into(),
                            item_label_plural: "games".into(),
                            invalid_status: "Fix the highlighted game fields before submitting.".into(),
                            ..FieldArrayText::default()
                        }}
                        initial_value={Some(json!([
                                {
                                    "id": 10,
                                    "name": "",
                                    "score": "not-a-number",
                                    "beat": false
                                }
                            ]))}
                            object_fields={Some(vec![
                                FieldArrayObjectField::hidden("id", JsonValueType::Number),
                                FieldArrayObjectField::string("name", "Game")
                                    .placeholder("Game name")
                                    .required(true),
                                FieldArrayObjectField::number("score", "Score")
                                    .placeholder("0"),
                                FieldArrayObjectField::boolean("beat", "Beat"),
                            ])}
                            add_label="Add row"
                            show_json_preview={true}
                        />
                    </Form>
                    { submitted_json_preview(&submitted_validation_rows) }
                </div>
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
                "text",
                "initial_value",
                "item_type",
                "allowed_item_types",
                "object_fields",
                "preserve_unknown_fields",
                "delete_behavior",
                "placeholder",
                "add_label",
                "min_items",
                "max_items",
                "allow_remove",
                "show_json_preview",
                "block_form_submit_when_invalid",
                "validators",
                "custom_validate",
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
                "FieldArrayText",
                "Option<Value>",
                "JsonValueType",
                "Option<Vec<JsonValueType>>",
                "Option<Vec<FieldArrayObjectField>>",
                "bool",
                "FieldArrayDeleteBehavior",
                "Option<AttrValue>",
                "AttrValue",
                "Option<usize>",
                "Option<usize>",
                "bool",
                "bool",
                "bool",
                "Vec<FieldArrayValidator>",
                "Option<Callback<FieldArrayValidationContext, Vec<FieldArrayCustomIssue>>>",
                "Option<Callback<Value>>",
                "Option<Callback<JsonInputValidity>>",
                "Option<Callback<JsonBackedValidationReport>>",
            ]
            .into_iter()
            .map(Html::from)
            .collect(),
        },
        Column {
            header: "Use".into(),
            values: vec![
                "DOM/accessibility ID for the FieldArray.",
                "Submitted form key. Defaults to id.",
                "Visible field-group label.",
                "Optional helper copy under the label.",
                "Small copy config for item labels, status text, and deleted-row review copy.",
                "Initial JSON array. Missing or non-array values normalize to an empty list.",
                "Default scalar item type and default type for new scalar rows.",
                "Optional scalar type choices. A single type hides the type picker.",
                "Switches FieldArray into object-record mode with labeled field definitions.",
                "Keeps existing object keys that are not listed in object_fields.",
                "Hard-removes by default; MarkDeleted keeps saved rows with a marker such as _deleted=true.",
                "Placeholder for scalar item values.",
                "Text for the add-row button.",
                "Minimum active rows before remove is disabled or invalid.",
                "Maximum active rows before add is disabled.",
                "Enables or disables row removal.",
                "Shows the raw JSON array preview.",
                "Blocks native form submission while the visible draft is invalid.",
                "Adds reusable array-level validators such as unique field checks.",
                "Adds advanced domain rules that can target the whole array, a row, or a specific field.",
                "Emits the field-local JSON array.",
                "Emits merged FieldArray/JsonInput validity.",
                "Emits structured validation issues for section-level summaries.",
            ]
            .into_iter()
            .map(Html::from)
            .collect(),
        },
    ];

    html! {
        <DemoComponent
            github_demo_path="form/field_array_demo_section.rs"
            github_source_path="form/field_array/mod.rs"
            title="FieldArray Component"
            description={Some(html! {
                <div class="space-y-2">
                    <p>{"FieldArray edits JSON arrays without exposing JsonInput path policies. Use it for scalar lists or repeated object records."}</p>
                    <p>{"Object rows support visible fields, hidden preserved fields, select fields, reusable validators, custom validation, soft deletion, and native form blocking. For chip-style string tags, use TagsInput instead."}</p>
                    <p>{"After submit, use FieldArrayRows to extract active JSON object rows into your own app/domain structs."}</p>
                </div>
            })}
            example={example}
            usage_code={include_str!("field_array_usage.rs")}
            props_table={Some(props_table)}
        />
    }
}

fn demo_heading(title: &'static str, description: &'static str) -> Html {
    html! {
        <div>
            <h3 class="text-sm font-semibold text-gray-900 dark:text-gray-100">
                { title }
            </h3>
            <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
                { description }
            </p>
        </div>
    }
}

fn section_validation_status(report: Option<&JsonBackedValidationReport>) -> Html {
    let Some(report) = report else {
        return html! {};
    };

    if report.is_valid {
        html! {
            <div class="rounded-md border border-emerald-200 bg-emerald-50 px-3 py-2 text-xs font-medium text-emerald-800 dark:border-emerald-900 dark:bg-emerald-950 dark:text-emerald-200">
                { "Section valid" }
            </div>
        }
    } else {
        html! {
            <div class="rounded-md border border-amber-200 bg-amber-50 px-3 py-2 text-xs text-amber-900 dark:border-amber-900 dark:bg-amber-950 dark:text-amber-200">
                <p class="font-medium">
                    { format!("{} issues in this section", report.issues.len()) }
                </p>
                <ul class="mt-1 space-y-1">
                    { for report.issues.iter().take(3).map(|issue| {
                        let prefix = issue
                            .location
                            .as_ref()
                            .map(|location| format!("{}: ", location.as_str()))
                            .unwrap_or_default();

                        html! {
                            <li>{ format!("{prefix}{}", issue.message) }</li>
                        }
                    }) }
                </ul>
            </div>
        }
    }
}
