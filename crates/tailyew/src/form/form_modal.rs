use crate::atoms::{Button, ButtonType};
use crate::molecules::{ModalButton, ModalSize, Notification, NotificationTypes};
use crate::Form;
use web_sys::SubmitEvent;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct FormModalProps {
    pub button_text: String,
    #[prop_or_default]
    pub button_type: ButtonType,
    pub modal_title: String,
    pub children: Children,
    pub onsubmit: Callback<SubmitEvent>,

    #[prop_or_default]
    pub is_open: bool,
    #[prop_or_default]
    pub on_modal_close: Option<Callback<()>>,

    #[prop_or_default]
    pub error_message: Option<String>,
    #[prop_or_default]
    pub success_message: Option<String>,

    #[prop_or("Submit".into())]
    pub submit_label: String,
    #[prop_or(false)]
    pub loading: bool,
    #[prop_or(ModalSize::Large)]
    pub modal_size: ModalSize,

    #[prop_or_default]
    pub extra_footer_buttons: Option<Callback<Callback<()>, Html>>,

    #[prop_or(true)]
    pub auto_close_on_success: bool,

    #[prop_or_default]
    pub on_success: Option<Callback<()>>,
    #[prop_or_default]
    pub on_error: Option<Callback<()>>,
}

#[function_component(FormModal)]
pub fn form_modal(props: &FormModalProps) -> Html {
    let close_modal_ref = use_mut_ref(|| None::<Callback<()>>);

    // Watch for success_message to auto-close modal and call on_success
    {
        let success_message = props.success_message.clone();
        let auto_close_on_success = props.auto_close_on_success;
        let close_modal_ref = close_modal_ref.clone();
        let on_success = props.on_success.clone();

        use_effect_with(success_message, move |message| {
            if message.is_some() {
                if let Some(cb) = on_success.clone() {
                    cb.emit(());
                }
                if auto_close_on_success {
                    if let Some(close_cb) = (*close_modal_ref.borrow()).clone() {
                        close_cb.emit(());
                    }
                }
            }
            || ()
        });
    }

    // Watch for error_message to trigger on_error (optional)
    {
        let error_message = props.error_message.clone();
        let on_error = props.on_error.clone();

        use_effect_with(error_message, move |message| {
            if message.is_some() {
                if let Some(cb) = on_error.clone() {
                    cb.emit(());
                }
            }
            || ()
        });
    }

    let form_onsubmit = {
        let onsubmit = props.onsubmit.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            onsubmit.emit(e);
        })
    };

    html! {
        <Form
            onsubmit_callback={form_onsubmit}
            show_submit_button={false}
            button_label={props.submit_label.clone()}
            loading={props.loading}
        >
            <ModalButton
                button_text={props.button_text.clone()}
                button_type={props.button_type.clone()}
                modal_title={props.modal_title.clone()}
                modal_size={props.modal_size}
                is_open={props.is_open}
                on_modal_close={props.on_modal_close.clone()}
                footer={Some({
                    let submit_label = props.submit_label.clone();
                    let extra_footer_buttons = props.extra_footer_buttons.clone();
                    let loading = props.loading;
                    let close_modal_ref = close_modal_ref.clone();

                    Callback::from(move |close_modal: Callback<()>| {
                        *close_modal_ref.borrow_mut() = Some(close_modal.clone());

                        html! {
                            <div class="flex justify-end space-x-2 pt-2">
                                {
                                    if let Some(cb) = &extra_footer_buttons {
                                        cb.emit(close_modal.clone())
                                    } else {
                                        html! {}
                                    }
                                }

                                <Button
                                    button_type={ButtonType::Submit}
                                    disabled={loading}
                                >
                                    { submit_label.clone() }
                                </Button>
                            </div>
                        }
                    })
                })}
                modal_content={html! {
                    <>
                        {
                            if let Some(error) = &props.error_message {
                                html! {
                                    <Notification
                                        message={error.clone()}
                                        notification_type={NotificationTypes::Error}
                                        visible={true}
                                        fixed={false}
                                    />
                                }
                            } else { html! {} }
                        }
                        {
                            if let Some(success) = &props.success_message {
                                html! {
                                    <Notification
                                        message={success.clone()}
                                        notification_type={NotificationTypes::Success}
                                        visible={true}
                                        fixed={false}
                                    />
                                }
                            } else { html! {} }
                        }

                        { for props.children.iter() }
                    </>
                }}
            />
        </Form>
    }
}
