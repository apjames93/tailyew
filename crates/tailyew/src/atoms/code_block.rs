use crate::{FormBuilder, FormBuilderConfig};
use yew::prelude::*;
use yew::virtual_dom::VNode; // ✨ Import VNode explicitly

#[derive(Properties, PartialEq, Clone)]
pub struct CodeBlockProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub language: Option<String>,
}

#[function_component(CodeBlock)]
pub fn code_block(props: &CodeBlockProps) -> Html {
    let CodeBlockProps {
        children,
        class,
        language,
    } = props;

    if let Some(lang) = language {
        if lang == "form" {
            if let Some(VNode::VText(vtext)) = children.iter().next() {
                let json = vtext.text.to_string();
                if let Ok(config) = serde_json::from_str::<FormBuilderConfig>(&json) {
                    return html! {
                        <FormBuilder
                            config={config}
                            onsubmit={Callback::from(|_| {})}
                        />
                    };
                } else {
                    return html! {
                        <pre class={classes!("bg-red-100", "p-4", "rounded", "overflow-auto", "text-sm", "font-mono", "text-red-600", class.clone())}>
                            {"Invalid Form JSON"}
                        </pre>
                    };
                }
            }
        }
    }

    // Fallback normal code block
    let content = children
        .iter()
        .map(|c| html! { { c.clone() } })
        .collect::<Html>();

    html! {
        <pre class={classes!(
            "bg-gray-100", "dark:bg-gray-800", "p-4", "rounded", "overflow-auto", "text-sm", "font-mono", "text-gray-800", "dark:text-gray-100",
            class.clone()
        )}>
            <code>
                { content }
            </code>
        </pre>
    }
}
