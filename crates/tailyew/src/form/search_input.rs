use crate::{Input, InputType, Li, Typo, Ul, XIcon};
use gloo_timers::callback::Timeout;
use std::rc::Rc;
use web_sys::{HtmlElement, HtmlInputElement};
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    pub label: String,
    pub value: String,
}

#[derive(Properties, PartialEq, Clone)]
pub struct SearchInputProps {
    pub items: Rc<Vec<Item>>,
    #[prop_or_default]
    pub default_selected: Option<Item>,
    #[prop_or_default]
    pub placeholder: Option<AttrValue>,
    #[prop_or_default]
    pub label: Option<AttrValue>,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or(false)]
    pub required: bool,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub aria_label: Option<AttrValue>,
    #[prop_or_default]
    pub aria_labelledby: Option<AttrValue>,
    #[prop_or_default]
    pub aria_describedby: Option<AttrValue>,
    #[prop_or_default]
    pub on_select: Option<Callback<String>>,
    #[prop_or_default]
    pub on_fetch_more: Option<Callback<()>>,
    #[prop_or(300)]
    pub debounce_ms: u32,
    #[prop_or("Please select a value from the list.".into())]
    pub error_title: AttrValue,
}

#[function_component(SearchInput)]
pub fn search_input(props: &SearchInputProps) -> Html {
    let SearchInputProps {
        items,
        default_selected,
        debounce_ms,
        on_select,
        on_fetch_more,
        placeholder,
        label,
        id,
        required,
        class,
        aria_label,
        aria_labelledby,
        aria_describedby,
        error_title,
    } = props.clone();

    let input_id = id.clone().unwrap_or_else(|| "search".into());
    let selected_item = use_state(|| default_selected.clone());
    let search_text = use_state(|| {
        default_selected
            .as_ref()
            .map(|i| i.label.clone())
            .unwrap_or_default()
    });

    let input_ref = use_node_ref();
    let dropdown_ref = use_node_ref();
    let filtered = use_state(Vec::new);
    let show_dropdown = use_state(|| false);
    let timeout_handle = use_mut_ref(|| None::<Timeout>);

    let apply_validity = {
        let error_title = error_title.clone();
        move |el: &HtmlInputElement, is_valid: bool| {
            if required {
                el.set_pattern(if is_valid { ".*" } else { "^$a" });
                el.set_custom_validity(if is_valid { "" } else { &error_title });
            } else {
                el.set_pattern(".*");
                el.set_custom_validity("");
            }
        }
    };

    // Debounced input change
    let oninput = {
        let filtered = filtered.clone();
        let items = items.clone();
        let timeout_handle = timeout_handle.clone();
        let on_fetch_more = on_fetch_more.clone();
        let show_dropdown = show_dropdown.clone();
        let input_ref = input_ref.clone();
        let selected_item = selected_item.clone();
        let apply_validity = apply_validity.clone();

        Callback::from(move |val: String| {
            if let Some(input_el) = input_ref.cast::<HtmlInputElement>() {
                apply_validity(&input_el, selected_item.is_some());
            }

            show_dropdown.set(true);

            if let Some(existing) = timeout_handle.borrow_mut().take() {
                existing.cancel();
            }

            let query = val.to_lowercase();
            let items = items.clone();
            let filtered = filtered.clone();
            let on_fetch_more = on_fetch_more.clone();

            Timeout::new(debounce_ms, move || {
                let mut matches: Vec<Item> = items
                    .iter()
                    .filter(|item| {
                        item.label.to_lowercase().contains(&query)
                            || item.value.to_lowercase().contains(&query)
                    })
                    .cloned()
                    .collect();

                matches.sort_by(|a, b| a.value.cmp(&b.value));
                matches.dedup_by(|a, b| a.value == b.value);

                if matches.is_empty() {
                    if let Some(fetch) = on_fetch_more {
                        fetch.emit(());
                    }
                }

                filtered.set(matches);
            })
            .forget();
        })
    };

    let on_click_item = {
        let selected_item = selected_item.clone();
        let filtered = filtered.clone();
        let on_select = on_select.clone();
        let show_dropdown = show_dropdown.clone();
        let input_ref = input_ref.clone();
        let apply_validity = apply_validity.clone();

        Callback::from(move |item: Item| {
            selected_item.set(Some(item.clone()));
            filtered.set(vec![]);
            show_dropdown.set(false);

            if let Some(input_el) = input_ref.cast::<HtmlInputElement>() {
                input_el.set_value(&item.label);
                apply_validity(&input_el, true);
            }

            if let Some(cb) = &on_select {
                cb.emit(item.value.clone());
            }
        })
    };

    let on_clear_selection = {
        let selected_item = selected_item.clone();
        let input_ref = input_ref.clone();
        let apply_validity = apply_validity.clone();

        Callback::from(move |_| {
            selected_item.set(None);

            if let Some(input_el) = input_ref.cast::<HtmlInputElement>() {
                input_el.set_value("");
                apply_validity(&input_el, false);
            }
        })
    };

    let on_focus = {
        let filtered = filtered.clone();
        let items = items.clone();
        let show_dropdown = show_dropdown.clone();

        Callback::from(move |_: FocusEvent| {
            show_dropdown.set(true);

            let mut list: Vec<Item> = items.iter().cloned().collect();
            list.sort_by(|a, b| a.value.cmp(&b.value));
            list.dedup_by(|a, b| a.value == b.value);
            filtered.set(list);
        })
    };

    {
        let show_dropdown = show_dropdown.clone();
        let dropdown_ref = dropdown_ref.clone();
        use_effect(move || {
            let listener = gloo::events::EventListener::new(
                &gloo::utils::document(),
                "mousedown",
                move |event| {
                    if let Some(target) = event.target_dyn_into::<HtmlElement>() {
                        if let Some(dropdown) = dropdown_ref.cast::<HtmlElement>() {
                            if !dropdown.contains(Some(&target)) {
                                show_dropdown.set(false);
                            }
                        }
                    }
                },
            );
            move || drop(listener)
        });
    }

    {
        let timeout_handle = timeout_handle.clone();
        use_effect(move || {
            move || {
                if let Some(handle) = timeout_handle.borrow_mut().take() {
                    handle.cancel();
                }
            }
        });
    }

    {
        let input_ref = input_ref.clone();
        let apply_validity = apply_validity.clone();
        let selected_item = selected_item.clone();

        use_effect(move || {
            if let Some(input_el) = input_ref.cast::<HtmlInputElement>() {
                apply_validity(&input_el, selected_item.is_some());
            }
            || ()
        });
    }

    let base_id = input_id.clone();
    let search_id = format!("search_{}", input_id);

    // Use prefixed IDs for visible input only
    let effective_aria_label = aria_label.clone();
    let effective_aria_labelledby = aria_labelledby.clone();
    let effective_aria_describedby = aria_describedby.clone();

    html! {
        <div class="relative space-y-2" ref={dropdown_ref}>
            <input
                type="hidden"
                name={base_id.clone()}
                value={selected_item.as_ref().map(|i| i.value.clone()).unwrap_or_default()}
                aria-required={AttrValue::from(required.to_string())}
                aria-describedby={aria_describedby.clone()}
                aria-label={aria_label.clone()}
                aria-labelledby={aria_labelledby.clone()}
            />

            <Input
                node_ref={input_ref.clone()}
                id={search_id.clone()}
                label={label.unwrap_or_else(|| "".into())}
                input_type={InputType::Search}
                default_value={(*search_text).clone()}
                placeholder={placeholder.unwrap_or_default()}
                class={class.clone()}
                on_change={Some(oninput)}
                on_focus={Some(on_focus)}
                aria_label={Some(AttrValue::from(format!("search-{}", effective_aria_label.unwrap_or_default())))}
                aria_labelledby={Some(AttrValue::from(format!("search-{}", effective_aria_labelledby.unwrap_or_default())))}
                aria_describedby={Some(AttrValue::from(format!("search-{}", effective_aria_describedby.unwrap_or_default())))}
            />

            if *show_dropdown && !filtered.is_empty() {
                <div class="relative transition-all duration-200 ease-out transform z-50 ">
                    <div class="rounded-t border border-b-0 bg-white dark:bg-gray-900 dark:border-gray-700 px-4 pt-3 pb-1">
                        <Typo>{"Select a value from the list"}</Typo>
                    </div>
                    <Ul class="absolute max-h-60 overflow-auto z-50 w-full bg-white shadow rounded-b border-t-0 border dark:bg-gray-900 dark:border-gray-700 transition-all duration-200 ease-in-out">
                        {
                            for filtered.iter().map(|item| {
                                let on_click_item = on_click_item.clone();
                                let item_clone = item.clone();
                                html! {
                                    <Li
                                        class="hover:bg-gray-100 dark:hover:bg-gray-800 px-4 py-2 transition-colors duration-150"
                                        onclick={Callback::from(move |_| on_click_item.emit(item_clone.clone()))}
                                    >
                                        { item.label.clone() }
                                    </Li>
                                }
                            })
                        }
                    </Ul>
                </div>
            }

            if let Some(item) = &*selected_item {
                <Ul class="text-sm text-gray-700 dark:text-gray-300">
                    <Li
                        onclick={Some(on_clear_selection)}
                        icon={html! { <XIcon size={12} /> }}
                        with_icon={true}
                        class="hover:bg-gray-100 dark:hover:bg-gray-800 bg-gray-50 dark:bg-gray-800 rounded px-4 py-2 flex items-center justify-left"
                    >
                        <Typo>{ format!("Selected: {}", item.label) }</Typo>
                    </Li>
                </Ul>
            }
        </div>
    }
}
