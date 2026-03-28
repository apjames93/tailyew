use web_sys::{HtmlElement, KeyboardEvent, ScrollBehavior, ScrollIntoViewOptions};
use yew::{AttrValue, prelude::*};

#[derive(Clone, PartialEq)]
pub struct TabItem {
    pub title: AttrValue,
    pub content: Html,
}

fn clamp_tab_index(index: usize, item_count: usize) -> usize {
    if item_count == 0 {
        0
    } else {
        index.min(item_count - 1)
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct TabsProps {
    /// List of tabs (title + content)
    pub items: Vec<TabItem>,
    /// Enable smooth scroll-into-view on tab select
    #[prop_or(false)]
    pub scroll_into_view: bool,
    /// Optional id prefix for tab and panel aria wiring
    #[prop_or_default]
    pub id_prefix: Option<AttrValue>,
    /// Initial active tab index for uncontrolled usage
    #[prop_or(0)]
    pub initial_active_tab: usize,
    /// Controlled active tab index
    #[prop_or_default]
    pub active_tab: Option<usize>,
    /// Optional callback fired whenever tab selection changes
    #[prop_or_default]
    pub on_tab_change: Option<Callback<usize>>,
}

#[component(Tabs)]
pub fn tabs(props: &TabsProps) -> Html {
    let scroll_into_view = props.scroll_into_view;
    let item_count = props.items.len();
    let base_id = props
        .id_prefix
        .clone()
        .unwrap_or_else(|| AttrValue::from("tabs"))
        .to_string();
    let initial_active_tab = clamp_tab_index(props.initial_active_tab, item_count);
    let active_tab_index = use_state(move || initial_active_tab);
    let effective_active_tab_index =
        clamp_tab_index(props.active_tab.unwrap_or(*active_tab_index), item_count);
    let base_id_for_click = base_id.clone();
    let tab_refs = use_mut_ref(Vec::<NodeRef>::new);

    {
        let mut refs = tab_refs.borrow_mut();
        if refs.len() < props.items.len() {
            refs.resize_with(props.items.len(), NodeRef::default);
        }
    }

    // Click handler: set active index and optionally scroll tab into view
    let on_tab_click = {
        let active_tab_index = active_tab_index.clone();
        let is_controlled = props.active_tab.is_some();
        let on_tab_change = props.on_tab_change.clone();
        let base_id = base_id_for_click;
        Callback::from(move |index: usize| {
            let next_index = clamp_tab_index(index, item_count);

            if !is_controlled {
                active_tab_index.set(next_index);
            }

            if let Some(on_tab_change) = &on_tab_change {
                on_tab_change.emit(next_index);
            }

            if scroll_into_view
                && let Some(window) = web_sys::window()
                && let Some(doc) = window.document()
                && let Some(el) = doc.get_element_by_id(&format!("{}-tab-{}", base_id, next_index))
            {
                let opts = ScrollIntoViewOptions::new();
                opts.set_behavior(ScrollBehavior::Smooth);
                el.scroll_into_view_with_scroll_into_view_options(&opts);
            }
        })
    };

    // Helper for styling tabs
    let tab_styles = |is_active: bool| {
        let base = if is_active {
            "px-4 py-2 text-blue-500 dark:text-blue-300 border-b-2 border-blue-500 dark:border-blue-300 font-medium cursor-pointer transition duration-200"
        } else {
            "px-4 py-2 text-gray-600 dark:text-gray-400 border-b-2 border-transparent font-medium cursor-pointer hover:text-blue-500 dark:hover:text-blue-300 transition duration-200"
        };
        format!(
            "{} focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-300",
            base
        )
    };

    // Render content for active tab
    let content = props
        .items
        .get(effective_active_tab_index)
        .map(|tab| tab.content.clone())
        .unwrap_or_else(|| html! { <div>{"No content available"}</div> });

    let active_tab_id = format!("{}-tab-{}", base_id, effective_active_tab_index);
    let active_panel_id = format!("{}-panel-{}", base_id, effective_active_tab_index);

    html! {
        <div class="tabs-component w-full p-2">
            <div
                class="flex flex-nowrap overflow-x-auto snap-x snap-mandatory border-b border-gray-200 dark:border-gray-700"
                role="tablist"
            >
                { for props.items.iter().enumerate().map(|(index, item)| {
                    let is_active = index == effective_active_tab_index;
                    let tab_ref = {
                        let refs = tab_refs.borrow_mut();
                        refs[index].clone()
                    };
                    let tab_id = format!("{}-tab-{}", base_id, index);
                    let panel_id = format!("{}-panel-{}", base_id, index);
                    let onclick = {
                        let on_tab_click = on_tab_click.clone();
                        Callback::from(move |_| on_tab_click.emit(index))
                    };
                    let onkeydown = {
                        let on_tab_click = on_tab_click.clone();
                        let tab_refs = tab_refs.clone();
                        let total = props.items.len();
                        Callback::from(move |e: KeyboardEvent| {
                            match e.key().as_str() {
                                "ArrowRight" => {
                                    e.prevent_default();
                                    let next = (index + 1) % total;
                                    on_tab_click.emit(next);
                                    if let Some(el) = tab_refs.borrow().get(next).and_then(|r| r.cast::<HtmlElement>()) {
                                        let _ = el.focus();
                                    }
                                }
                                "ArrowLeft" => {
                                    e.prevent_default();
                                    let prev = if index == 0 { total.saturating_sub(1) } else { index - 1 };
                                    on_tab_click.emit(prev);
                                    if let Some(el) = tab_refs.borrow().get(prev).and_then(|r| r.cast::<HtmlElement>()) {
                                        let _ = el.focus();
                                    }
                                }
                                "Enter" | " " => {
                                    e.prevent_default();
                                    on_tab_click.emit(index);
                                }
                                _ => {}
                            }
                        })
                    };

                    html! {
                        <div
                            key={index}
                            id={tab_id.clone()}
                            class={classes!("flex-shrink-", "snap-start", tab_styles(is_active))}
                            role="tab"
                            tabindex={if is_active { "0" } else { "-1" }}
                            aria-selected={is_active.to_string()}
                            aria-controls={panel_id.clone()}
                            onclick={onclick.clone()}
                            onkeydown={onkeydown}
                            ref={tab_ref}
                        >
                            { &item.title }
                        </div>
                    }
                }) }
            </div>

            <div
                id={active_panel_id}
                role="tabpanel"
                class="mt-4"
                aria-labelledby={active_tab_id}
                tabindex="0"
            >
                { content }
            </div>
        </div>
    }
}
