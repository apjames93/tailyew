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
        label="Upload Resume"
        initial_file_name=""
        accept={Some(".pdf,.docx")}
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
                "label".into(),
                "initial_file_name".into(),
                "accept".into(),
                "class".into(),
                "on_change".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "String".into(),
                "String".into(),
                "String".into(),
                "Option<String>".into(),
                "Classes".into(),
                "Option<Callback<String>>".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "Input ID and label 'for' target.".into(),
                "Label text for the file input.".into(),
                "Initial file name to display (if any).".into(),
                "Optional file type filters (e.g., \".pdf,.docx\").".into(),
                "Additional Tailwind classes.".into(),
                "Callback fired when a file is selected, passing its name.".into(),
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
            label="Upload Resume"
            initial_file_name=""
            accept={".pdf,.docx"}
            on_change={on_change}
        />
    }
}
