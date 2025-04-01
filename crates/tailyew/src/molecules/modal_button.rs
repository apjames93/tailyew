// warp-yew/frontend/src/templates/modal_button.rs

use crate::atoms::{Button, ButtonType};
use crate::molecules::Modal;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ModalButtonProps {
    pub button_text: String,
    pub modal_title: String,
    pub modal_content: Html,
    #[prop_or_default]
    pub on_modal_close: Option<Callback<()>>,
    #[prop_or_default]
    pub is_open: bool,
}

#[function_component(ModalButton)]
pub fn modal_button(props: &ModalButtonProps) -> Html {
    let modal_open = use_state(|| props.is_open);

    let toggle_modal = {
        let modal_open = modal_open.clone();
        Callback::from(move |_| {
            modal_open.set(!*modal_open);
        })
    };

    let close_modal = {
        let modal_open = modal_open.clone();
        let on_modal_close = props.on_modal_close.clone();
        Callback::from(move |_| {
            modal_open.set(false);
            if let Some(callback) = on_modal_close.clone() {
                callback.emit(());
            }
        })
    };

    html! {
        <div>
            <Button
                button_type={ButtonType::Primary}
                onclick={toggle_modal.clone()}
            >
                { html! { &props.button_text } }
            </Button>

            <Modal
                title={props.modal_title.clone()}
                is_open={*modal_open}
                on_close={close_modal} // Use close_modal to handle the close behavior
            >
                { props.modal_content.clone() }
            </Modal>
        </div>
    }
}
