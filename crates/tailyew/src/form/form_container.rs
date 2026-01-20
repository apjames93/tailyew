use crate::atoms::{Button, ButtonType};
use crate::form::FormSubmitCallback;
use crate::molecules::{Notification, NotificationTypes};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlFormElement;
use yew::events::SubmitEvent;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct FormProps {
    pub children: Children,

    pub onsubmit_callback: FormSubmitCallback,

    #[prop_or_default]
    pub form_class: Classes,

    #[prop_or("Submit".into())]
    pub button_label: String,

    #[prop_or(true)]
    pub show_submit_button: bool,

    #[prop_or_default]
    pub id: Option<AttrValue>,

    #[prop_or_default]
    pub extra_footer_buttons: Option<Callback<Callback<()>, Html>>,

    #[prop_or_default]
    pub disabled: bool,
}

#[component(Form)]
pub fn form(props: &FormProps) -> Html {
    let FormProps {
        children,
        onsubmit_callback,
        form_class,
        button_label,
        show_submit_button,
        id,
        extra_footer_buttons,
        disabled,
    } = props.clone();

    // Internal state
    let loading = use_state(|| false);
    let error_message = use_state(|| None::<String>);
    let success_message = use_state(|| None::<String>);

    // Compute form CSS classes
    let form_classes = if form_class.is_empty() {
        classes!(
            "space-y-6",
            "p-1",
            "transition",
            "duration-150",
            "dark:text-gray-200"
        )
    } else {
        form_class.clone()
    };

    // Wrap onsubmit logic
    let onsubmit_wrapper = {
        let loading = loading.clone();
        let error_message = error_message.clone();
        let success_message = success_message.clone();
        let onsubmit_callback = onsubmit_callback.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            // HTML5 form validation
            if let Some(target) = e.target() {
                if let Ok(form_el) = target.dyn_into::<HtmlFormElement>() {
                    if !form_el.check_validity() {
                        return;
                    }
                }
            }

            loading.set(true);
            error_message.set(None);
            success_message.set(None);

            let future = onsubmit_callback.emit(e.clone());
            spawn_local({
                let loading = loading.clone();
                let error_message = error_message.clone();
                let success_message = success_message.clone();

                async move {
                    match future.await {
                        Ok(Some(msg)) => success_message.set(Some(msg)),
                        Ok(None) => success_message.set(None),
                        Err(err) => error_message.set(Some(err)),
                    }
                    loading.set(false);
                }
            });
        })
    };

    html! {
        <div>
            {
                if let Some(err) = &*error_message {
                    html! {
                        <Notification
                            message={err.clone()}
                            notification_type={NotificationTypes::Error}
                            visible={true}
                            fixed={false}
                            show_close={true}
                        />
                    }
                } else if let Some(suc) = &*success_message {
                    html! {
                        <Notification
                            message={suc.clone()}
                            notification_type={NotificationTypes::Success}
                            visible={true}
                            fixed={false}
                            show_close={true}
                        />
                    }
                } else {
                    html! {}
                }
            }

            <form
                id={id.clone()}
                class={form_classes}
                onsubmit={onsubmit_wrapper}
            >
                { for children.iter() }

                if show_submit_button || extra_footer_buttons.is_some() {
                    <div class="flex justify-end space-x-2 pt-2">
                        { if let Some(cb) = &extra_footer_buttons {
                            cb.emit(Callback::from(|_| {}))
                        } else {
                            html! {}
                        }}
                        if show_submit_button {
                            <Button
                            button_type={ButtonType::Submit}
                            disabled={*loading || disabled}
                            class="ml-auto"
                        >
                            {
                                if *loading {
                                    html! { "Submitting..." }
                                } else {
                                    html! { button_label.clone() }
                                }
                            }
                        </Button>
                        }
                    </div>
                }
            </form>
        </div>
    }
}
