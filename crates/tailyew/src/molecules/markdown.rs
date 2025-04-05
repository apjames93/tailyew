use pulldown_cmark::{Event, HeadingLevel, Parser, Tag};
use yew::prelude::*;

use crate::{CodeBlock, Image, Li, MarkerType, TagType, Typo, Ul, A};

#[derive(Properties, PartialEq)]
pub struct MarkdownProps {
    pub content: String,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Markdown)]
pub fn markdown(props: &MarkdownProps) -> Html {
    let parser = Parser::new(&props.content);
    let nodes = markdown_to_yew(parser);

    html! {
        <div class={classes!("text-left", "max-w-none", props.class.clone())}>
            { nodes }
        </div>
    }
}

fn markdown_to_yew(parser: Parser) -> Html {
    let mut html_nodes = Vec::new();
    let mut tags_stack = Vec::new();

    for event in parser {
        match event {
            Event::Start(tag) => {
                tags_stack.push((tag, Vec::new()));
            }
            Event::End(_tag) => {
                if let Some((open_tag, children)) = tags_stack.pop() {
                    let node = match open_tag {
                        Tag::Paragraph => html! {
                            <Typo tag={TagType::P}>
                            {children}
                        </Typo>
                         },
                        Tag::Heading(level, _, _) => match level {
                            HeadingLevel::H1 => html! {
                                <Typo tag={TagType::H1}>
                                    {children}
                                </Typo>
                            },
                            HeadingLevel::H2 => html! {
                                <Typo tag={TagType::H2}>
                                    {children}
                                </Typo>
                            },
                            HeadingLevel::H3 => html! {
                                <Typo tag={TagType::H3}>
                                    {children}
                                </Typo>
                            },
                            HeadingLevel::H4 => html! {
                                <Typo tag={TagType::H4}>
                                    {children}
                                </Typo>
                            },
                            HeadingLevel::H5 => html! {
                                <Typo tag={TagType::H5}>
                                    {children}
                                </Typo>
                            },
                            HeadingLevel::H6 => html! {
                                <Typo tag={TagType::H6}>
                                    {children}
                                </Typo>
                            },
                        },
                        Tag::BlockQuote => html! {
                           <Typo tag={TagType::BlockQuote}>
                               {children}
                           </Typo>
                        },
                        Tag::CodeBlock(_) => html! {
                           <CodeBlock>
                               {children}
                           </CodeBlock>
                        },
                        Tag::List(None) => html! {
                            <Ul marker_type={MarkerType::Disc} class="mb-4 pl-4">
                                {children}
                            </Ul>
                        },
                        Tag::List(Some(_)) => html! {
                            <Ul marker_type={MarkerType::Decimal} class="mb-4 pl-4">
                                {children}
                            </Ul>
                        },

                        Tag::Item => html! {
                            <Li class="mb-1">
                                {children}
                            </Li>
                        },
                        Tag::Emphasis => html! {
                            <Typo tag={TagType::Emphasis}>
                                {children}
                            </Typo>
                        },
                        Tag::Strong => html! {
                            <Typo tag={TagType::Strong}>
                                {children}
                            </Typo>
                        },
                        Tag::Link(_, dest, _) => {
                            html! { <A href={dest.to_string()}>{children}</A> }
                        }
                        Tag::Image(_, src, alt) => html! {
                            <Image
                                src={src.to_string()}
                                alt={alt.to_string()}
                                class="my-4"
                            />
                        },
                        _ => html! { <>{children}</> },
                    };

                    if let Some((_, parent_children)) = tags_stack.last_mut() {
                        parent_children.push(node);
                    } else {
                        html_nodes.push(node);
                    }
                }
            }
            Event::Text(text) => {
                if let Some((_, children)) = tags_stack.last_mut() {
                    children.push(html! { { text.to_string() } });
                } else {
                    html_nodes.push(html! { { text.to_string() } });
                }
            }
            Event::Code(code) => {
                if let Some((_, children)) = tags_stack.last_mut() {
                    children.push(html! { <code class="font-mono bg-gray-100 px-1 rounded">{ code.to_string() }</code> });
                } else {
                    html_nodes.push(html! { <code class="font-mono bg-gray-100 px-1 rounded">{ code.to_string() }</code> });
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
