use crate::form::FormSubmitCallback;
use crate::{A, CodeBlock, Column, Image, Li, MarkerType, Table, TagType, Typo, Ul};
use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag};
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
    let parser = Parser::new_ext(&props.content, markdown_options());
    let nodes = markdown_to_yew(parser, props.on_form_submit.clone());

    html! {
        <div class={classes!("text-left", "max-w-none", props.class.clone())}>
            { nodes }
        </div>
    }
}

// --- Core Markdown Parser ---

fn markdown_options() -> Options {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options
}

enum Frame<'a> {
    Html { tag: Tag<'a>, children: Vec<Html> },
    Table(TableState),
    TableHead(Vec<Html>),
    TableRow(Vec<Html>),
    TableCell(Vec<Html>),
}

#[derive(Default)]
struct TableState {
    headers: Vec<Html>,
    rows: Vec<Vec<Html>>,
}

fn markdown_to_yew(parser: Parser<'_>, on_form_submit: Option<FormSubmitCallback>) -> Html {
    let mut html_nodes = Vec::new();
    let mut tags_stack = Vec::new();

    for event in parser {
        match event {
            Event::Start(tag) => {
                let frame = match tag {
                    Tag::Table(_) => Frame::Table(TableState::default()),
                    Tag::TableHead => Frame::TableHead(Vec::new()),
                    Tag::TableRow => Frame::TableRow(Vec::new()),
                    Tag::TableCell => Frame::TableCell(Vec::new()),
                    _ => Frame::Html {
                        tag,
                        children: Vec::new(),
                    },
                };
                tags_stack.push(frame);
            }

            Event::End(_) => {
                if let Some(frame) = tags_stack.pop() {
                    match frame {
                        Frame::Html { tag, children } => {
                            let node = render_tag(tag, children, on_form_submit.clone());
                            push_to_stack_or_root(&mut tags_stack, &mut html_nodes, node);
                        }
                        Frame::Table(table) => {
                            let node = render_table(table);
                            push_to_stack_or_root(&mut tags_stack, &mut html_nodes, node);
                        }
                        Frame::TableHead(cells) => {
                            if let Some(table) = last_table_mut(&mut tags_stack) {
                                table.headers = cells;
                            }
                        }
                        Frame::TableRow(cells) => {
                            if let Some(table) = last_table_mut(&mut tags_stack) {
                                table.rows.push(cells);
                            }
                        }
                        Frame::TableCell(children) => {
                            if let Some(frame) = tags_stack.last_mut() {
                                match frame {
                                    Frame::TableHead(cells) => {
                                        cells.push(render_table_header_cell(children));
                                    }
                                    Frame::TableRow(cells) => {
                                        cells.push(render_table_cell(children));
                                    }
                                    _ => {
                                        let cell = render_table_cell(children);
                                        push_to_stack_or_root(
                                            &mut tags_stack,
                                            &mut html_nodes,
                                            cell,
                                        );
                                    }
                                }
                            } else {
                                let cell = render_table_cell(children);
                                push_to_stack_or_root(&mut tags_stack, &mut html_nodes, cell);
                            }
                        }
                    }
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
                let node = render_inline_code(code.to_string());
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

fn push_to_stack_or_root(stack: &mut Vec<Frame<'_>>, root: &mut Vec<Html>, node: Html) {
    if let Some(Frame::Html { children, .. } | Frame::TableCell(children)) = stack.last_mut() {
        children.push(node);
        return;
    }

    root.push(node);
}

fn render_table_header_cell(children: Vec<Html>) -> Html {
    render_table_inline(children)
}

fn render_inline_code(code: String) -> Html {
    html! {
        <code class="font-mono text-[0.875em] px-1.5 py-0.5 rounded border border-gray-200 bg-gray-100 text-gray-800 dark:border-gray-600 dark:bg-gray-800 dark:text-gray-100">
            { code }
        </code>
    }
}

fn last_table_mut<'a, 'b>(stack: &'a mut [Frame<'b>]) -> Option<&'a mut TableState> {
    stack.iter_mut().rev().find_map(|frame| match frame {
        Frame::Table(table) => Some(table),
        _ => None,
    })
}

fn render_table_cell(children: Vec<Html>) -> Html {
    if children.is_empty() {
        Html::default()
    } else {
        html! {
            <Typo tag={TagType::Span} class="text-gray-700 dark:text-gray-300">
                { render_table_inline(children) }
            </Typo>
        }
    }
}

fn render_table_inline(children: Vec<Html>) -> Html {
    match children.len() {
        0 => Html::default(),
        1 => children.into_iter().next().unwrap_or_default(),
        _ => html! { <>{children}</> },
    }
}

fn render_table(table: TableState) -> Html {
    let column_count = table
        .rows
        .iter()
        .map(Vec::len)
        .fold(table.headers.len(), usize::max);

    if column_count == 0 {
        return Html::default();
    }

    let columns = (0..column_count)
        .map(|index| Column {
            header: table.headers.get(index).cloned().unwrap_or_default(),
            values: table
                .rows
                .iter()
                .map(|row| row.get(index).cloned().unwrap_or_default())
                .collect(),
        })
        .collect::<Vec<_>>();

    html! { <Table columns={columns} /> }
}

fn render_tag(
    tag: Tag<'_>,
    children: Vec<Html>,
    on_form_submit: Option<FormSubmitCallback>,
) -> Html {
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

        Tag::BlockQuote(_) => html! {
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
