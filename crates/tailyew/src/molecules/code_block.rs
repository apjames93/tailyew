use crate::{ButtonType, CopyIcon, CopyToClipboard, FormBuilder, FormBuilderConfig};
use yew::prelude::*;
use yew::virtual_dom::VNode;

#[derive(Properties, PartialEq, Clone)]
pub struct CodeBlockProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub language: Option<String>,
    #[prop_or_default]
    pub onsubmit: Option<Callback<SubmitEvent>>,

    /// Whether to show the "Copy" button
    #[prop_or(true)]
    pub show_copy: bool,
}

#[function_component(CodeBlock)]
pub fn code_block(props: &CodeBlockProps) -> Html {
    let CodeBlockProps {
        children,
        class,
        language,
        onsubmit,
        show_copy,
    } = props;

    // Check for embedded form config
    if let Some(lang) = language {
        if lang == "form" {
            if let Some(VNode::VText(vtext)) = children.iter().next() {
                let json = vtext.text.to_string();
                if let Ok(config) = serde_json::from_str::<FormBuilderConfig>(&json) {
                    return html! {
                        <FormBuilder
                            config={config}
                            onsubmit={onsubmit.clone().unwrap_or_else(|| Callback::from(|_| {}))}
                        />
                    };
                } else {
                    return html! {
                        <pre class={classes!(
                            "bg-red-100", "p-4", "rounded", "overflow-auto",
                            "text-sm", "font-mono", "text-red-600",
                            class.clone()
                        )}>
                            {"Invalid Form JSON"}
                        </pre>
                    };
                }
            }
        }
    }

    // Extract raw text to copy
    let text_to_copy = children
        .iter()
        .filter_map(|c| match c {
            VNode::VText(t) => Some(t.text.to_string()),
            _ => None,
        })
        .collect::<Vec<String>>()
        .join("");

    // Show copy button in top-right
    html! {
        <div class="relative">
            { if *show_copy {
                html! {
                    <div class="absolute top-2 right-2 z-10">
                        <CopyToClipboard
                            value={text_to_copy.clone()}
                            button_type={ButtonType::Ghost}
                            copied_button_type={ButtonType::Primary}
                            class="p-1 hover:bg-gray-200 dark:hover:bg-gray-700 rounded"
                        >
                            <CopyIcon />
                        </CopyToClipboard>
                    </div>
                }
            } else {
                html! {}
            }}

            <pre class={classes!(
                "bg-gray-100", "dark:bg-gray-800",
                "p-4", "rounded", "overflow-auto",
                "text-sm", "font-mono",
                "text-gray-800", "dark:text-gray-100",
                class.clone()
            )}>
                <code>
                    { for children.iter() }
                </code>
            </pre>
        </div>
    }
}
