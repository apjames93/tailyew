use crate::atoms::{Button, ButtonType};
use crate::molecules::{ModalButton, ModalSize, Notification, NotificationTypes};
use crate::Form;
use web_sys::SubmitEvent;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ModalButtonConfig {
    pub button_text: String,
    pub button_type: ButtonType,
    pub modal_title: String,
    pub modal_size: ModalSize,
    pub is_open: bool,
    pub on_modal_close: Option<Callback<()>>,
}

#[derive(Properties, PartialEq, Clone)]
pub struct FormModalProps {
    pub modal_button: ModalButtonConfig,
    pub children: Children,
    pub onsubmit: Callback<SubmitEvent>,

    #[prop_or_default]
    pub error_message: Option<String>,
    #[prop_or_default]
    pub success_message: Option<String>,

    #[prop_or("Submit".into())]
    pub submit_label: String,
    #[prop_or(false)]
    pub loading: bool,

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

    // 🔥 Move to internal state
    let internal_success_message = use_state(|| props.success_message.clone());
    let internal_error_message = use_state(|| props.error_message.clone());

    {
        let success_message = internal_success_message.clone();
        let auto_close_on_success = props.auto_close_on_success;
        let close_modal_ref = close_modal_ref.clone();
        let on_success = props.on_success.clone();

        use_effect_with(success_message.clone(), move |message| {
            if message.is_some() {
                if let Some(cb) = on_success.clone() {
                    cb.emit(());
                }
                if auto_close_on_success {
                    if let Some(close_cb) = (*close_modal_ref.borrow()).clone() {
                        close_cb.emit(());
                        // 🔥 Reset success message after closing
                        success_message.set(None);
                    }
                }
            }
            || ()
        });
    }

    {
        let error_message = internal_error_message.clone();
        let on_error = props.on_error.clone();

        use_effect_with(error_message.clone(), move |message| {
            if message.is_some() {
                if let Some(cb) = on_error.clone() {
                    cb.emit(());
                }
                // 🔥 Reset error message (optional)
                // error_message.set(None);
            }
            || ()
        });
    }

    let form_onsubmit = {
        let onsubmit = props.onsubmit.clone();
        let internal_success_message = internal_success_message.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            internal_success_message.set(Some("Form submitted successfully.".to_string()));
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
                button_text={props.modal_button.button_text.clone()}
                button_type={props.modal_button.button_type.clone()}
                modal_title={props.modal_button.modal_title.clone()}
                modal_size={props.modal_button.modal_size}
                is_open={props.modal_button.is_open}
                on_modal_close={props.modal_button.on_modal_close.clone()}
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
                            if let Some(error) = &*internal_error_message {
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
                            if let Some(success) = &*internal_success_message {
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
