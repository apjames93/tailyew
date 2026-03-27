use crate::templates::demos::DemoComponent;
use tailyew::organisms::Markdown;
use tailyew::organisms::table::Column;
use tailyew::{async_callback, e_checkbox_checked, e_input_value};
use yew::prelude::*;

const MARKDOWN_DOC: &str = include_str!("./markdown_demo.md");

#[component(MarkdownDemoSection)]
pub fn markdown_demo_section() -> Html {
    let submitted_values = use_state(|| "".to_string());

    let on_form_submit = async_callback({
        let submitted_values = submitted_values.clone();
        move |e: SubmitEvent| {
            let submitted_values = submitted_values.clone();
            async move {
                let email = e_input_value("email", &e);
                let name = e_input_value("name", &e);
                let accept = e_checkbox_checked("accept", &e);

                let output = format!(
                    "Submitted values:\nEmail: {}\nName: {}\nAccepted Terms: {}",
                    email, name, accept
                );
                web_sys::console::log_1(&output.clone().into());
                submitted_values.set(output);

                Ok(Some("Form submitted successfully.".into()))
            }
        }
    });

    let example = html! {
        <>
            <Markdown content={MARKDOWN_DOC} on_form_submit={Some(on_form_submit)} />

            if !submitted_values.is_empty() {
                <div class="mt-6 p-4 border rounded text-sm bg-gray-50 dark:bg-gray-800 dark:text-gray-300">
                    { (*submitted_values).clone() }
                </div>
            }
        </>
    };

    let usage_code = r#"
    const MARKDOWN_DOC: &str = include_str!("./markdown_demo.md");
    
    <Markdown content={MARKDOWN_DOC} on_form_submit={Some(on_form_submit)} />
    "#;

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec!["content".into(), "class".into(), "on_form_submit".into()],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "String".into(),
                "Classes".into(),
                "Option<AsyncCallback<SubmitEvent>>".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "Markdown string to render.".into(),
                "Optional Tailwind classes for the root container.".into(),
                "Optional callback for form submissions found in markdown.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            github_demo_path="organisms/markdown_demo_section.rs"
            github_source_path="organisms/markdown.rs"
            title="Markdown Component"
            description={Some(html! {
              <p>{"The `Markdown` component renders sanitized markdown as styled HTML using `pulldown-cmark`. It supports inline styles, block elements, markdown tables rendered through TailYew `Table`, code formatting, and dynamic TailYew forms."}</p>
            })}
            example={example}
            usage_code={usage_code}
            props_table={Some(props_table)}
        />
    }
}
