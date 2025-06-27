use crate::atoms::{Button, ButtonType};
use crate::molecules::{Modal, ModalSize};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ModalButtonProps {
    /// Content to render inside the trigger button (e.g., text or icons)
    #[prop_or_default]
    pub trigger_children: Children,

    /// Style for the trigger button
    #[prop_or(ButtonType::Primary)]
    pub button_type: ButtonType,

    /// Title for the modal dialog
    pub modal_title: String,

    /// Content inside the modal body
    #[prop_or_default]
    pub modal_content: Html,

    /// Controls initial open state of the modal
    #[prop_or_default]
    pub is_open: bool,

    /// Callback when the modal is closed
    #[prop_or_default]
    pub on_modal_close: Option<Callback<()>>,

    /// Footer content generator (receives a close callback)
    #[prop_or_default]
    pub footer: Option<Callback<Callback<()>, Html>>,

    /// Size of the modal (e.g., Large, Medium, Small)
    #[prop_or(ModalSize::Large)]
    pub modal_size: ModalSize,

    /// ARIA label (for screen readers if no heading)
    #[prop_or_default]
    pub aria_label: Option<String>,

    /// ID of the heading element for aria-labelledby
    #[prop_or_default]
    pub aria_labelledby: Option<String>,
}

#[function_component(ModalButton)]
pub fn modal_button(props: &ModalButtonProps) -> Html {
    let ModalButtonProps {
        trigger_children,
        button_type,
        modal_title,
        modal_content,
        is_open,
        on_modal_close,
        footer,
        modal_size,
        aria_label,
        aria_labelledby,
    } = props;

    let modal_open = use_state(|| *is_open);

    let toggle_modal = {
        let modal_open = modal_open.clone();
        Callback::from(move |_| modal_open.set(!*modal_open))
    };

    let close_modal = {
        let modal_open = modal_open.clone();
        let on_modal_close = on_modal_close.clone();
        Callback::from(move |_| {
            modal_open.set(false);
            if let Some(cb) = on_modal_close.clone() {
                cb.emit(());
            }
        })
    };

    let footer_content = footer.as_ref().map(|cb| cb.emit(close_modal.clone()));

    html! {
        <div>
            <Button
                button_type={button_type.clone()}
                onclick={toggle_modal.clone()}
            >
                { for trigger_children.iter() }
            </Button>

            <Modal
                title={modal_title.clone()}
                is_open={*modal_open}
                on_close={close_modal.clone()}
                size={*modal_size}
                aria_label={aria_label.clone()}
                aria_labelledby={aria_labelledby.clone()}
            >
                <div class="space-y-6 text-sm text-gray-800 dark:text-gray-100">
                    { modal_content.clone() }

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
