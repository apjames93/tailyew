use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct CodeBlockProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,
}

#[function_component(CodeBlock)]
pub fn code_block(props: &CodeBlockProps) -> Html {
    let CodeBlockProps { children, class } = props;

    html! {
        <pre class={classes!(
            "bg-gray-100", "dark:bg-gray-800", "p-4", "rounded", "my-4", "overflow-auto", "text-sm", "font-mono", "text-gray-800", "dark:text-gray-100",
            class.clone()
        )}>
            <code>
                { for children.iter() }
            </code>
        </pre>
    }
}
