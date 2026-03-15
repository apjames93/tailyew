use crate::form::FormSubmitCallback;
use crate::system::use_themed_classes;
use crate::{CodeBlock, Image, Li, MarkerType, TagType, Typo, Ul, A};
use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Parser, Tag};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MarkdownProps {
    pub content: String,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub on_form_submit: Option<FormSubmitCallback>,
}

#[component(Markdown)]
pub fn markdown(props: &MarkdownProps) -> Html {
    let parser = Parser::new(&props.content);
    let nodes = markdown_to_yew(parser, props.on_form_submit.clone());
    let classes = use_themed_classes(
        "Markdown",
        "root",
        classes!("text-left", "max-w-none"),
        props.class.clone(),
    );

    html! {
        <div class={classes}>
            { nodes }
        </div>
    }
}

// --- Core Markdown Parser ---

fn markdown_to_yew(parser: Parser, on_form_submit: Option<FormSubmitCallback>) -> Html {
    let mut html_nodes = Vec::new();
    let mut tags_stack = Vec::new();

    for event in parser {
        match event {
            Event::Start(tag) => {
                tags_stack.push((tag, Vec::new()));
            }

            Event::End(_) => {
                if let Some((tag, children)) = tags_stack.pop() {
                    let node = render_tag(tag, children, on_form_submit.clone());
                    push_to_stack_or_root(&mut tags_stack, &mut html_nodes, node);
                }
            }

            Event::Text(text) => {
                push_to_stack_or_root(
                    &mut tags_stack,
                    &mut html_nodes,
                    html! { { text.to_string() } },
                );
            }

            Event::Code(code) => {
                let node = html! {
                    <code class="font-mono bg-gray-100 px-1 rounded">{ code.to_string() }</code>
                };
                push_to_stack_or_root(&mut tags_stack, &mut html_nodes, node);
            }

            Event::SoftBreak | Event::HardBreak => {
                push_to_stack_or_root(&mut tags_stack, &mut html_nodes, html! { <br /> });
            }

            _ => {}
        }
    }

    Html::from_iter(html_nodes)
}

fn push_to_stack_or_root(stack: &mut Vec<(Tag, Vec<Html>)>, root: &mut Vec<Html>, node: Html) {
    if let Some((_, parent)) = stack.last_mut() {
        parent.push(node);
    } else {
        root.push(node);
    }
}

fn render_tag(tag: Tag, children: Vec<Html>, on_form_submit: Option<FormSubmitCallback>) -> Html {
    match tag {
        Tag::Paragraph => html! { <Typo tag={TagType::P}>{children}</Typo> },

        Tag::Heading { level, .. } => {
            let tag_type = match level {
                HeadingLevel::H1 => TagType::H1,
                HeadingLevel::H2 => TagType::H2,
                HeadingLevel::H3 => TagType::H3,
                HeadingLevel::H4 => TagType::H4,
                HeadingLevel::H5 => TagType::H5,
                HeadingLevel::H6 => TagType::H6,
            };
            html! { <Typo tag={tag_type}>{children}</Typo> }
        }

        Tag::BlockQuote { .. } => html! {
            <Typo tag={TagType::BlockQuote}>{children}</Typo>
        },

        Tag::List(None) => html! {
            <Ul marker_type={MarkerType::Disc} class="mb-4 pl-4">{children}</Ul>
        },

        Tag::List(Some(_)) => html! {
            <Ul marker_type={MarkerType::Decimal} class="mb-4 pl-4">{children}</Ul>
        },

        Tag::Item => html! { <Li class="mb-1">{children}</Li> },

        Tag::Emphasis => html! { <Typo tag={TagType::Emphasis}>{children}</Typo> },
        Tag::Strong => html! { <Typo tag={TagType::Strong}>{children}</Typo> },

        Tag::Link { dest_url, .. } => html! {
            <A href={dest_url.to_string()}>{children}</A>
        },

        Tag::Image {
            dest_url, title, ..
        } => html! {
            <Image src={dest_url.to_string()} alt={title.to_string()} class="my-4" />
        },

        Tag::CodeBlock(kind) => {
            let language = match kind {
                CodeBlockKind::Fenced(lang) => Some(lang.to_string()),
                CodeBlockKind::Indented => None,
            };
            html! {
                <CodeBlock language={language} onsubmit={on_form_submit}>
                    {children}
                </CodeBlock>
            }
        }

        _ => html! { <>{children}</> },
    }
}
