use crate::{
    atoms::{Button, ButtonType, TagType, Typo},
    organisms::{NestedItem, NestedList},
};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct SidebarButton {
    pub icon: Html,
    pub list: Vec<NestedItem>,
}

#[derive(Properties, PartialEq, Clone)]
pub struct SidebarProps {
    pub icon_list: Vec<SidebarButton>,
    pub on_select: Callback<AttrValue>,
    #[prop_or(true)]
    pub auto_close: bool,
    #[prop_or_default]
    pub top_offset_class: Classes,
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    let SidebarProps {
        icon_list,
        on_select,
        auto_close,
        top_offset_class,
    } = props.clone();

    let active_index = use_state(|| None::<usize>);

    html! {
        <>
            // Overlay on mobile when any drawer is open
            {
                if active_index.is_some() {
                    let close_sidebar = {
                        let active_index = active_index.clone();
                        Callback::from(move |_| active_index.set(None))
                    };
                    html! {
                        <div
                            class="fixed inset-0 bg-black bg-opacity-40 z-30 md:hidden"
                            onclick={close_sidebar}
                        />
                    }
                } else {
                    html! {}
                }
            }


            <div class="relative flex">

                // Sidebar Toggle Buttons (always visible)
                <div
                    class={classes!(
                            "fixed", "left-0", "z-50", "w-14", "h-screen", "bg-white", "dark:bg-gray-900",
                            "border-r", "border-gray-200", "dark:border-gray-700",
                            "flex", "flex-col", "items-center", "space-y-2",
                            top_offset_class.clone()
                        )}
                    >
                    {
                        for icon_list.iter().enumerate().map(|(i, btn)| {
                            let is_active = *active_index == Some(i);
                            let set_active = {
                                let active_index = active_index.clone();
                                Callback::from(move |_| {
                                    if is_active {
                                        active_index.set(None)
                                    } else {
                                        active_index.set(Some(i))
                                    }
                                })
                            };

                            html! {
                                <Button
                                    button_type={ButtonType::Ghost}
                                    class={classes!("p-2", if is_active { "bg-gray-200 dark:bg-gray-800" } else { "" })}
                                    onclick={set_active}
                                >
                                    { btn.icon.clone() }
                                </Button>
                            }
                        })
                    }
                </div>

                // Sidebar Drawers (slide-out)
                {
                    for icon_list.iter().enumerate().map(|(i, btn)| {
                        let is_open = *active_index == Some(i);

                        let close_sidebar = {
                            let active_index = active_index.clone();
                            Callback::from(move |_| active_index.set(None))
                        };

                        let item_on_select = {
                            let on_select = on_select.clone();
                            let close_sidebar = close_sidebar.clone();
                            Callback::from(move |value: AttrValue| {
                                on_select.emit(value.clone());
                                if auto_close {
                                    close_sidebar.emit(());
                                }
                            })
                        };


                        let drawer_classes = classes!(
                            "fixed",
                            "inset-y-0",
                            "left-14",
                            "w-64",
                            "bg-white",
                            "dark:bg-gray-900",
                            "border-r",
                            "border-gray-200",
                            "dark:border-gray-700",
                            "overflow-y-auto",
                            "p-4",
                            "z-40",
                            "transition-transform",
                            "duration-300",
                            "ease-in-out",
                            if is_open { "translate-x-0" } else { "-translate-x-full" }
                        );

                        html! {
                            <div class={drawer_classes}>
                                <NestedList list={btn.list.clone()} on_select={item_on_select} />
                            </div>
                        }
                    })
                }
            </div>
        </>
    }
}
