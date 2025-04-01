use web_sys::window;
use yew::prelude::*;

use crate::atoms::Button;

#[function_component(ThemeToggle)]
pub fn theme_toggle() -> Html {
    // Try to get the theme from localStorage or default to "light"
    let initial_theme = {
        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                storage
                    .get_item("theme")
                    .unwrap_or(Some("light".to_string()))
                    .unwrap_or("light".to_string())
            } else {
                "light".to_string()
            }
        } else {
            "light".to_string()
        }
    };

    let theme = use_state(|| initial_theme);

    // Callback to toggle theme
    let toggle_theme = {
        let theme = theme.clone();
        Callback::from(move |_| {
            let new_theme = if *theme == "light" { "dark" } else { "light" };
            theme.set(new_theme.to_string());

            // Apply theme class to document
            if let Some(document) = window().and_then(|w| w.document()) {
                if let Some(html_element) = document.document_element() {
                    html_element
                        .set_attribute("class", new_theme)
                        .expect("Could not set class attribute");
                }
            }

            // Save to localStorage after state update
            if let Some(window) = window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    storage
                        .set_item("theme", new_theme)
                        .expect("Failed to save theme to local storage");
                }
            }
        })
    };

    // On component mount, apply the initial theme to the document
    {
        let theme = theme.clone();
        use_effect_with(theme, move |theme| {
            web_sys::console::log_1(&format!("theme: {:?}", (*theme)).into());

            if let Some(document) = window().and_then(|w| w.document()) {
                if let Some(html_element) = document.document_element() {
                    html_element
                        .set_attribute("class", theme)
                        .expect("Could not set initial theme class");
                }
            }
            || ()
        });
    }

    html! {
        <div>
            <Button onclick={toggle_theme}>
                { if *theme == "light" { "🌞 Light Mode" } else { "🌙 Dark Mode" } }
            </Button>
        </div>
    }
}
