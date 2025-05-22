use crate::templates::demos::DemoComponent;
use tailyew::organisms::table::Column;
use tailyew::organisms::Markdown;
use tailyew::{e_checkbox_checked, e_input_value};
use yew::prelude::*;

const MARKDOWN_DOC: &str = include_str!("./markdown_demo.md");

#[function_component(MarkdownDemoSection)]
pub fn markdown_demo_section() -> Html {
    let submitted_values = use_state(|| "".to_string());

    let on_form_submit = {
        let submitted_values = submitted_values.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let email = e_input_value("email", &e);
            let name = e_input_value("name", &e);
            let accept = e_checkbox_checked("accept", &e);

            let output = format!(
                "Submitted values:\nEmail: {}\nName: {}\nAccepted Terms: {}",
                email, name, accept
            );
            web_sys::console::log_1(&output.clone().into());
            submitted_values.set(output);
        })
    };

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
                "Option<Callback<SubmitEvent>>".into(),
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
            title="Markdown Component"
            description={Some(html! {
              <p>{"The `Markdown` component renders sanitized markdown as styled HTML using `pulldown-cmark`. It supports inline styles, block elements, code formatting, and dynamic TailYew forms."}</p>
            })}
            example={example}
            usage_code={usage_code}
            props_table={Some(props_table)}
        />
    }
}
