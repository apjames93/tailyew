use crate::atoms::{Button, ButtonType};
use crate::molecules::{Notification, NotificationTypes};
use wasm_bindgen::JsCast;
use web_sys::HtmlFormElement;
use yew::events::SubmitEvent;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct FormProps {
    pub children: Children,
    pub onsubmit_callback: Callback<SubmitEvent>,

    #[prop_or_default]
    pub form_class: Classes,

    #[prop_or("Submit".into())]
    pub button_label: String,

    #[prop_or(true)]
    pub show_submit_button: bool,

    #[prop_or(false)]
    pub loading: bool,

    #[prop_or_default]
    pub id: Option<String>,

    #[prop_or_default]
    pub error_message: Option<String>,
    #[prop_or_default]
    pub success_message: Option<String>,

    #[prop_or_default]
    pub extra_footer_buttons: Option<Callback<Callback<()>, Html>>,
}

#[function_component(Form)]
pub fn form(props: &FormProps) -> Html {
    let FormProps {
        children,
        onsubmit_callback,
        form_class,
        button_label,
        show_submit_button,
        loading,
        id,
        error_message: prop_error,
        success_message: prop_success,
        extra_footer_buttons,
    } = props.clone();

    // Compute CSS classes for the <form>
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

    // Wrap onsubmit to prevent default and validate
    let onsubmit_wrapper = {
        let onsubmit_callback = onsubmit_callback.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if let Some(target) = e.target() {
                if let Ok(form_el) = target.dyn_into::<HtmlFormElement>() {
                    if !form_el.check_validity() {
                        return;
                    }
                }
            }
            onsubmit_callback.emit(e);
        })
    };

    html! {
        <div>
            {
                // Show either error or success based on props
                if let Some(err) = &prop_error {
                    html! {
                        <Notification
                            message={err.clone()}
                            notification_type={NotificationTypes::Error}
                            visible={true}
                            fixed={false}
                            show_close={true}
                        />
                    }
                } else if let Some(suc) = &prop_success {
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
                                disabled={loading}
                                class="ml-auto"
                            >{ button_label.clone() }</Button>
                        }
                    </div>
                }
            </form>
        </div>
    }
}
