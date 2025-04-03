use crate::atoms::{Button, ButtonType};
use crate::molecules::{Notification, NotificationTypes};
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlFormElement, HtmlInputElement};
use yew::events::SubmitEvent;
use yew::prelude::*;

pub fn e_input_value(id: &str, e: &SubmitEvent) -> String {
    let target: EventTarget = e.target().expect("Event should have a target.");
    let form: HtmlFormElement = target.unchecked_into();
    if let Some(input) = form.get_with_name(id) {
        let input: HtmlInputElement = input.unchecked_into();
        input.value()
    } else {
        web_sys::console::error_1(
            &format!("Input element with name '{}' not found in form.", id).into(),
        );
        String::new()
    }
}

pub fn e_checkbox_checked(id: &str, e: &SubmitEvent) -> bool {
    let target = e.target().expect("Event should have a target.");
    let form: HtmlFormElement = target.unchecked_into();
    if let Some(input) = form.get_with_name(id) {
        let input: HtmlInputElement = input.unchecked_into();
        input.checked()
    } else {
        false
    }
}

#[derive(Properties, PartialEq)]
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
        error_message,
        success_message,
    } = props;

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

    html! {
        <div>
            if let Some(error) = error_message {
                <Notification
                    message={error.to_string()}
                    notification_type={NotificationTypes::Error}
                    visible={true}
                    fixed={false}
                />
            }
            if let Some(success) = success_message {
                <Notification
                    message={success.to_string()}
                    notification_type={NotificationTypes::Success}
                    visible={true}
                    fixed={false}
                />
            }

            <form id={id.clone()} class={form_classes} onsubmit={onsubmit_callback.clone()}>
                { for children.iter() }

                if *show_submit_button {
                    <div class="flex justify-end mt-4">
                        <Button
                            button_type={ButtonType::Submit}
                            disabled={*loading}
                            class="ml-auto"
                        >
                            { button_label.clone() }
                        </Button>
                    </div>
                }
            </form>
        </div>
    }
}
