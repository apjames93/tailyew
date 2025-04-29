use crate::{
    CodeBlock, FormBuilder, FormBuilderConfig, Image, Li, MarkerType, TagType, Typo, Ul, A,
};
use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Parser, Tag};
use serde_json::from_str;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MarkdownProps {
    pub content: String,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub on_form_submit: Option<Callback<SubmitEvent>>,
}

#[function_component(Markdown)]
pub fn markdown(props: &MarkdownProps) -> Html {
    let parser = Parser::new(&props.content);
    let nodes = markdown_to_yew(parser, props.on_form_submit.clone());

    html! {
        <div class={classes!("text-left", "max-w-none", props.class.clone())}>
            { nodes }
        </div>
    }
}

fn markdown_to_yew(parser: Parser, on_form_submit: Option<Callback<SubmitEvent>>) -> Html {
    let mut html_nodes = Vec::new();
    let mut tags_stack = Vec::new();
    let mut current_form_json: Option<String> = None;
    let mut inside_form_block = false;

    for event in parser {
        match event {
            Event::Start(tag) => match &tag {
                Tag::CodeBlock(CodeBlockKind::Fenced(lang)) if lang.to_lowercase() == "form" => {
                    inside_form_block = true;
                    current_form_json = Some(String::new());
                }
                _ => {
                    tags_stack.push((tag, Vec::<Html>::new()));
                }
            },
            Event::End(tag) => match tag {
                Tag::CodeBlock(CodeBlockKind::Fenced(lang)) if lang.to_lowercase() == "form" => {
                    inside_form_block = false;
                    if let Some(json) = current_form_json.take() {
                        if let Ok(config) = from_str::<FormBuilderConfig>(&json) {
                            html_nodes.push(html! {
                                    <FormBuilder
                                        config={config}
                                        onsubmit={on_form_submit.clone().unwrap_or(Callback::from(|_| {}))}
                                    />
                                });
                        } else {
                            html_nodes.push(html! { <CodeBlock>{json}</CodeBlock> });
                        }
                    }
                }
                _ => {
                    if let Some((open_tag, children)) = tags_stack.pop() {
                        let node = match open_tag {
                            Tag::Paragraph => html! { <Typo tag={TagType::P}>{children}</Typo> },
                            Tag::Heading(level, _, _) => match level {
                                HeadingLevel::H1 => {
                                    html! { <Typo tag={TagType::H1}>{children}</Typo> }
                                }
                                HeadingLevel::H2 => {
                                    html! { <Typo tag={TagType::H2}>{children}</Typo> }
                                }
                                HeadingLevel::H3 => {
                                    html! { <Typo tag={TagType::H3}>{children}</Typo> }
                                }
                                HeadingLevel::H4 => {
                                    html! { <Typo tag={TagType::H4}>{children}</Typo> }
                                }
                                HeadingLevel::H5 => {
                                    html! { <Typo tag={TagType::H5}>{children}</Typo> }
                                }
                                HeadingLevel::H6 => {
                                    html! { <Typo tag={TagType::H6}>{children}</Typo> }
                                }
                            },
                            Tag::BlockQuote => {
                                html! { <Typo tag={TagType::BlockQuote}>{children}</Typo> }
                            }
                            Tag::List(None) => {
                                html! { <Ul marker_type={MarkerType::Disc} class="mb-4 pl-4">{children}</Ul> }
                            }
                            Tag::List(Some(_)) => {
                                html! { <Ul marker_type={MarkerType::Decimal} class="mb-4 pl-4">{children}</Ul> }
                            }
                            Tag::Item => html! { <Li class="mb-1">{children}</Li> },
                            Tag::Emphasis => {
                                html! { <Typo tag={TagType::Emphasis}>{children}</Typo> }
                            }
                            Tag::Strong => html! { <Typo tag={TagType::Strong}>{children}</Typo> },
                            Tag::Link(_, dest, _) => {
                                html! { <A href={dest.to_string()}>{children}</A> }
                            }
                            Tag::Image(_, src, alt) => {
                                html! { <Image src={src.to_string()} alt={alt.to_string()} class="my-4" /> }
                            }
                            Tag::CodeBlock(code_block_kind) => {
                                let language = match code_block_kind {
                                    pulldown_cmark::CodeBlockKind::Fenced(lang) => {
                                        Some(lang.to_string())
                                    }
                                    pulldown_cmark::CodeBlockKind::Indented => None,
                                };

                                html! {
                                    <CodeBlock language={language}>
                                        {children}
                                    </CodeBlock>
                                }
                            }
                            _ => html! { <>{children}</> },
                        };
                        if let Some((_, parent_children)) = tags_stack.last_mut() {
                            parent_children.push(node);
                        } else {
                            html_nodes.push(node);
                        }
                    }
                }
            },
            Event::Text(text) => {
                if inside_form_block {
                    if let Some(json) = &mut current_form_json {
                        json.push_str(&text);
                    }
                } else if let Some((_, children)) = tags_stack.last_mut() {
                    children.push(html! { { text.to_string() } });
                } else {
                    html_nodes.push(html! { { text.to_string() } });
                }
            }
            Event::Code(code) => {
                if let Some((_, children)) = tags_stack.last_mut() {
                    children.push(html! { <code class="font-mono bg-gray-100 px-1 rounded">{ code.as_ref() }</code> });
                } else {
                    html_nodes.push(html! { <code class="font-mono bg-gray-100 px-1 rounded">{ code.as_ref() }</code> });
                }
            }
            Event::SoftBreak | Event::HardBreak => {
                if let Some((_, children)) = tags_stack.last_mut() {
                    children.push(html! { <br /> });
                } else {
                    html_nodes.push(html! { <br /> });
                }
            }
            _ => {}
        }
    }

    html! { for html_nodes }
}
