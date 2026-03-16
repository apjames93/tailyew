use js_sys::Array;
use wasm_bindgen::{JsCast, closure::Closure};
use web_sys::{CanvasRenderingContext2d, MutationObserver, MutationObserverInit, window};
use web_sys::{Element, ResizeObserver, ResizeObserverEntry};
use yew::prelude::*;

/// Track the rendered width of an element using ResizeObserver.
#[hook]
pub fn use_container_width(container_ref: &NodeRef) -> f64 {
    let width = use_state(|| 0.0_f64);

    {
        let container_ref = container_ref.clone();
        let width = width.clone();

        // Attach effect with the ref as a dependency so it only runs
        // when the actual DOM node behind the ref changes.
        use_effect_with(container_ref, move |container_ref| {
            // We'll optionally store an observer and clean it up later.
            let mut observer_opt: Option<ResizeObserver> = None;

            if let Some(element) = container_ref.cast::<Element>() {
                let width_state = width.clone();

                // JS callback: entries[0].contentRect.width
                let callback = Closure::<dyn FnMut(js_sys::Array, ResizeObserver)>::wrap(Box::new(
                    move |entries: js_sys::Array, _observer: ResizeObserver| {
                        if let Ok(entry) = entries.get(0).dyn_into::<ResizeObserverEntry>() {
                            let rect = entry.content_rect();
                            let new_width = rect.width();

                            let current = *width_state;
                            // Only update if changed by at least 1px to avoid thrash
                            if (new_width - current).abs() >= 1.0 {
                                width_state.set(new_width);
                            }
                        }
                    },
                ));

                let observer = ResizeObserver::new(callback.as_ref().unchecked_ref())
                    .expect("create observer");

                observer.observe(&element);

                // Keep the closure alive for the life of this effect.
                callback.forget();

                observer_opt = Some(observer);
            }

            // Single cleanup closure, captures `observer_opt` (which may be None).
            move || {
                if let Some(observer) = observer_opt {
                    observer.disconnect();
                }
            }
        });
    }

    *width
}

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

    use_effect_with((), move |_| {
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

        move || observer.disconnect()
    });

    theme
}
