use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{MouseEvent, Node};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct PopoverProps {
    pub trigger: Html,
    pub content: Html,
    #[prop_or(false)]
    pub is_open: bool,
    #[prop_or_default]
    pub on_close: Option<Callback<MouseEvent>>,
}

#[function_component(Popover)]
pub fn popover(props: &PopoverProps) -> Html {
    let PopoverProps {
        trigger,
        content,
        is_open,
        on_close,
    } = props;

    let popover_ref = use_node_ref();
    let open = use_state(|| *is_open);

    // Sync internal state with prop if changed
    if *open != *is_open {
        open.set(*is_open);
    }

    let on_close = on_close.clone();

    let toggle = {
        let open = open.clone();
        Callback::from(move |_| open.set(!*open))
    };

    let close = {
        let open = open.clone();
        let on_close = on_close.clone();
        Callback::from(move |event: MouseEvent| {
            open.set(false);
            if let Some(cb) = &on_close {
                cb.emit(event);
            }
        })
    };

    {
        let popover_ref = popover_ref.clone();
        let open = open.clone();

        use_effect(move || {
            let closure =
                Closure::<dyn Fn(MouseEvent)>::wrap(Box::new(move |event: MouseEvent| {
                    if let Some(popover) = popover_ref.cast::<web_sys::HtmlElement>() {
                        if let Some(target) = event.target().and_then(|t| t.dyn_into::<Node>().ok())
                        {
                            if !popover.contains(Some(&target)) {
                                open.set(false);
                            }
                        }
                    }
                }));

            let window = web_sys::window().unwrap();
            window
                .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
                .unwrap();

            // Keep the closure alive
            let _closure = closure;
            move || {
                window
                    .remove_event_listener_with_callback("click", _closure.as_ref().unchecked_ref())
                    .unwrap();
            }
        });
    }

    html! {
        <div ref={popover_ref} class="relative inline-block">
            <div class="cursor-pointer" onclick={toggle}>
                { trigger.clone() }
            </div>

            if *open {
                <div
                    class={classes!(
                        "absolute", "z-50", "w-64", "md:w-80", "lg:w-96", "mt-2",
                        "p-4", "rounded", "shadow-xl", "border", "bg-white", "dark:bg-gray-800",
                        "dark:border-gray-700", "transition", "duration-200", "ease-in-out"
                    )}
                    style="top: 100%; left: 50%; transform: translate(-50%, 0);"
                    role="dialog"
                    aria-modal="true"
                >
                    <div class="relative">
                        { content.clone() }
                        <button
                            class="absolute top-1 right-1 text-gray-500 dark:text-gray-300 hover:text-gray-700 dark:hover:text-gray-500"
                            onclick={close}
                            aria-label="Close Popover"
                        >
                            { "✕" }
                        </button>
                    </div>
                </div>
            }
        </div>
    }
}
