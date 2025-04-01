// src/molecules/popover.rs

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::MouseEvent;
use web_sys::Node;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct PopoverProps {
    pub trigger: Html, // The element that will trigger the popover when clicked
    pub content: Html, // The content that will be shown inside the popover
    #[prop_or(false)]
    pub is_open: bool, // Whether the popover is open or closed
    #[prop_or_default]
    pub on_close: Option<Callback<MouseEvent>>, // Callback for handling the close action
}

#[function_component(Popover)]
pub fn popover(props: &PopoverProps) -> Html {
    let PopoverProps {
        trigger,
        content,
        is_open,
        on_close,
    } = props.clone();
    let popover_open = use_state(|| is_open);

    let toggle_popover = {
        let popover_open = popover_open.clone();
        Callback::from(move |_| {
            popover_open.set(!*popover_open);
        })
    };

    let close_popover = {
        let popover_open = popover_open.clone();
        let on_close = on_close.clone();
        Callback::from(move |event: MouseEvent| {
            popover_open.set(false);
            if let Some(on_close) = on_close.clone() {
                on_close.emit(event);
            }
        })
    };

    let popover_ref = use_node_ref();

    {
        let popover_ref = popover_ref.clone();
        let popover_open = popover_open.clone();

        use_effect_with((), move |()| {
            let closure = Closure::<dyn Fn(MouseEvent)>::new(move |event: MouseEvent| {
                if let Some(popover) = popover_ref.cast::<web_sys::HtmlElement>() {
                    let event_target: Node = event.target().unwrap().dyn_into().unwrap();

                    if !popover.contains(Some(&event_target)) {
                        popover_open.set(false);
                    }
                }
            });

            web_sys::window()
                .unwrap()
                .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
                .unwrap();

            move || {
                web_sys::window()
                    .unwrap()
                    .remove_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
                    .unwrap();
            }
        });
    }

    html! {
        <div ref={popover_ref} class="relative">
            // Trigger Element
            <div class="inline-block cursor-pointer" onclick={toggle_popover}>
                { trigger.clone() }
            </div>

            // Popover Content
            { if *popover_open {
                html! {
                    <div
                        class={classes!(
                            "absolute", "mt-2", "w-64", "md:w-80", "lg:w-96", "p-4", "z-10", "rounded", "shadow-lg",
                            "border", "dark:border-gray-700", "bg-white", "dark:bg-gray-800",
                            "transition-all", "duration-200", "ease-in-out", "transform", "opacity-100", "scale-100",
                        )}
                        style="top: 100%; left: 50%; transform: translate(-50%, 0);"
                    >
                        <div class="relative">
                            { content.clone() }

                            // Close Button
                            <button
                                class="absolute top-1 right-1 text-gray-500 dark:text-gray-300 hover:text-gray-700 dark:hover:text-gray-500 transition duration-150"
                                onclick={close_popover.clone()}
                                aria-label="Close Popover"
                            >
                                { "✕" }
                            </button>
                        </div>
                    </div>
                }
            } else {
                html! { <></> }
            }}
        </div>
    }
}
