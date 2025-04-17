use crate::atoms::{Button, ButtonType};
use crate::molecules::{Modal, ModalSize};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ModalButtonProps {
    pub button_text: String,
    #[prop_or_default]
    pub button_type: ButtonType,
    pub modal_title: String,
    #[prop_or_default]
    pub modal_content: Html,
    #[prop_or_default]
    pub is_open: bool,
    #[prop_or_default]
    pub on_modal_close: Option<Callback<()>>,

    /// A callback that receives the `close_modal` handler and returns a block of buttons.
    #[prop_or_default]
    pub footer: Option<Callback<Callback<()>, Html>>,

    #[prop_or(ModalSize::Large)]
    pub modal_size: ModalSize,
}

#[function_component(ModalButton)]
pub fn modal_button(props: &ModalButtonProps) -> Html {
    let modal_open = use_state(|| props.is_open);

    let toggle_modal = {
        let modal_open = modal_open.clone();
        Callback::from(move |_| modal_open.set(!*modal_open))
    };

    let close_modal = {
        let modal_open = modal_open.clone();
        let on_modal_close = props.on_modal_close.clone();
        Callback::from(move |_| {
            modal_open.set(false);
            if let Some(cb) = on_modal_close.clone() {
                cb.emit(());
            }
        })
    };

    // Generate footer content dynamically
    let footer_content = props.footer.as_ref().map(|cb| cb.emit(close_modal.clone()));

    html! {
        <div>
            <Button
                button_type={props.button_type.clone()}
                onclick={toggle_modal}
            >
                { &*props.button_text }
            </Button>

            <Modal
                title={props.modal_title.clone()}
                is_open={*modal_open}
                on_close={close_modal.clone()}
                size={props.modal_size}
            >
                <div class="space-y-6 text-sm text-gray-800 dark:text-gray-100">
                    { props.modal_content.clone() }

                    {
                        if let Some(buttons) = footer_content {
                            html! {
                                <div class="flex justify-end space-x-2 pt-2">
                                    { buttons }
                                </div>
                            }
                        } else {
                            html! {}
                        }
                    }
                </div>
            </Modal>
        </div>
    }
}
