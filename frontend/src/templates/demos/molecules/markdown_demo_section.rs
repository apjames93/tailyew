use crate::templates::demos::DemoComponent;
use tailyew::molecules::Markdown;
use tailyew::organisms::table::Column;
use yew::prelude::*;

const MARKDOWN_DOC: &str = include_str!("./markdown_demo.md");

#[function_component(MarkdownDemoSection)]
pub fn markdown_demo_section() -> Html {
    let example = html! {
        <Markdown content={MARKDOWN_DOC} />
    };

    let usage_code = r#"
    const MARKDOWN_DOC: &str = include_str!("./markdown_demo.md");
    
    <Markdown content={MARKDOWN_DOC} />
    "#;

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec!["content".into(), "class".into()],
        },
        Column {
            header: "Type".into(),
            values: vec!["String".into(), "Classes".into()],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "Markdown string to render.".into(),
                "Optional Tailwind classes for the root container.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            title="Markdown Component"
            description={Some(html! {
              <p>{"The `Markdown` component renders sanitized markdown as styled HTML using `pulldown-cmark`. It supports inline styles, block elements, and code formatting."}</p>
            })}
            example={example}
            usage_code={usage_code}
            props_table={Some(props_table)}
        />
    }
}
