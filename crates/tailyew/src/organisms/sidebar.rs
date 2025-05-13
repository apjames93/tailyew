use crate::{
    atoms::{Button, ButtonType, Typo},
    organisms::{NestedItem, NestedList},
};
use yew::prelude::*;

#[derive(PartialEq, Clone, Default)]
pub enum SidebarPosition {
    #[default]
    Left,
    Right,
    Static,
}

#[derive(Properties, PartialEq, Clone)]
pub struct SidebarButton {
    pub icon: Html,
    pub list: Vec<NestedItem>,
    #[prop_or(html! { {"Menu"} })]
    pub open_text: Html,
}

#[derive(Properties, PartialEq, Clone)]
pub struct SidebarProps {
    pub icon_list: Vec<SidebarButton>,
    pub on_select: Callback<AttrValue>,
    #[prop_or(true)]
    pub auto_close: bool,
    #[prop_or_default]
    pub top_offset_class: Classes,
    #[prop_or_default]
    pub position: SidebarPosition,
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    let SidebarProps {
        icon_list,
        on_select,
        auto_close,
        top_offset_class,
        position,
    } = props.clone();

    let active_index = use_state(|| None::<usize>);

    let side_class = match position {
        SidebarPosition::Left => "left-0",
        SidebarPosition::Right => "right-0",
        SidebarPosition::Static => "relative",
    };

    let overlay = if active_index.is_some() && !matches!(position, SidebarPosition::Static) {
        let close = {
            let active_index = active_index.clone();
            Callback::from(move |_| active_index.set(None))
        };

        html! {
            <div
                class="fixed inset-0 z-30 bg-transparent"
                onclick={close}
            />
        }
    } else {
        html! {}
    };

    html! {
        <>
            { overlay }

            <div
            class={classes!(
                if matches!(position, SidebarPosition::Static) { "" } else { "fixed z-40 h-screen" },
                "top-0",
                side_class,
                "bg-white",
                "dark:bg-gray-900",
                "border-r",
                "border-gray-200",
                "dark:border-gray-700",
                "transition-all",
                "duration-300",
                "flex",
                "flex-col",
                "items-stretch",
                "overflow-y-auto",
                if matches!(position, SidebarPosition::Static) {
                    "h-full"
                } else {
                    ""
                },
                if active_index.is_some() { "w-64" } else { "w-14" },
                top_offset_class.clone(),
            )}
            >
                {
                    for icon_list.iter().enumerate().map(|(i, btn)| {
                        let is_active = *active_index == Some(i);

                        let toggle = {
                            let active_index = active_index.clone();
                            Callback::from(move |_| {
                                if is_active {
                                    active_index.set(None)
                                } else {
                                    active_index.set(Some(i))
                                }
                            })
                        };

                        let on_select_internal = {
                            let on_select = on_select.clone();
                            let active_index = active_index.clone();
                            Callback::from(move |value: AttrValue| {
                                on_select.emit(value.clone());
                                if auto_close {
                                    active_index.set(None);
                                }
                            })
                        };

                        html! {
                            <>
                                <Button
                                    button_type={ButtonType::Ghost}
                                    class="flex items-center gap-2 p-2 px-4 w-full hover:bg-gray-100 dark:hover:bg-gray-800"
                                    onclick={toggle.clone()}
                                >
                                    { btn.icon.clone() }
                                    {
                                        if is_active {
                                            match &btn.open_text {
                                                Html::VText(text) => html! { <Typo>{ text.text.clone() }</Typo> },
                                                other => other.clone(),
                                            }
                                        } else {
                                            html! {}
                                        }
                                    }
                                </Button>

                                {
                                    if is_active {
                                        html! {
                                            <div class="pl-4 pr-2 py-2">
                                                <NestedList list={btn.list.clone()} on_select={on_select_internal.clone()} />
                                            </div>
                                        }
                                    } else {
                                        html! {}
                                    }
                                }
                            </>
                        }
                    })
                }
            </div>
        </>
    }
}
