use crate::templates::demos::DemoComponent;
use crate::templates::demos::form::json_form_demo_helpers::{
    json_submit_handler, submitted_json_preview,
};
use serde_json::Value;
use tailyew::form::{Form, TagsInput};
use tailyew::organisms::table::Column;
use yew::prelude::*;

#[component(TagsInputDemoSection)]
pub fn tags_input_demo_section() -> Html {
    let submitted_release_tags = use_state(|| Value::Null);
    let submitted_capabilities = use_state(|| Value::Null);
    let submitted_max_tags = use_state(|| Value::Null);
    let submitted_duplicate_tags = use_state(|| Value::Null);
    let submitted_suggestion_tags = use_state(|| Value::Null);

    let example = html! {
        <div class="grid gap-6 text-left xl:grid-cols-2">
            <div class="space-y-3">
                <Form
                    onsubmit_callback={json_submit_handler("release_tags", submitted_release_tags.clone())}
                    button_label={"Submit tags".to_owned()}
                >
                    <TagsInput
                        id="demo_release_tags"
                        name="release_tags"
                        label="Release tags"
                        helper_text={Some("Press Enter or comma to add a tag. Remove buttons update the hidden JSON array.")}
                        initial_tags={vec!["beta".to_owned(), "internal".to_owned()]}
                        suggestions={vec![
                            "stable".to_owned(),
                            "beta".to_owned(),
                            "internal".to_owned(),
                            "customer-facing".to_owned(),
                        ]}
                    />
                </Form>
                { submitted_json_preview(&submitted_release_tags) }
            </div>

            <div class="space-y-3">
                <Form
                    onsubmit_callback={json_submit_handler("demo_capabilities", submitted_capabilities.clone())}
                    button_label={"Submit capabilities".to_owned()}
                >
                    <TagsInput
                        id="demo_capabilities"
                        label="Capabilities"
                        helper_text={Some("A compact chip editor for feature flags and capabilities.")}
                        initial_tags={vec![
                            "rag".to_owned(),
                            "tool-calls".to_owned(),
                            "evals".to_owned(),
                        ]}
                        placeholder="Add capability"
                        suggestions={vec![
                            "streaming".to_owned(),
                            "batch".to_owned(),
                            "audit-log".to_owned(),
                        ]}
                    />
                </Form>
                { submitted_json_preview(&submitted_capabilities) }
            </div>

            <div class="space-y-3">
                <Form
                    onsubmit_callback={json_submit_handler("demo_max_tags", submitted_max_tags.clone())}
                    button_label={"Submit limited tags".to_owned()}
                >
                    <TagsInput
                        id="demo_max_tags"
                        label="Limited tags"
                        helper_text={Some("Adding is disabled when the maximum of four tags is reached.")}
                        initial_tags={vec![
                            "stable".to_owned(),
                            "internal".to_owned(),
                            "platform".to_owned(),
                            "managed".to_owned(),
                        ]}
                        max_tags={Some(4)}
                    />
                </Form>
                { submitted_json_preview(&submitted_max_tags) }
            </div>

            <div class="space-y-3">
                <Form
                    onsubmit_callback={json_submit_handler("demo_duplicate_tags", submitted_duplicate_tags.clone())}
                    button_label={"Submit unique tags".to_owned()}
                >
                    <TagsInput
                        id="demo_duplicate_tags"
                        label="Duplicate handling"
                        helper_text={Some("Duplicates are blocked by default; set allow_duplicates to true when repeated values are meaningful.")}
                        initial_tags={vec!["beta".to_owned(), "internal".to_owned()]}
                        suggestions={vec![
                            "beta".to_owned(),
                            "preview".to_owned(),
                            "internal".to_owned(),
                        ]}
                    />
                </Form>
                { submitted_json_preview(&submitted_duplicate_tags) }
            </div>

            <div class="space-y-3">
                <Form
                    onsubmit_callback={json_submit_handler("demo_suggestion_tags", submitted_suggestion_tags.clone())}
                    button_label={"Submit selected tags".to_owned()}
                >
                    <TagsInput
                        id="demo_suggestion_tags"
                        label="Suggestion-only tags"
                        helper_text={Some("Custom tags are disabled here; users can only add tags from the suggestion buttons.")}
                        initial_tags={vec!["rag".to_owned()]}
                        placeholder="Select a suggestion"
                        allow_custom_tags={false}
                        suggestions={vec![
                            "rag".to_owned(),
                            "tool-calls".to_owned(),
                            "evals".to_owned(),
                            "streaming".to_owned(),
                        ]}
                    />
                </Form>
                { submitted_json_preview(&submitted_suggestion_tags) }
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
                "initial_tags",
                "placeholder",
                "allow_duplicates",
                "allow_custom_tags",
                "min_tags",
                "max_tags",
                "suggestions",
                "block_form_submit_when_invalid",
                "on_change",
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
                "Vec<String>",
                "AttrValue",
                "bool",
                "bool",
                "Option<usize>",
                "Option<usize>",
                "Vec<String>",
                "bool",
                "Option<Callback<Vec<String>>>",
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
                "Initial chip values.",
                "Input placeholder.",
                "Allows repeated tag values.",
                "Allows users to add tags that are not in suggestions.",
                "Minimum tags before remove is disabled.",
                "Maximum tags before add is disabled.",
                "Optional quick-add suggestions.",
                "Blocks native form submission while tag count constraints are invalid.",
                "Emits Vec<String> when tags change.",
                "Emits the submitted JSON string array.",
                "Emits simple validity state for tag count constraints.",
                "Emits structured validation issues for section-level summaries.",
            ]
            .into_iter()
            .map(Html::from)
            .collect(),
        },
    ];

    html! {
        <DemoComponent
            github_demo_path="form/tags_input_demo_section.rs"
            github_source_path="form/tags_input/mod.rs"
            title="TagsInput Component"
            description={Some(html! {
                <div class="space-y-2">
                    <p>{"TagsInput is a chip-based string array editor. It avoids JSON table chrome while preserving native form submission through one hidden JSON array input."}</p>
                    <p>{"Min/max tag constraints block native form submission by default without adding any extra named fields to the submitted payload."}</p>
                </div>
            })}
            example={example}
            usage_code={include_str!("tags_input_usage.rs")}
            props_table={Some(props_table)}
        />
    }
}
