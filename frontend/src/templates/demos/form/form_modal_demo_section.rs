use crate::templates::demos::DemoComponent;
use tailyew::form::*;
use tailyew::{Button, ButtonType, ModalSize};
use web_sys::SubmitEvent;
use yew::prelude::*;

#[function_component(FormModalDemoSection)]
pub fn form_modal_demo_section() -> Html {
    let response = use_state(|| "".to_string());
    let error_message = use_state(|| None::<String>);
    let success_message = use_state(|| None::<String>);

    let onsubmit = {
        let response = response.clone();
        let error_message = error_message.clone();
        let success_message = success_message.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let email = e_input_value("email", &e);
            let phone = e_input_value("phone", &e);
            let accepted = e_checkbox_checked("terms", &e);

            if !email.contains('@') {
                error_message.set(Some("Email must contain '@'.".into()));
                success_message.set(None);
            } else {
                response.set(format!(
                    "email: {}\nphone: {}\nterms accepted: {}",
                    email, phone, accepted
                ));
                error_message.set(None);
                success_message.set(Some("Form submitted successfully.".into()));
            }
        })
    };

    let extra_buttons = |label: String| {
        Callback::from(move |close_modal: Callback<()>| {
            let cancel_label = label.clone();
            let delete_label = label.clone();

            html! {
                <>
                    <Button
                        button_type={ButtonType::Secondary}
                        onclick={{
                            let close_modal = close_modal.clone();
                            Callback::from(move |_| {
                                web_sys::console::log_1(&format!("{cancel_label} Cancel clicked").into());
                                close_modal.emit(());
                            })
                        }}
                    >
                        { "Cancel" }
                    </Button>
                    <Button
                        button_type={ButtonType::Danger}
                        onclick={{
                            let close_modal = close_modal.clone();
                            Callback::from(move |_| {
                                web_sys::console::log_1(&format!("{delete_label} Delete clicked").into());
                                close_modal.emit(());
                            })
                        }}
                    >
                        { "Delete" }
                    </Button>
                </>
            }
        })
    };

    let example = html! {
        <div class="max-w-xl mx-auto space-y-8">
            <FormModal
                modal_button={ModalButtonConfig {
                    button_text: "Auto Close Form".into(),
                    button_type: ButtonType::Primary,
                    modal_title: "Auto-Close on Success".into(),
                    modal_size: ModalSize::Large,
                    is_open: false,
                    on_modal_close: None,
                }}
                onsubmit={onsubmit.clone()}
                error_message={(*error_message).clone()}
                success_message={(*success_message).clone()}
                submit_label="Save"
                auto_close_on_success={true}
                on_success={Some(Callback::from(|_| web_sys::console::log_1(&"✅ Success callback".into())))}
                on_error={Some(Callback::from(|_| web_sys::console::log_1(&"❌ Error callback".into())))}
                extra_footer_buttons={Some(extra_buttons("Auto Close".to_string()))}
            >
                <Input id="email" label="Email" input_type={InputType::Email} placeholder="you@example.com" />
                <PhoneInput id="phone" label="Phone" placeholder="123-456-7890" />
                <Checkbox id="terms" label="I accept the terms" />
            </FormModal>

            <FormModal
                modal_button={ModalButtonConfig {
                    button_text: "Manual Close Form".into(),
                    button_type: ButtonType::Primary,
                    modal_title: "Manual Modal Close".into(),
                    modal_size: ModalSize::Large,
                    is_open: false,
                    on_modal_close: None,
                }}
                onsubmit={onsubmit}
                error_message={(*error_message).clone()}
                success_message={(*success_message).clone()}
                submit_label="Submit"
                auto_close_on_success={false}
                extra_footer_buttons={Some(extra_buttons("Manual Close".to_string()))}
            >
                <Input id="email" label="Email" input_type={InputType::Email} placeholder="you@example.com" />
                <PhoneInput id="phone" label="Phone" placeholder="123-456-7890" default_value="123-456-7890" />
                <Checkbox id="terms" label="I accept the terms" />
            </FormModal>

            <div class="text-sm whitespace-pre-wrap text-gray-700 dark:text-gray-300 border rounded p-2">
                { (*response).clone() }
            </div>
        </div>
    };

    html! {
        <DemoComponent
            github_demo_path="form/form_modal_demo_section.rs"
            github_source_path="form/form_modal.rs"
            title="FormModal Component"
            description={Some(html! {
                <p>{"The `FormModal` combines `Form` and `ModalButton`, handling submission, state, and flexible modal footers. You can configure auto-close, attach callbacks, and display validation messages."}</p>
            })}
            example={example}
            usage_code={r#"
<FormModal
    button_text="Open Modal".into()
    modal_title="Form Modal Example".into()
    onsubmit={onsubmit}
    error_message={error_message.clone()}
    success_message={success_message.clone()}
    submit_label="Save"
    auto_close_on_success={true}
    on_success={Some(on_success_cb)}
    on_error={Some(on_error_cb)}
    extra_footer_buttons={Some(Callback::from(|close_modal: Callback<()>| {
        html! {
            <>
                <Button
                    button_type={ButtonType::Secondary}
                    onclick={Callback::from(move |_| close_modal.emit(()))}
                >
                    { "Cancel" }
                </Button>
                <Button
                    button_type={ButtonType::Danger}
                    onclick={Callback::from(move |_| close_modal.emit(()))}
                >
                    { "Delete" }
                </Button>
            </>
        }
    }))}
>
    <Input id="email" label="Email" input_type={InputType::Email} />
    <Checkbox id="terms" label="I accept the terms" />
</FormModal>
            "#}
            props_table={None}
        />
    }
}
