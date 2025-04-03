use crate::atoms::{Button, ButtonType, TagType, Typo};
use crate::icons::XIcon;
use gloo::events::EventListener;
use gloo::utils::document;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use yew::prelude::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ModalSize {
    Small,
    Medium,
    Large,
}

#[derive(Properties, PartialEq, Clone)]
pub struct ModalProps {
    pub title: String,
    pub is_open: bool,
    pub on_close: Callback<()>,
    pub children: Children,
    #[prop_or(ModalSize::Large)]
    pub size: ModalSize,
}

#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    let ModalProps {
        title,
        is_open,
        on_close,
        children,
        size,
    } = props;

    let node_ref = use_node_ref();
    let on_close = Rc::new(on_close.clone());
    let children = Rc::new(children.clone());

    // Focus modal on open
    {
        let node_ref = node_ref.clone();
        let is_open = *is_open;
        use_effect(move || {
            if is_open {
                if let Some(node) = node_ref.cast::<web_sys::HtmlElement>() {
                    let _ = node.focus();
                }
            }
            || ()
        });
    }

    // Escape key closes modal
    {
        let on_close = on_close.clone();
        use_effect(move || {
            let listener = EventListener::new(&document(), "keydown", move |event| {
                if let Ok(event) = event.clone().dyn_into::<web_sys::KeyboardEvent>() {
                    if event.key() == "Escape" {
                        on_close.emit(());
                    }
                }
            });
            || drop(listener)
        });
    }

    if !*is_open {
        return html! {};
    }

    let on_overlay_click = {
        let on_close = on_close.clone();
        Callback::from(move |_: MouseEvent| on_close.emit(()))
    };

    let on_close_click = {
        let on_close = on_close.clone();
        Callback::from(move |_| on_close.emit(()))
    };

    let modal_size_class = match size {
        ModalSize::Small => "w-1/4",
        ModalSize::Medium => "w-1/2",
        ModalSize::Large => "w-3/4",
    };

    html! {
        <div
            class="fixed inset-0 bg-gray-800 bg-opacity-50 dark:bg-gray-900 dark:bg-opacity-70 flex justify-center items-center z-50"
            onclick={on_overlay_click}
            ref={node_ref}
            tabindex="-1"
        >
            <div
                class={format!(
                    "bg-white dark:bg-gray-800 {} p-6 rounded-lg shadow-xl transform transition-transform duration-300 ease-in-out scale-100 focus:outline-none max-h-[80vh] overflow-y-auto",
                    modal_size_class
                )}
                role="dialog"
                aria-labelledby="modal-title"
                aria-describedby="modal-description"
                tabindex="0"
                onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}
            >
                <div class="flex justify-between items-center border-b pb-4 border-gray-200 dark:border-gray-700 mb-4">
                    <Typo tag={TagType::H2}>{ title.clone() }</Typo>
                    <Button
                        onclick={on_close_click}
                        button_type={ButtonType::Icon}
                    >
                        <XIcon />
                    </Button>
                </div>

                <div class="mt-4 text-gray-700 dark:text-gray-300" id="modal-description">
                    { for (*children).iter() }
                </div>
            </div>
        </div>
    }
}
