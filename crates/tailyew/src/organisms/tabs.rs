use yew::{prelude::*, AttrValue};

#[derive(Clone, PartialEq)]
pub struct TabItem {
    pub title: AttrValue,
    pub content: Html,
}

#[derive(Properties, PartialEq, Clone)]
pub struct TabsProps {
    pub items: Vec<TabItem>,
}

#[function_component(Tabs)]
pub fn tabs(props: &TabsProps) -> Html {
    let active_tab_index = use_state(|| 0);

    let on_tab_click = {
        let active_tab_index = active_tab_index.clone();
        Callback::from(move |index: usize| active_tab_index.set(index))
    };

    let tab_styles = |is_active: bool| {
        if is_active {
            "px-4 py-2 text-blue-500 dark:text-blue-300 border-b-2 border-blue-500 dark:border-blue-300 font-medium cursor-pointer transition duration-200"
        } else {
            "px-4 py-2 text-gray-600 dark:text-gray-400 border-b-2 border-transparent font-medium cursor-pointer hover:text-blue-500 dark:hover:text-blue-300 transition duration-200"
        }
    };

    let content = props
        .items
        .get(*active_tab_index)
        .map(|tab| tab.content.clone())
        .unwrap_or_else(|| html! { <div>{"No content available"}</div> });

    html! {
        <div class="tabs-component w-full p-2">
            <div
                class="flex border-b border-gray-200 dark:border-gray-700"
                role="tablist"
            >
                { for props.items.iter().enumerate().map(|(index, item)| {
                    let is_active = index == *active_tab_index;
                    let onclick = {
                        let on_tab_click = on_tab_click.clone();
                        Callback::from(move |_| on_tab_click.emit(index))
                    };

                    html! {
                        <div
                            key={index}
                            id={format!("tab-{}", index)}
                            class={tab_styles(is_active)}
                            role="tab"
                            tabindex={if is_active { "0" } else { "-1" }}
                            aria-selected={is_active.to_string()}
                            aria-controls={format!("tabpanel-{}", index)}
                            onclick={onclick}
                        >
                            { &item.title }
                        </div>
                    }
                }) }
            </div>

            <div
                id={format!("tabpanel-{}", *active_tab_index)}
                role="tabpanel"
                class="mt-4"
                aria-labelledby={format!("tab-{}", *active_tab_index)}
            >
                { content }
            </div>
        </div>
    }
}
