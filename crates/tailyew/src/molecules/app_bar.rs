use yew::{prelude::*, AttrValue};

#[derive(PartialEq, Clone, Default)]
pub enum AppBarPosition {
    Top,
    Bottom,
    #[default]
    DefaultPosition,
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

    html! {
        <nav class={format!("bg-white dark:bg-gray-900 shadow-lg transition-colors duration-300 {}", get_position_class(position))}>
            <div class="flex items-center justify-between p-4 w-full">
                // Logo or Title Section
                <div class="flex items-center space-x-4">
                    {
                        if let Some(url) = logo_url.clone() {
                            html! { <img src={url} class="h-8 w-8" alt="logo"/> }
                        } else {
                            html! {}
                        }
                    }

                    {
                        if let Some(title_text) = title.clone() {
                            html! { <span class="text-xl font-bold text-gray-900 dark:text-gray-100">{title_text}</span> }
                        } else {
                            html! {}
                        }
                    }
                </div>

                // Links Section (Hidden on small screens, visible on medium screens and above)
                <div class="hidden md:flex space-x-6 items-center">
                    { for links.iter().cloned() } // Use iter().cloned() to prevent moving the original `links` vector
                </div>

                // Responsive Menu Toggle Button (Visible on small screens)
                <button class="md:hidden text-gray-900 dark:text-gray-100" onclick={toggle_menu.clone()}>
                    { if *menu_open { "✖" } else { "☰" } }
                </button>

                // Action Buttons Section
                <div class="space-x-4 hidden md:flex items-center">
                    { for actions.iter().cloned() }
                </div>
            </div>

            // Responsive Menu Links
            { if *menu_open {
                html! {
                    <div class="md:hidden bg-gray-50 dark:bg-gray-800 text-gray-900 dark:text-gray-100 flex flex-col items-center space-y-4 p-4 w-full">
                        { for links.iter().cloned() }
                        <div class="space-x-4">
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

/// Determine the position class for the AppBar based on the `position` prop.
fn get_position_class(position: AppBarPosition) -> String {
    match position {
        AppBarPosition::Bottom => "fixed bottom-0 left-0 w-full".to_string(),
        AppBarPosition::Top => {
            "fixed top-0 left-0 w-full z-50 bg-white dark:bg-gray-900".to_string()
        }
        AppBarPosition::DefaultPosition => "w-full".to_string(),
    }
}
