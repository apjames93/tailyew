use crate::atoms::{Button, ButtonType};
use crate::molecules::{ModalButton, ModalSize};
use crate::Form;
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
    /// your submit handler
    pub onsubmit: Callback<SubmitEvent>,

    /// banner‐level form error
    #[prop_or_default]
    pub error_message: Option<String>,
    /// banner‐level form success
    #[prop_or_default]
    pub success_message: Option<String>,

    /// text for the submit button
    #[prop_or("Submit".into())]
    pub submit_label: String,

    /// any extra footer buttons you want
    #[prop_or_default]
    pub extra_footer_buttons: Option<Callback<Callback<()>, Html>>,

    /// auto-close modal whenever we get a new `success_message`
    #[prop_or(true)]
    pub auto_close_on_success: bool,
    /// hook to fire on each new success
    #[prop_or_default]
    pub on_success: Option<Callback<()>>,
    /// hook to fire on each new error
    #[prop_or_default]
    pub on_error: Option<Callback<()>>,
}

#[function_component(FormModal)]
pub fn form_modal(props: &FormModalProps) -> Html {
    // 1) Unique form ID so our Button can target it
    let form_id = use_state(|| Date::now().to_string());

    // 2) Internal loading state
    let loading = use_state(|| false);

    // 3) Store the modal-close callback for auto-close
    let close_cb = use_mut_ref(|| None::<Callback<()>>);

    // 4) Wrap the user’s onsubmit to flip loading → true
    let internal_submit = {
        let onsubmit = props.onsubmit.clone();
        let loading = loading.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            loading.set(true);
            onsubmit.emit(e);
        })
    };

    // 5) Fire on_success + auto-close when success_message arrives (and clear loading)
    {
        let close_cb = close_cb.clone();
        let on_success = props.on_success.clone();
        let auto_close = props.auto_close_on_success;
        let loading = loading.clone();
        use_effect_with(props.success_message.clone(), move |msg| {
            if msg.is_some() {
                loading.set(false);
                if let Some(cb) = &on_success {
                    cb.emit(());
                }
                if auto_close {
                    if let Some(close) = &*close_cb.borrow() {
                        close.emit(());
                    }
                }
            }
            || ()
        });
    }

    // 6) Fire on_error when error_message arrives (and clear loading)
    {
        let on_error = props.on_error.clone();
        let loading = loading.clone();
        use_effect_with(props.error_message.clone(), move |msg| {
            if msg.is_some() {
                loading.set(false);
                if let Some(cb) = &on_error {
                    cb.emit(());
                }
            }
            || ()
        });
    }

    // 7) Build the footer slot, now using our `<Button>` and internal loading
    let footer = {
        let submit_lbl = props.submit_label.clone();
        let extra = props.extra_footer_buttons.clone();
        let close_cb = close_cb.clone();
        let form_id = (*form_id).clone();
        let loading = *loading;

        Callback::from(move |set_modal_close: Callback<()>| {
            *close_cb.borrow_mut() = Some(set_modal_close.clone());
            html! {
                <div class="flex justify-end space-x-2 pt-2">
                    {
                        if let Some(cb) = &extra {
                            cb.emit(set_modal_close.clone())
                        } else {
                            html!{}
                        }
                    }
                    <Button
                        button_type={ButtonType::Submit}
                        form={Some(form_id.clone())}
                        disabled={loading}
                    >
                        { submit_lbl.clone() }
                    </Button>
                </div>
            }
        })
    };

    html! {
        <ModalButton
            trigger_children={html!{ props.modal_button.button_text.clone() }}
            button_type={props.modal_button.button_type.clone()}
            modal_title={props.modal_button.modal_title.clone()}
            modal_size={props.modal_button.modal_size}
            is_open={props.modal_button.is_open}
            on_modal_close={props.modal_button.on_modal_close.clone()}
            footer={Some(footer)}
            modal_content={html! {
                <Form
                    id={Some((*form_id).clone())}
                    onsubmit_callback={internal_submit}
                    error_message={props.error_message.clone()}
                    success_message={props.success_message.clone()}
                    show_submit_button={false}
                >
                    { for props.children.iter() }
                </Form>
            }}
        />
    }
}
