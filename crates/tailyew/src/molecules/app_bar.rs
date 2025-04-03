use yew::{prelude::*, AttrValue};

#[derive(PartialEq, Clone, Default)]
pub enum AppBarPosition {
    #[default]
    Static,
    Top,
    Bottom,
}

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

    let nav_classes = classes!(
        "bg-white",
        "dark:bg-gray-900",
        "shadow-md",
        "transition-colors",
        "duration-300",
        get_position_class(position)
    );

    html! {
        <nav class={nav_classes}>
            <div class="flex items-center justify-between px-4 py-3 w-full max-w-7xl mx-auto">
                // Left: Logo + Title
                <div class="flex items-center space-x-3">
                    { logo_url.map(|url| html! {
                        <img src={url} class="h-8 w-8" alt="Logo" />
                    }) }
                    { title.map(|text| html! {
                        <span class="text-xl font-bold text-gray-900 dark:text-gray-100">{ text }</span>
                    }) }
                </div>

                // Middle: Navigation links (Desktop only)
                <div class="hidden md:flex items-center space-x-6">
                    { for links.iter().cloned() }
                </div>

                // Right: Action buttons (Desktop only)
                <div class="hidden md:flex items-center space-x-4">
                    { for actions.iter().cloned() }
                </div>

                // Hamburger (Mobile only)
                <button
                    class="md:hidden text-gray-900 dark:text-gray-100"
                    onclick={toggle_menu.clone()}
                    aria-expanded={menu_open.to_string()}
                    aria-label="Toggle menu"
                >
                    { if *menu_open { "✖" } else { "☰" } }
                </button>
            </div>

            // Mobile menu
            { if *menu_open {
                html! {
                    <div class="md:hidden px-4 py-3 space-y-4 bg-gray-100 dark:bg-gray-800 text-gray-900 dark:text-gray-100">
                        <div class="flex flex-col space-y-2 items-start">
                            { for links.iter().cloned() }
                        </div>
                        <div class="flex space-x-4">
                            { for actions.iter().cloned() }
                        </div>
                    </div>
                }
            } else {
                html! {}
            }}
        </nav>
    }
}

/// Convert position to Tailwind classes
fn get_position_class(position: AppBarPosition) -> &'static str {
    match position {
        AppBarPosition::Top => "fixed top-0 left-0 w-full z-50",
        AppBarPosition::Bottom => "fixed bottom-0 left-0 w-full z-50",
        AppBarPosition::Static => "w-full",
    }
}
