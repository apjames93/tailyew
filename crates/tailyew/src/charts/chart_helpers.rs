use js_sys::Array;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{window, CanvasRenderingContext2d, MutationObserver, MutationObserverInit};
use yew::prelude::*;

/// Centralized theme-based styles for canvas drawing
pub struct ThemeStyles {
    pub stroke: &'static str,
    pub text: &'static str,
}

pub fn get_theme_styles(theme: &str) -> ThemeStyles {
    match theme {
        "dark" => ThemeStyles {
            stroke: "#fff",
            text: "#fff",
        },
        _ => ThemeStyles {
            stroke: "#888",
            text: "#000",
        },
    }
}

/// Applies stroke and text styles based on the provided ThemeStyles
pub fn apply_theme_styles(ctx: &CanvasRenderingContext2d, styles: &ThemeStyles) {
    ctx.set_stroke_style_str(styles.stroke);
    ctx.set_fill_style_str(styles.text);
    ctx.set_font("12px sans-serif");
}

/// Reactive hook to watch `<html class="dark">` and return `"light"` or `"dark"`
#[hook]
pub fn use_get_chart_theme() -> UseStateHandle<String> {
    let theme = use_state(|| {
        window()
            .and_then(|w| w.document())
            .and_then(|d| d.document_element())
            .map(|el| {
                if el.class_name().contains("dark") {
                    "dark".to_string()
                } else {
                    "light".to_string()
                }
            })
            .unwrap_or_else(|| "light".to_string())
    });

    let theme_clone = theme.clone();

    use_effect(move || {
        let document = window().unwrap().document().unwrap();
        let element = document.document_element().unwrap();
        let element_for_closure = element.clone();

        let callback = Closure::wrap(Box::new(
            move |_records: Array, _observer: MutationObserver| {
                let is_dark = element_for_closure.class_name().contains("dark");
                theme_clone.set(if is_dark {
                    "dark".to_string()
                } else {
                    "light".to_string()
                });
            },
        ) as Box<dyn FnMut(_, _)>);

        let config = MutationObserverInit::new();
        config.set_attributes(true);

        let observer = MutationObserver::new(callback.as_ref().unchecked_ref())
            .expect("failed to create MutationObserver");

        observer
            .observe_with_options(&element, &config)
            .expect("failed to observe <html>");

        callback.forget();
        || ()
    });

    theme
}
