use crate::form::{async_callback, FormBuilder, FormSubmitCallback, RenderFieldProps};
use crate::system::use_themed_classes;
use crate::{ButtonType, CopyIcon, CopyToClipboard};
use serde::Deserialize;
use web_sys::SubmitEvent;
use yew::{prelude::*, virtual_dom::VNode};

#[derive(Properties, PartialEq, Clone)]
pub struct CodeBlockProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub language: Option<String>,
    #[prop_or_default]
    pub onsubmit: Option<FormSubmitCallback>,
    #[prop_or(true)]
    pub show_copy: bool,
}

#[derive(Deserialize)]
struct EmbeddedFormConfig {
    #[serde(default)]
    inputs: Vec<RenderFieldProps>,
    #[serde(default)]
    button_label: Option<String>,
}

#[component(CodeBlock)]
pub fn code_block(props: &CodeBlockProps) -> Html {
    let CodeBlockProps {
        children,
        class,
        language,
        onsubmit,
        show_copy,
    } = props.clone();
    let pre_classes = use_themed_classes(
        "CodeBlock",
        "root",
        classes!(
            "bg-gray-100",
            "dark:bg-gray-800",
            "p-4",
            "rounded",
            "overflow-auto",
            "text-sm",
            "font-mono",
            "text-gray-800",
            "dark:text-gray-100"
        ),
        class.clone(),
    );

    // if the markdown fence says ` ```form `, try to parse the first text node as JSON:
    if language.as_deref() == Some("form") {
        if let Some(VNode::VText(vt)) = children.iter().next() {
            let raw = vt.text.trim();
            match serde_json::from_str::<EmbeddedFormConfig>(raw) {
                Ok(cfg) => {
                    // fallback async onsubmit
                    let submit_cb = onsubmit
                        .clone()
                        .unwrap_or_else(|| async_callback(|_e: SubmitEvent| async { Ok(None) }));

                    return html! {
                        <FormBuilder
                            onsubmit={submit_cb}
                            inputs={cfg.inputs}
                            button_label={cfg.button_label}
                        />
                    };
                }
                Err(err) => {
                    return html! {
                        <pre class="bg-red-100 p-4 rounded text-red-600">
                            { format!("⚠️ could not parse Form JSON: {}", err) }
                        </pre>
                    };
                }
            }
        }
    }

    // fallback to normal code block rendering
    let text_to_copy = children
        .iter()
        .filter_map(|c| {
            if let VNode::VText(t) = c {
                Some(t.text.clone())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join("");

    html! {
        <div class="relative">
            { if show_copy {
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
                html!{}
            }}
            <pre class={pre_classes}>
                <code>{ for children.iter() }</code>
            </pre>
        </div>
    }
}
