use crate::templates::demos::DemoComponent;
use tailyew::form::FileInput;
use tailyew::organisms::table::Column;
use yew::prelude::*;

const USAGE_CODE: &str = r#"
let file = use_state(|| "".to_string());
let on_change = {
    let file = file.clone();
    Callback::from(move |name: String| file.set(name))
};

html! {
    <FileInput
        id="resume"
        name="resume_file"
        label="Upload Resume"
        initial_file_name=""
        accept=".pdf,.docx"
        helper_text="Accepted formats: PDF or DOCX."
        required={true}
        on_change={Some(on_change)}
    />
}
"#;

#[component(FileInputDemoSection)]
pub fn file_input_demo_section() -> Html {
    let example = html! { <FileInputUsage /> };

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "id".into(),
                "name".into(),
                "label".into(),
                "initial_file_name".into(),
                "accept".into(),
                "helper_text".into(),
                "error".into(),
                "visually_hidden_label".into(),
                "aria_invalid".into(),
                "aria_describedby".into(),
                "required".into(),
                "disabled".into(),
                "class".into(),
                "on_change".into(),
                "on_blur".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "AttrValue".into(),
                "Option<AttrValue>".into(),
                "AttrValue".into(),
                "AttrValue".into(),
                "AttrValue".into(),
                "Option<AttrValue>".into(),
                "Option<AttrValue>".into(),
                "bool".into(),
                "Option<bool>".into(),
                "Option<AttrValue>".into(),
                "bool".into(),
                "bool".into(),
                "Classes".into(),
                "Option<Callback<String>>".into(),
                "Option<Callback<FocusEvent>>".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "DOM/accessibility ID.".into(),
                "Submitted form field name. Defaults to id.".into(),
                "Label text for the file input.".into(),
                "Initial file name to display (if any).".into(),
                "Optional file type filters (e.g., \".pdf,.docx\").".into(),
                "Optional helper copy below the file picker.".into(),
                "External error message shown below the picker.".into(),
                "Hides the label visually while preserving it for screen readers.".into(),
                "Overrides computed aria-invalid state.".into(),
                "Additional aria-describedby IDs.".into(),
                "Marks the file input as required.".into(),
                "Disables the file input.".into(),
                "Additional Tailwind classes.".into(),
                "Callback fired when a file is selected, passing its name.".into(),
                "Called when the file input loses focus.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            github_demo_path="form/file_input_demo_section.rs"
            github_source_path="form/file_input.rs"
            title="FileInput Component"
            description={Some(html! {
                <p>{"The `FileInput` component provides a styled file picker with live filename preview and customizable file type restrictions."}</p>
            })}
            example={example}
            usage_code={USAGE_CODE}
            props_table={Some(props_table)}
        />
    }
}

#[component(FileInputUsage)]
fn file_input_usage() -> Html {
    let file = use_state(|| "".to_string());
    let on_change = {
        let file = file.clone();
        Callback::from(move |name: String| file.set(name))
    };

    html! {
            <FileInput
                id="resume"
                name="resume_file"
            label="Upload Resume"
            initial_file_name=""
            accept={".pdf,.docx"}
            helper_text="Accepted formats: PDF or DOCX."
            required={true}
            on_change={Some(on_change)}
        />
    }
}
