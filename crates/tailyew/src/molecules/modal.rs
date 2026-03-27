use crate::atoms::{Button, ButtonType, TagType, Typo};
use crate::icons::XIcon;
use gloo::events::EventListener;
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, KeyboardEvent};
use yew::{create_portal, prelude::*};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ModalSize {
    Small,
    Medium,
    Large,
    Fullscreen,
}

#[derive(Properties, PartialEq, Clone)]
pub struct ModalProps {
    pub title: String,
    pub is_open: bool,
    pub on_close: Callback<()>,
    pub children: Children,
    #[prop_or(ModalSize::Large)]
    pub size: ModalSize,
    #[prop_or_default]
    pub aria_label: Option<AttrValue>,
    #[prop_or_default]
    pub aria_labelledby: Option<AttrValue>,
}

fn modal_size_class(size: ModalSize) -> &'static str {
    match size {
        ModalSize::Small => "w-1/4",
        ModalSize::Medium => "w-1/2",
        ModalSize::Large => "w-3/4",
        ModalSize::Fullscreen => "w-screen h-screen p-0 rounded-none",
    }
}

fn body_max_height_class(size: ModalSize) -> &'static str {
    match size {
        ModalSize::Fullscreen => "max-h-[calc(100vh-80px)]",
        _ => "max-h-[70vh]",
    }
}

fn modal_scroll_class(size: ModalSize) -> &'static str {
    match size {
        ModalSize::Fullscreen => "h-screen",
        _ => "shadow-xl max-h-[80vh]",
    }
}

fn modal_padding_class(size: ModalSize) -> &'static str {
    if size == ModalSize::Fullscreen {
        "p-2"
    } else {
        "p-6 rounded-lg"
    }
}

fn document_body_host() -> Option<Element> {
    web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.body())
        .map(|body| body.unchecked_into::<Element>())
}

#[component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    let ModalProps {
        title,
        is_open,
        on_close,
        children,
        size,
        aria_label,
        aria_labelledby,
    } = props;

    let dialog_ref = use_node_ref();
    let portal_host = use_state(|| None::<Element>);
    let title_id = use_state(|| AttrValue::from(format!("modal-title-{}", Uuid::new_v4())));
    let description_id =
        use_state(|| AttrValue::from(format!("modal-description-{}", Uuid::new_v4())));

    // Resolve the portal host after mount so hydration/SSR paths do not try to create a portal.
    {
        let portal_host = portal_host.clone();
        let has_portal_host = (*portal_host).is_some();

        use_effect_with(
            (*is_open, has_portal_host),
            move |(is_open, has_portal_host)| {
                if *is_open
                    && !*has_portal_host
                    && let Some(host) = document_body_host()
                {
                    portal_host.set(Some(host));
                }

                || ()
            },
        );
    }

    // Focus the dialog when it becomes visible.
    {
        let dialog_ref = dialog_ref.clone();
        let has_portal_host = (*portal_host).is_some();

        use_effect_with(
            (*is_open, has_portal_host),
            move |(is_open, has_portal_host)| {
                if *is_open
                    && *has_portal_host
                    && let Some(node) = dialog_ref.cast::<HtmlElement>()
                {
                    let _ = node.focus();
                }

                || ()
            },
        );
    }

    // Escape key closes modal.
    {
        let on_close = on_close.clone();
        use_effect_with(*is_open, move |is_open| {
            let listener = if *is_open {
                web_sys::window()
                    .and_then(|window| window.document())
                    .map(|document| {
                        EventListener::new(&document, "keydown", move |event| {
                            if let Some(event) = event.dyn_ref::<KeyboardEvent>()
                                && event.key() == "Escape"
                            {
                                on_close.emit(());
                            }
                        })
                    })
            } else {
                None
            };

            move || {
                drop(listener);
            }
        });
    }

    if !*is_open {
        return html! {};
    }

    let Some(portal_host) = (*portal_host).clone() else {
        return html! {};
    };

    let on_overlay_click = {
        let on_close = on_close.clone();
        Callback::from(move |_: MouseEvent| on_close.emit(()))
    };

    let on_close_click = {
        let on_close = on_close.clone();
        Callback::from(move |_| on_close.emit(()))
    };

    let resolved_aria_labelledby = aria_labelledby
        .clone()
        .or_else(|| aria_label.is_none().then(|| (*title_id).clone()));

    let modal_content = html! {
        <div
            class="fixed inset-0 bg-gray-800 bg-opacity-50 dark:bg-gray-900 dark:bg-opacity-70 flex justify-center items-center z-50"
            onclick={on_overlay_click}
        >
            <div
                class={format!(
                    "bg-white dark:bg-gray-800 flex flex-col {} {} transform transition-transform duration-300 ease-in-out scale-100 focus:outline-none {}",
                    modal_size_class(*size),
                    modal_scroll_class(*size),
                    modal_padding_class(*size)
                )}
                ref={dialog_ref}
                role="dialog"
                aria-modal="true"
                aria-label={aria_label.clone()}
                aria-labelledby={resolved_aria_labelledby}
                aria-describedby={Some((*description_id).clone())}
                tabindex="-1"
                onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}
            >
                // -- STICKY HEADER --
                <div class="flex justify-between items-center border-b pb-4 border-gray-200 dark:border-gray-700 mb-4 sticky top-0 z-10 bg-white dark:bg-gray-800">
                    <Typo id={Some((*title_id).clone())} tag={TagType::H2} class="text-lg">
                        { html! { title.clone() } }
                    </Typo>
                    <Button
                        on_click={on_close_click}
                        button_type={ButtonType::Icon}
                    >
                        <XIcon />
                    </Button>
                </div>

                // -- SCROLLABLE BODY --
                <div
                    class={format!(
                        "mt-4 text-gray-700 dark:text-gray-300 overflow-y-auto {}",
                        body_max_height_class(*size)
                    )}
                    id={(*description_id).clone()}
                >
                    { for children.iter() }
                </div>
            </div>
        </div>
    };

    create_portal(modal_content, portal_host)
}
