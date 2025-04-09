use crate::atoms::{Button, ButtonType};
use crate::molecules::Modal;
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

    #[prop_or_default]
    pub on_confirm_click: Option<Callback<()>>,
    #[prop_or("Confirm".into())]
    pub confirm_button_text: String,
    #[prop_or(ButtonType::Primary)]
    pub confirm_button_type: ButtonType,
    #[prop_or(false)]
    pub confirm_disabled: bool,
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

    let confirm = {
        let modal_open = modal_open.clone();
        let on_modal_close = props.on_modal_close.clone();
        let on_confirm_click = props.on_confirm_click.clone();

        Callback::from(move |_| {
            if let Some(confirm_handler) = on_confirm_click.clone() {
                confirm_handler.emit(());
            }
            modal_open.set(false);
            if let Some(close_handler) = on_modal_close.clone() {
                close_handler.emit(());
            }
        })
    };

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
                on_close={close_modal}
            >
                <div class="space-y-6 text-sm text-gray-800 dark:text-gray-100">
                    { props.modal_content.clone() }

                    {
                        if props.on_confirm_click.is_some() {
                            html! {
                                <div class="flex justify-end space-x-2 pt-2">
                                    <Button
                                        button_type={props.confirm_button_type.clone()}
                                        onclick={confirm}
                                        disabled={props.confirm_disabled}
                                    >
                                        { &*props.confirm_button_text }
                                    </Button>
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
