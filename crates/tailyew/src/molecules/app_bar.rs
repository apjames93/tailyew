// crates/tailyew/src/molecules/app_bar.rs

use super::{AppBarPosition, NavBar};
use crate::atoms::{Button, ButtonType};
use crate::organisms::NestedItem;
use crate::organisms::NestedList;
use yew::{prelude::*, AttrValue};

#[derive(Properties, PartialEq, Clone)]
pub struct AppBarProps {
    #[prop_or_default]
    pub title: Option<AttrValue>,
    #[prop_or_default]
    pub logo_url: Option<AttrValue>,
    #[prop_or_default]
    pub nested_list: Vec<NestedItem>,
    #[prop_or_default]
    pub position: AppBarPosition,

    #[prop_or_default]
    pub logo_onclick: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub title_onclick: Option<Callback<MouseEvent>>,
}

#[function_component(AppBar)]
pub fn app_bar(props: &AppBarProps) -> Html {
    let AppBarProps {
        title,
        logo_url,
        nested_list,
        position,
        logo_onclick,
        title_onclick,
    } = props.clone();

    let drawer_open = use_state(|| false);

    // Button toggle
    let toggle_drawer = {
        let drawer_open = drawer_open.clone();
        Callback::from(move |_| drawer_open.set(!*drawer_open))
    };

    // Close drawer on overlay click (MouseEvent)
    let close_drawer_mouse = {
        let drawer_open = drawer_open.clone();
        Callback::from(move |_| drawer_open.set(false))
    };

    // Close drawer on NestedList select (AttrValue)
    let close_drawer_attr = {
        let drawer_open = drawer_open.clone();
        Callback::from(move |_value: AttrValue| drawer_open.set(false))
    };

    html! {
        <NavBar position={position}>
            <div class="flex items-center justify-between w-full">
                // Left: Logo
                { logo_url.map(|url| {
                    let maybe_logo_onclick = logo_onclick.clone();
                    html! {
                        <img
                            src={url}
                            class={classes!(
                                "h-8", "w-8",
                                maybe_logo_onclick.as_ref().map(|_| "cursor-pointer")
                            )}
                            alt="Logo"
                            onclick={maybe_logo_onclick.unwrap_or_else(|| Callback::from(|_| {}))} // no-op if none
                        />
                    }
                }) }

                // Title
                { title.map(|text| {
                    let maybe_title_onclick = title_onclick.clone();
                    html! {
                        <div
                            class={classes!(
                                "absolute", "left-1/2", "transform", "-translate-x-1/2",
                                maybe_title_onclick.as_ref().map(|_| "cursor-pointer")
                            )}
                            onclick={maybe_title_onclick.unwrap_or_else(|| Callback::from(|_| {}))} // no-op if none
                        >
                            <span class="text-xl font-bold text-gray-900 dark:text-gray-100">
                                { text }
                            </span>
                        </div>
                    }
                }) }

                // Right: Drawer toggle button
                <div class="flex items-center">
                    <Button button_type={ButtonType::Icon} onclick={toggle_drawer} class="p-2">
                        { if *drawer_open {
                            html! { <span class="text-2xl">{ "✖" }</span> }
                        } else {
                            html! { <span class="text-2xl">{ "☰" }</span> } // hamburger
                        }}
                    </Button>
                </div>
            </div>

            // Mobile Drawer
            {
                if *drawer_open {
                    html! {
                        <div>
                            // Overlay
                            <div
                                class="fixed inset-0 bg-black bg-opacity-40 z-30"
                                onclick={close_drawer_mouse.clone()}
                            />

                            // Drawer content
                            <div class="fixed top-0 right-0 w-64 h-full bg-white dark:bg-gray-900 shadow-lg z-40 p-4">
                                <NestedList list={nested_list.clone()} on_select={close_drawer_attr} />
                            </div>
                        </div>
                    }
                } else {
                    html! {}
                }
            }
        </NavBar>
    }
}
