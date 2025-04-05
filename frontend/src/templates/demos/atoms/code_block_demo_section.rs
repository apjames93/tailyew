// frontend/src/templates/demos/code_block_demo_section.rs

use crate::templates::demos::DemoComponent;
use tailyew::atoms::CodeBlock;
use tailyew::organisms::table::Column;
use yew::prelude::*;

#[function_component(CodeBlockDemoSection)]
pub fn code_block_demo_section() -> Html {
    let example = html! {
        <CodeBlock>
            { r#"<Button button_type={ButtonType::Primary}>{ "Click Me" }</Button>"# }
        </CodeBlock>
    };

    let usage_code = r#"
        <CodeBlock>
            { <Button button_type={ButtonType::Primary}>{ "Click Me" }</Button># }
        </CodeBlock>    
"#;

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec!["children".into(), "class".into()],
        },
        Column {
            header: "Type".into(),
            values: vec!["Children".into(), "Classes".into()],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "The content to display inside the code block.".into(),
                "Optional Tailwind class overrides.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            title="CodeBlock Component"
            description={Some(html! {
                <p>{ "The `CodeBlock` component renders a pre-styled <pre><code> block using Tailwind. It’s great for documentation and usage examples." }</p>
            })}
            example={example}
            usage_code={usage_code}
            props_table={Some(props_table)}
        />
    }
}
