use crate::atoms::{Li, MarkerType, Typo, Ul};
use crate::molecules::Accordion;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct NestedItem {
    pub text: AttrValue,
    pub value: AttrValue,
    pub children: Option<Vec<NestedItem>>,
}

impl NestedItem {
    pub fn new<T: Into<AttrValue>>(text: T) -> Self {
        let value = text.into();
        Self {
            text: value.clone(),
            value,
            children: None,
        }
    }

    pub fn with_children<T: Into<AttrValue>>(text: T, children: Vec<NestedItem>) -> Self {
        let value = text.into();
        Self {
            text: value.clone(),
            value,
            children: Some(children),
        }
    }

    pub fn with_value<T: Into<AttrValue>>(text: T, value: T) -> Self {
        Self {
            text: text.into(),
            value: value.into(),
            children: None,
        }
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct NestedListProps {
    pub list: Vec<NestedItem>,
    pub on_select: Callback<AttrValue>,
    #[prop_or(0)]
    pub start_index: usize,
}

#[function_component(NestedList)]
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

#[function_component(NestedListItem)]
fn nested_list_item(props: &NestedListItemProps) -> Html {
    let NestedListItemProps {
        item,
        on_select,
        index,
    } = props.clone();

    let handle_click = {
        let value = item.value.clone();
        let on_select_clone = on_select.clone();
        Callback::from(move |_| on_select_clone.emit(value.clone()))
    };

    let row_class = format!(
        "w-full {}",
        if index % 2 == 0 {
            "bg-gray-100 dark:bg-gray-800"
        } else {
            "bg-white dark:bg-gray-900"
        }
    );

    if let Some(children) = item.children.clone() {
        let (nested_html, _) = render_nested_list(children, on_select.clone(), index + 1);

        html! {
            <Li class="w-full">
                <Accordion
                    title={item.text.to_string()}
                    compact={true}
                    class={classes!(row_class)}
                    content_class="pl-2"
                    default_open={false}
                >
                    { nested_html }
                </Accordion>
            </Li>
        }
    } else {
        html! {
            <Li class="w-full">
                <div
                    role="button"
                    tabindex={0}
                    onclick={handle_click}
                    class={classes!(
                        "w-full", "px-4", "py-3", "flex", "items-center", "rounded-lg", "shadow-md",
                        "transition", "text-sm", "font-medium", "text-gray-900", "dark:text-gray-100",
                        "hover:bg-gray-200", "dark:hover:bg-gray-700",
                        row_class
                    )}
                >
                    <Typo class="text-sm font-medium m-0">{ item.text.clone() }</Typo>
                </div>
            </Li>
        }
    }
}
