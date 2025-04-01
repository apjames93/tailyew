use crate::atoms::{Button, ButtonType};
use crate::molecules::{Notification, NotificationTypes};
use yew::prelude::*;

use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlFormElement, HtmlInputElement};
use yew::events::SubmitEvent;

pub fn e_input_value(id: &str, e: &SubmitEvent) -> String {
    let target: EventTarget = e.target().expect("Event should have a target.");
    let form: HtmlFormElement = target.unchecked_into();

    // Use get_with_name and handle the None case explicitly
    if let Some(input) = form.get_with_name(id) {
        let input: HtmlInputElement = input.unchecked_into();
        input.value().to_string()
    } else {
        web_sys::console::error_1(
            &format!("Input element with name '{}' not found in form.", id).into(),
        );
        String::new() // Return an empty string or handle the error as needed
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
    #[prop_or_else(|| Some("".to_string()))]
    pub form_class: Option<String>,
    #[prop_or_else(|| Some("Submit".to_string()))]
    pub button_label: Option<String>,
    #[prop_or(true)]
    pub show_submit_button: bool,
    #[prop_or(false)]
    pub loading: bool,
    #[prop_or_else(|| Some("".to_string()))]
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

    let form_classes = if let Some(class) = form_class {
        classes!(class.clone())
    } else {
        classes!(
            "space-y-6",          // Spacing between elements
            "p-1",                // Padding
            "transition",         // Smooth transitions
            "duration-150",       // Transition duration
            "dark:text-gray-200", // Dark mode text color
        )
    };

    // Create the onsubmit callback event handler
    let onsubmit = onsubmit_callback.clone();

    html! {
        <>
            if let Some(error) = error_message {
                <Notification
                    message={error.clone()}
                    notification_type={NotificationTypes::Error}
                    is_visible={true}
                    is_fixed={false}
                />
            }
            if let Some(success) = success_message {
                <Notification
                    message={success.clone()}
                    notification_type={NotificationTypes::Success}
                    is_visible={true}
                    is_fixed={false}
                />
            }

            <form
                id={id.clone()}
                class={form_classes}
                {onsubmit}
            >
                { for children.iter() }

                if *show_submit_button {
                    <div class="flex justify-end mt-4">
                        <Button
                            button_type={ButtonType::Submit}
                            disabled={loading}
                            class="ml-auto"
                        >
                            { button_label.clone().unwrap_or_else(|| "Submit".to_string()) }
                        </Button>
                    </div>
                }
            </form>
        </>
    }
}
