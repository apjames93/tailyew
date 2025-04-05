// crates/tailyew/src/molecules/app_bar.rs

use yew::{prelude::*, AttrValue};

use super::{AppBarPosition, NavBar};

#[derive(Properties, PartialEq, Clone)]
pub struct AppBarProps {
    #[prop_or_default]
    pub title: Option<AttrValue>,
    #[prop_or_default]
    pub logo_url: Option<AttrValue>,
    #[prop_or_default]
    pub links: Vec<Html>,
    #[prop_or_default]
    pub actions: Vec<Html>,
    #[prop_or_default]
    pub position: AppBarPosition,
}

#[function_component(AppBar)]
pub fn app_bar(props: &AppBarProps) -> Html {
    let AppBarProps {
        title,
        logo_url,
        links,
        actions,
        position,
    } = props.clone();

    let menu_open = use_state(|| false);

    let toggle_menu = {
        let menu_open = menu_open.clone();
        Callback::from(move |_| menu_open.set(!*menu_open))
    };

    html! {
        <NavBar position={position}>
            <div class="flex items-center justify-between w-full">
                // Left: Logo + Title
                <div class="flex items-center space-x-3">
                    { logo_url.map(|url| html! {
                        <img src={url} class="h-8 w-8" alt="Logo" />
                    }) }
                    { title.map(|text| html! {
                        <span class="text-xl font-bold text-gray-900 dark:text-gray-100">{ text }</span>
                    }) }
                </div>

                // Desktop links + actions
                <div class="hidden md:flex items-center space-x-6">
                    { for links.iter().cloned() }
                    { for actions.iter().cloned() }
                </div>

                // Hamburger menu toggle (mobile)
                <button
                    class="md:hidden text-gray-900 dark:text-gray-100"
                    onclick={toggle_menu.clone()}
                    aria-expanded={menu_open.to_string()}
                    aria-label="Toggle menu"
                >
                    { if *menu_open { "✖" } else { "☰" } }
                </button>
            </div>

            // Mobile Menu
            {
                if *menu_open {
                    html! {
                        <div class="md:hidden mt-4 space-y-4 bg-gray-100 dark:bg-gray-800 text-gray-900 dark:text-gray-100 rounded-lg px-4 py-3">
                            <div class="flex flex-col space-y-2">
                                { for links.iter().cloned() }
                            </div>
                            <div class="flex flex-wrap gap-2">
                                { for actions.iter().cloned() }
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
