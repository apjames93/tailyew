use crate::atoms::{Li, MarkerType, Ul};
use crate::molecules::Accordion;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew::virtual_dom::VNode;

/// Semantic link configuration for a full-width navigation row.
#[derive(Clone, PartialEq)]
pub struct NestedItemLink {
    pub href: AttrValue,
    pub target: Option<AttrValue>,
    pub rel: Option<AttrValue>,
}

impl NestedItemLink {
    pub fn new(href: impl Into<AttrValue>) -> Self {
        Self {
            href: href.into(),
            target: None,
            rel: None,
        }
    }

    pub fn external(href: impl Into<AttrValue>) -> Self {
        Self {
            href: href.into(),
            target: Some("_blank".into()),
            rel: Some("noopener noreferrer".into()),
        }
    }

    fn resolved_rel(&self) -> Option<AttrValue> {
        match (self.target.as_deref(), self.rel.clone()) {
            (Some("_blank"), None) => Some("noopener noreferrer".into()),
            (_, rel) => rel,
        }
    }
}

/// Declares how a leaf [`NestedItem`] should behave.
///
/// This keeps the actual interactive row element in TailYew's control so the
/// full styled surface owns focus, hit target, and semantics.
#[derive(Clone, PartialEq, Default)]
pub enum NestedItemRowKind {
    #[default]
    Select,
    Link(NestedItemLink),
    Content,
    Html,
}

#[derive(Clone, PartialEq)]
pub struct NestedItem {
    pub label: Html,
    pub value: AttrValue,
    pub children: Option<Vec<NestedItem>>,
    pub row_kind: NestedItemRowKind,
}

impl NestedItem {
    pub fn new<T: Into<AttrValue>>(text: T) -> Self {
        let value = text.into();
        Self::leaf(
            html! { { value.clone() } },
            value,
            NestedItemRowKind::Select,
        )
    }

    pub fn with_select<T: Into<AttrValue>>(text: T, value: impl Into<AttrValue>) -> Self {
        let text = text.into();
        Self::leaf(html! { { text } }, value, NestedItemRowKind::Select)
    }

    pub fn with_children<T: Into<AttrValue>>(text: T, children: Vec<NestedItem>) -> Self {
        let value = text.into();
        Self::branch(html! { { value.clone() } }, value, children)
    }

    pub fn with_children_html(
        label: Html,
        value: impl Into<AttrValue>,
        children: Vec<NestedItem>,
    ) -> Self {
        Self::branch(label, value, children)
    }

    /// Prefer `with_select`, `with_link`, or `with_content` when the row
    /// semantics are known ahead of time. This remains supported for
    /// compatibility with HTML labels that render links.
    pub fn with_html(label: Html, value: impl Into<AttrValue>) -> Self {
        Self::leaf(label, value, NestedItemRowKind::Html)
    }

    pub fn with_link<T: Into<AttrValue>>(
        text: T,
        value: impl Into<AttrValue>,
        href: impl Into<AttrValue>,
    ) -> Self {
        let text = text.into();
        Self::leaf(
            html! { { text } },
            value,
            NestedItemRowKind::Link(NestedItemLink::new(href)),
        )
    }

    pub fn with_link_html(
        label: Html,
        value: impl Into<AttrValue>,
        href: impl Into<AttrValue>,
    ) -> Self {
        Self::leaf(
            label,
            value,
            NestedItemRowKind::Link(NestedItemLink::new(href)),
        )
    }

    pub fn with_external_link<T: Into<AttrValue>>(
        text: T,
        value: impl Into<AttrValue>,
        href: impl Into<AttrValue>,
    ) -> Self {
        let text = text.into();
        Self::leaf(
            html! { { text } },
            value,
            NestedItemRowKind::Link(NestedItemLink::external(href)),
        )
    }

    pub fn with_external_link_html(
        label: Html,
        value: impl Into<AttrValue>,
        href: impl Into<AttrValue>,
    ) -> Self {
        Self::leaf(
            label,
            value,
            NestedItemRowKind::Link(NestedItemLink::external(href)),
        )
    }

    /// Creates a non-interactive row container for embedded widgets.
    pub fn with_content(label: Html, value: impl Into<AttrValue>) -> Self {
        Self::leaf(label, value, NestedItemRowKind::Content)
    }

    fn leaf(label: Html, value: impl Into<AttrValue>, row_kind: NestedItemRowKind) -> Self {
        Self {
            label,
            value: value.into(),
            children: None,
            row_kind,
        }
    }

    fn branch(label: Html, value: impl Into<AttrValue>, children: Vec<NestedItem>) -> Self {
        Self {
            label,
            value: value.into(),
            children: Some(children),
            row_kind: NestedItemRowKind::Select,
        }
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct NestedListProps {
    pub list: Vec<NestedItem>,
    /// Emitted when a `Select` or `Link` leaf row is activated.
    pub on_select: Callback<AttrValue>,
    #[prop_or(0)]
    pub start_index: usize,
}

#[component(NestedList)]
pub fn nested_list(props: &NestedListProps) -> Html {
    let NestedListProps {
        list,
        on_select,
        start_index,
    } = props.clone();

    let (html, _) = render_nested_list(list, on_select, start_index);
    html
}

/// Recursively renders a nested list while returning the next available index
fn render_nested_list(
    list: Vec<NestedItem>,
    on_select: Callback<AttrValue>,
    start_index: usize,
) -> (Html, usize) {
    let mut index = start_index;

    let children_html = list
        .into_iter()
        .map(|item| {
            let current_index = index;
            index += 1;

            html! {
                <NestedListItem
                    item={item}
                    on_select={on_select.clone()}
                    index={current_index}
                />
            }
        })
        .collect::<Html>();

    (
        html! {
            <Ul marker_type={MarkerType::None} class="space-y-1">
                { children_html }
            </Ul>
        },
        index,
    )
}

#[derive(Properties, PartialEq, Clone)]
struct NestedListItemProps {
    pub item: NestedItem,
    pub on_select: Callback<AttrValue>,
    pub index: usize,
}

#[component(NestedListItem)]
fn nested_list_item(props: &NestedListItemProps) -> Html {
    let NestedListItemProps {
        item,
        on_select,
        index,
    } = props.clone();
    let NestedItem {
        label,
        value,
        children,
        row_kind,
    } = item;

    if let Some(children) = children {
        let (nested_html, _) = render_nested_list(children, on_select.clone(), index + 1);
        let row_class = format!("w-full {}", striped_row_background(index));

        html! {
            <Li class="w-full">
                <Accordion
                    title={label.clone()}
                    compact={true}
                    class={classes!(row_class)}
                    content_class={classes!("pl-2")}
                    default_open={false}
                >
                    // Inject label manually into Accordion header
                    <div class="font-semibold text-gray-900 dark:text-gray-100 px-4 py-2">
                        { label }
                    </div>
                    { nested_html }
                </Accordion>
            </Li>
        }
    } else {
        render_leaf_row(label, value, row_kind, on_select, index)
    }
}

fn render_leaf_row(
    label: Html,
    value: AttrValue,
    row_kind: NestedItemRowKind,
    on_select: Callback<AttrValue>,
    index: usize,
) -> Html {
    let handle_select = {
        let value = value.clone();
        let on_select = on_select.clone();
        Callback::from(move |_| on_select.emit(value.clone()))
    };

    let row = match row_kind {
        NestedItemRowKind::Select => html! {
            <button
                type="button"
                onclick={handle_select}
                class={interactive_row_classes(index)}
            >
                { label }
            </button>
        },
        NestedItemRowKind::Link(link) => {
            let rel = link.resolved_rel();
            let NestedItemLink { href, target, .. } = link;

            html! {
                <a
                    href={href}
                    target={target}
                    rel={rel}
                    onclick={handle_select}
                    class={interactive_row_classes(index)}
                >
                    { label }
                </a>
            }
        }
        NestedItemRowKind::Content => html! {
            <div class={content_row_classes(index)}>
                { label }
            </div>
        },
        NestedItemRowKind::Html => html! {
            <HtmlRow
                label={label}
                value={value}
                on_select={on_select}
                index={index}
            />
        },
    };

    html! {
        <Li class="w-full">
            { row }
        </Li>
    }
}

fn striped_row_background(index: usize) -> &'static str {
    if index.is_multiple_of(2) {
        "bg-gray-100 dark:bg-gray-800"
    } else {
        "bg-white dark:bg-gray-900"
    }
}

fn interactive_row_classes(index: usize) -> Classes {
    classes!(
        "w-full",
        "px-4",
        "py-3",
        "flex",
        "items-center",
        "rounded-lg",
        "shadow-md",
        "transition",
        "text-sm",
        "font-medium",
        "text-gray-900",
        "dark:text-gray-100",
        "hover:bg-gray-200",
        "dark:hover:bg-gray-700",
        "text-left",
        "no-underline",
        "focus:outline-none",
        "focus:ring-2",
        "focus:ring-accent",
        "focus:ring-offset-2",
        "focus:ring-offset-white",
        "dark:focus:ring-accent-dark",
        "dark:focus:ring-offset-gray-900",
        striped_row_background(index)
    )
}

#[derive(Properties, PartialEq, Clone)]
struct HtmlRowProps {
    pub label: Html,
    pub value: AttrValue,
    pub on_select: Callback<AttrValue>,
    pub index: usize,
}

#[component(HtmlRow)]
fn html_row(props: &HtmlRowProps) -> Html {
    let HtmlRowProps {
        label,
        value,
        on_select,
        index,
    } = props.clone();

    let root_ref = use_node_ref();
    let delegates_to_anchor = use_state(|| html_row_maybe_delegates_to_anchor(&label));

    {
        let root_ref = root_ref.clone();
        let delegates_to_anchor = delegates_to_anchor.clone();
        use_effect_with(label.clone(), move |_| {
            if let Some(root) = root_ref.cast::<web_sys::Element>() {
                let has_anchor = root.query_selector("a[href]").ok().flatten().is_some();

                if has_anchor {
                    style_html_anchor_descendant(&root);
                }

                if *delegates_to_anchor != has_anchor {
                    delegates_to_anchor.set(has_anchor);
                }
            }

            || ()
        });
    }

    if *delegates_to_anchor {
        let handle_click = {
            let value = value.clone();
            let on_select = on_select.clone();
            Callback::from(move |event: MouseEvent| {
                if click_target_has_anchor(&event) {
                    on_select.emit(value.clone());
                }
            })
        };

        html! {
            <div
                ref={root_ref}
                class={html_row_delegate_classes(index)}
                onclick={handle_click}
            >
                { label }
            </div>
        }
    } else {
        let handle_click = {
            let value = value.clone();
            let on_select = on_select.clone();
            Callback::from(move |_| on_select.emit(value.clone()))
        };

        html! {
            <button
                ref={root_ref}
                type="button"
                onclick={handle_click}
                class={interactive_row_classes(index)}
            >
                { label }
            </button>
        }
    }
}

fn content_row_classes(index: usize) -> Classes {
    classes!(
        "w-full",
        "px-4",
        "py-3",
        "flex",
        "items-center",
        "rounded-lg",
        "shadow-md",
        "text-sm",
        "font-medium",
        "text-gray-900",
        "dark:text-gray-100",
        striped_row_background(index)
    )
}

fn html_row_delegate_classes(index: usize) -> Classes {
    classes!(
        "w-full",
        "rounded-lg",
        "shadow-md",
        "overflow-hidden",
        "transition",
        "text-sm",
        "font-medium",
        "text-gray-900",
        "dark:text-gray-100",
        "hover:bg-gray-200",
        "dark:hover:bg-gray-700",
        "focus-within:ring-2",
        "focus-within:ring-accent",
        "focus-within:ring-offset-2",
        "focus-within:ring-offset-white",
        "dark:focus-within:ring-accent-dark",
        "dark:focus-within:ring-offset-gray-900",
        striped_row_background(index)
    )
}

fn style_html_anchor_descendant(root: &web_sys::Element) {
    let Ok(Some(anchor)) = root.query_selector("a[href]") else {
        return;
    };

    if anchor
        .get_attribute("data-tailyew-html-row-anchor")
        .is_some()
    {
        return;
    }

    let existing_style = anchor.get_attribute("style").unwrap_or_default();
    let merged_style = if existing_style.is_empty() {
        html_row_anchor_style().to_owned()
    } else {
        format!("{existing_style};{}", html_row_anchor_style())
    };

    let _ = anchor.set_attribute("style", &merged_style);
    let _ = anchor.set_attribute("data-tailyew-html-row-anchor", "true");
}

fn html_row_anchor_style() -> &'static str {
    "display:flex;width:100%;align-items:center;padding:0.75rem 1rem;box-sizing:border-box;text-align:left;text-decoration:none;color:inherit;"
}

fn html_row_maybe_delegates_to_anchor(node: &VNode) -> bool {
    match node {
        VNode::VComp(_) | VNode::VPortal(_) | VNode::VRef(_) | VNode::VSuspense(_) => true,
        VNode::VTag(tag) => {
            tag.tag() == "a"
                || tag
                    .children()
                    .map(html_row_maybe_delegates_to_anchor)
                    .unwrap_or(false)
        }
        VNode::VList(list) => list.iter().any(html_row_maybe_delegates_to_anchor),
        VNode::VText(_) | VNode::VRaw(_) => false,
    }
}

fn click_target_has_anchor(event: &MouseEvent) -> bool {
    event
        .target()
        .and_then(|target| target.dyn_into::<web_sys::Element>().ok())
        .and_then(|element| element.closest("a[href]").ok().flatten())
        .is_some()
}
