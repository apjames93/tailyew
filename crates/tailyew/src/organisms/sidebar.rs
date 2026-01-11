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
    // e.g. "top-16" to clear your app bar
    #[prop_or_default]
    pub top_offset_class: Classes,
    #[prop_or_default]
    pub position: SidebarPosition,
}

#[component(Sidebar)]
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

    // build the position-specific classes first
    let (position_classes, height_classes) = if matches!(position, SidebarPosition::Static) {
        // static mode – let parent control height
        (classes!("relative"), classes!("h-full"))
    } else {
        // fixed under navbar to bottom
        (
            classes!("fixed", "z-40", top_offset_class.clone(), "bottom-0"),
            classes!(), // no extra height classes
        )
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
                    position_classes,
                    height_classes,
                    side_class,
                    // look & feel
                    "bg-white",
                    "dark:bg-gray-900",
                    "border-r",
                    "border-gray-200",
                    "dark:border-gray-700",
                    "transition-all",
                    "duration-300",
                    // structure
                    "flex",
                    "flex-col",
                    "items-stretch",
                    // scroll
                    "overflow-y-auto",
                    // width based on active state
                    if active_index.is_some() { "w-64" } else { "w-14" },
                    // make sure last item is reachable
                    "pb-16",
                )}
            >
                {
                    for icon_list.iter().enumerate().map(|(i, btn)| {
                        let is_active = *active_index == Some(i);
                        let panel_id = AttrValue::from(format!("sidebar-panel-{i}"));

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
                                    aria_expanded={Some(AttrValue::from(is_active.to_string()))}
                                    aria_controls={Some(panel_id.clone())}
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
                                            <div id={panel_id} class="pl-4 pr-2 py-2">
                                                <NestedList
                                                    list={btn.list.clone()}
                                                    on_select={on_select_internal.clone()}
                                                />
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
