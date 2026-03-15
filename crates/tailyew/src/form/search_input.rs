use crate::form_deserializer::*;
use crate::system::use_themed_classes;
use crate::SelectOption;
use crate::{Input, InputType, Label, Li, Typo, Ul, XIcon};
use gloo_timers::callback::Timeout;
use js_sys::Date;
use serde::Deserialize;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlInputElement};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone, Default, Deserialize)]
pub struct SearchInputProps {
    #[prop_or_default]
    #[serde(default, deserialize_with = "de_attr")]
    pub id: AttrValue,

    #[prop_or_default]
    #[serde(default)]
    pub items: Vec<SelectOption>,

    #[prop_or_default]
    #[serde(default)]
    pub default_selected: Option<SelectOption>,

    #[prop_or_default]
    #[serde(default, deserialize_with = "de_option_attr")]
    pub placeholder: Option<AttrValue>,

    #[prop_or_default]
    #[serde(default, deserialize_with = "de_option_attr")]
    pub label: Option<AttrValue>,

    #[prop_or(false)]
    #[serde(default)]
    pub required: bool,

    #[prop_or_default]
    #[serde(default, deserialize_with = "de_classes")]
    pub class: Classes,

    #[prop_or_default]
    #[serde(default, rename = "aria-label", deserialize_with = "de_option_attr")]
    pub aria_label: Option<AttrValue>,

    #[prop_or_default]
    #[serde(
        default,
        rename = "aria-labelledby",
        deserialize_with = "de_option_attr"
    )]
    pub aria_labelledby: Option<AttrValue>,

    #[prop_or_default]
    #[serde(
        default,
        rename = "aria-describedby",
        deserialize_with = "de_option_attr"
    )]
    pub aria_describedby: Option<AttrValue>,

    #[prop_or_default]
    #[serde(skip)]
    pub on_select: Option<Callback<String>>,

    #[prop_or_default]
    #[serde(skip)]
    pub on_fetch_more: Option<Callback<()>>,

    #[prop_or(300)]
    #[serde(default)]
    pub debounce_ms: u32,
    #[prop_or("Please select a value from the list.".into())]
    #[serde(default, deserialize_with = "de_attr")]
    pub error_title: AttrValue,

    #[prop_or(false)]
    #[serde(default)]
    pub disabled: bool,
}

#[component(SearchInput)]
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
        disabled,
    } = props.clone();

    let input_id = id.clone();
    let selected_item = use_state(|| default_selected.clone());
    let search_text = use_state(|| {
        default_selected
            .as_ref()
            .map(|i| i.label.clone())
            .unwrap_or_default()
    });

    let input_ref = use_node_ref();
    let dropdown_ref = use_node_ref();
    let filtered = use_state(Vec::<SelectOption>::new);
    let show_dropdown = use_state(|| false);
    let timeout_handle = use_mut_ref(|| None::<Timeout>);
    let dropdown_id =
        use_state(|| AttrValue::from(format!("search-dropdown-{}", Date::now() as u64)));

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
                let mut matches: Vec<SelectOption> = items
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

        Callback::from(move |item: SelectOption| {
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

            let mut list: Vec<SelectOption> = items.to_vec();
            list.sort_by(|a, b| a.value.cmp(&b.value));
            list.dedup_by(|a, b| a.value == b.value);
            filtered.set(list);
        })
    };

    // close dropdown on outside click
    {
        let show_dropdown = show_dropdown.clone();
        let dropdown_ref = dropdown_ref.clone();
        use_effect_with(
            (*show_dropdown, dropdown_ref.clone()),
            move |(is_open, dropdown_ref)| {
                let is_open = *is_open;
                let dropdown_ref = dropdown_ref.clone();
                let show_dropdown = show_dropdown.clone();

                let listener = if is_open {
                    Some(gloo::events::EventListener::new(
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
                    ))
                } else {
                    None
                };
                move || {
                    drop(listener);
                }
            },
        );
    }

    {
        let show_dropdown = show_dropdown.clone();
        use_effect_with(*show_dropdown, move |is_open| {
            let listener = if *is_open {
                let show_dropdown = show_dropdown.clone();
                Some(gloo::events::EventListener::new(
                    &gloo::utils::document(),
                    "keydown",
                    move |e| {
                        if let Some(evt) = e.dyn_ref::<web_sys::KeyboardEvent>() {
                            if evt.key() == "Escape" {
                                show_dropdown.set(false);
                            }
                        }
                    },
                ))
            } else {
                None
            };

            move || drop(listener)
        });
    }

    // cancel outstanding timeout on unmount
    {
        let timeout_handle = timeout_handle.clone();
        use_effect_with((), move |_| {
            move || {
                if let Some(handle) = timeout_handle.borrow_mut().take() {
                    handle.cancel();
                }
            }
        });
    }

    // ensure validity pattern is correct on mount/update
    {
        let input_ref = input_ref.clone();
        let apply_validity = apply_validity.clone();
        let selected_item = selected_item.clone();
        use_effect_with(
            (input_ref.clone(), selected_item.is_some()),
            move |(input_ref, is_selected)| {
                if let Some(input_el) = input_ref.cast::<HtmlInputElement>() {
                    apply_validity(&input_el, *is_selected);
                }
            },
        );
    }

    let root_classes = use_themed_classes(
        "SearchInput",
        "root",
        classes!("relative", "space-y-2"),
        Classes::default(),
    );
    let input_classes =
        use_themed_classes("SearchInput", "input", Classes::default(), class.clone());

    let base_id = input_id.clone();
    let search_id = format!("search_{}", input_id);

    html! {
        <div class={root_classes} ref={dropdown_ref}>
            // hidden field to hold the actual value
            <input
                type="hidden"
                name={base_id.clone()}
                value={selected_item.as_ref().map(|i| i.value.clone()).unwrap_or_default()}
                aria-required={AttrValue::from(required.to_string())}
                required={required}
                aria-describedby={aria_describedby.clone()}
                aria-label={aria_label.clone()}
                aria-labelledby={aria_labelledby.clone()}
            />

            // the visible search input
            <Label for_id={search_id.clone()} required={required} text={label.unwrap_or("".into())} />
            <Input
                node_ref={input_ref.clone()}
                id={search_id.clone()}
                input_type={InputType::Search}
                default_value={(*search_text).clone()}
                placeholder={placeholder.unwrap_or_default()}
                class={input_classes}
                disabled={disabled}
                autocomplete={"off"}
                on_change={Some(oninput)}
                on_focus={Some(on_focus)}
                aria_expanded={Some(AttrValue::from((*show_dropdown).to_string()))}
                aria_controls={Some((*dropdown_id).clone())}
                aria_haspopup={Some(AttrValue::from("listbox"))}
                aria_label={aria_label.clone().map(|v| AttrValue::from(format!("search-{v}")))}
                aria_labelledby={aria_labelledby.clone().map(|v| AttrValue::from(format!("search-{v}")))}
                aria_describedby={aria_describedby.clone().map(|v| AttrValue::from(format!("search-{v}")))}
            />

            {
                if *show_dropdown && !filtered.is_empty() {
                    html! {
                        <div class="relative transition-all duration-200 ease-out transform z-50">
                            <div class="rounded-t border border-b-0 bg-white dark:bg-gray-900 dark:border-gray-700 px-4 pt-3 pb-1">
                                <Typo>{"Select a value from the list"}</Typo>
                            </div>
                            <div id={(*dropdown_id).clone()}>
                                <Ul class="absolute max-h-60 overflow-auto z-50 w-full bg-white shadow rounded-b border-t-0 border dark:bg-gray-900 dark:border-gray-700 transition-all duration-200 ease-in-out">
                                    { for filtered.iter().map(|item| {
                                        let on_click = on_click_item.clone();
                                        let item_clone = item.clone();
                                        html! {
                                            <Li
                                                class="hover:bg-gray-100 dark:hover:bg-gray-800 px-4 py-2 transition-colors duration-150"
                                                on_click={Callback::from(move |_| on_click.emit(item_clone.clone()))}
                                            >
                                                { html! { item.label.clone() } }
                                            </Li>
                                        }
                                    }) }
                                </Ul>
                            </div>
                        </div>
                    }
                } else {
                    html! {}
                }
            }

            {
                if let Some(item) = &*selected_item {
                    html! {
                        <Ul class="text-sm text-gray-700 dark:text-gray-300">
                            <Li
                                on_click={if disabled { None } else { Some(on_clear_selection) }}
                                icon={if disabled { None } else { Some(html! { <XIcon size={12} /> }) }}
                                class="hover:bg-gray-100 dark:hover:bg-gray-800 bg-gray-50 dark:bg-gray-800 rounded px-4 py-2 flex items-center justify-left"
                            >
                                <Typo>{ html! { format!("Selected: {}", item.label) } }</Typo>
                            </Li>
                        </Ul>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}
