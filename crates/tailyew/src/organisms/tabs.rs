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
    let TabsProps { items } = props;
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

    html! {
        <div class="tabs-component w-full p-2">
            <div class="flex border-b border-gray-200 dark:border-gray-700">
                { for items.iter().enumerate().map(|(index, item)| {
                    let is_active = index == *active_tab_index;
                    let on_tab_click = on_tab_click.clone();

                    html! {
                        <div
                            class={tab_styles(is_active)}
                            role="tab"
                            tabindex={if is_active { "0" } else { "-1" }}
                            aria-selected={is_active.to_string()}
                            onclick={Callback::from(move |_| on_tab_click.emit(index))}
                        >
                            { &item.title }
                        </div>
                    }
                })}
            </div>

            <div class="mt-4">
                {
                    if let Some(tab) = items.get(*active_tab_index) {
                        tab.content.clone()
                    } else {
                        html! { <div>{"No content available"}</div> }
                    }
                }
            </div>
        </div>
    }
}
