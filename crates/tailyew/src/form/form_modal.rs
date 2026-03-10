use crate::atoms::{Button, ButtonType};
use crate::form::{Form, FormSubmitCallback, FormSubmitFuture};
use crate::molecules::{ModalButton, ModalSize};
use js_sys::Date;
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
    pub onsubmit: FormSubmitCallback,

    #[prop_or("Submit".into())]
    pub submit_label: String,

    #[prop_or_default]
    pub extra_footer_buttons: Option<Callback<Callback<()>, Html>>,

    #[prop_or(true)]
    pub auto_close_on_success: bool,

    #[prop_or_default]
    pub on_success: Option<Callback<()>>,

    #[prop_or_default]
    pub on_error: Option<Callback<()>>,

    #[prop_or_default]
    pub disabled: bool,
}

#[component(FormModal)]
pub fn form_modal(props: &FormModalProps) -> Html {
    let form_id = use_state(|| Date::now().to_string());
    let loading = use_state(|| false);
    let close_cb = use_mut_ref(|| None::<Callback<()>>);

    // Footer with submit + optional extra buttons
    let footer = {
        let submit_lbl = props.submit_label.clone();
        let extra = props.extra_footer_buttons.clone();
        let close_cb = close_cb.clone();
        let disabled = props.disabled;
        let form_id = (*form_id).clone();
        let loading = loading.clone();

        Callback::from(move |set_modal_close: Callback<()>| {
            *close_cb.borrow_mut() = Some(set_modal_close.clone());
            let is_loading = *loading;
            html! {
                <div class="flex justify-end space-x-2 pt-2">
                    {
                        if let Some(cb) = &extra {
                            cb.emit(set_modal_close.clone())
                        } else {
                            html! {}
                        }
                    }
                    <Button
                        button_type={ButtonType::Submit}
                        form={Some(form_id.clone())}
                        disabled={is_loading || disabled}
                    >
                        { html! { submit_lbl.clone() } }
                    </Button>
                </div>
            }
        })
    };

    let wrapped_onsubmit: FormSubmitCallback = {
        let original = props.onsubmit.clone();
        let loading = loading.clone();
        let on_success = props.on_success.clone();
        let on_error = props.on_error.clone();
        let auto_close = props.auto_close_on_success;
        let close_cb = close_cb.clone();

        Callback::from(move |e: SubmitEvent| {
            loading.set(true);
            let fut: FormSubmitFuture = original.emit(e);
            let loading = loading.clone();
            let on_success = on_success.clone();
            let on_error = on_error.clone();
            let close_cb = close_cb.clone();

            let wrapped: FormSubmitFuture = Box::pin(async move {
                let outcome = fut.await;
                loading.set(false);

                match &outcome {
                    Ok(_) => {
                        if let Some(cb) = &on_success {
                            cb.emit(());
                        }
                        if auto_close {
                            if let Some(close) = &*close_cb.borrow() {
                                close.emit(());
                            }
                        }
                    }
                    Err(_) => {
                        if let Some(cb) = &on_error {
                            cb.emit(());
                        }
                    }
                }

                outcome
            });

            wrapped
        })
    };

    html! {
        <ModalButton
            trigger_children={html! { props.modal_button.button_text.clone() }}
            button_type={props.modal_button.button_type.clone()}
            modal_title={props.modal_button.modal_title.clone()}
            modal_size={props.modal_button.modal_size}
            is_open={props.modal_button.is_open}
            on_modal_close={props.modal_button.on_modal_close.clone()}
            footer={Some(footer)}
            modal_content={html! {
                <Form
                    id={Some((*form_id).clone())}
                    onsubmit_callback={wrapped_onsubmit}
                    show_submit_button={false}
                    disabled={props.disabled}
                >
                    { for props.children.iter() }
                </Form>
            }}
        />
    }
}
