use crate::templates::demos::{DemoComponent, FormInputs};
use gloo_net::http::Request;
use tailyew::form::*;
use tailyew::{async_callback, Button, ButtonType, ModalSize};
use web_sys::SubmitEvent;
use yew::prelude::*;

#[function_component(FormModalDemoSection)]
pub fn form_modal_demo_section() -> Html {
    // Response body text
    let response_text = use_state(|| "".to_string());

    // onsubmit callback: fetch from httpstat.us
    let onsubmit = async_callback({
        let response_text = response_text.clone();

        move |e: SubmitEvent| {
            let response_text = response_text.clone();

            async move {
                // Grab the status code from the form
                let code = e_input_value("status", &e);

                let url = format!("https://httpstat.us/{}", code);
                match Request::get(&url)
                    .header("Accept", "application/json")
                    .send()
                    .await
                {
                    Ok(resp) => {
                        let status = resp.status();
                        let text = resp.text().await.unwrap_or_default();
                        response_text.set(text.clone());

                        if (200..300).contains(&status) {
                            let msg = format!("Success {}: {}", status, text);
                            Ok(Some(msg))
                        } else {
                            let msg = format!("Error {}: {}", status, text);
                            Err(msg)
                        }
                    }
                    Err(err) => {
                        let msg = format!("Network error: {}", err);
                        Err(msg)
                    }
                }
            }
        }
    });

    // Reusable extra footer buttons for manual close
    let extra_buttons = |label: String| {
        Callback::from(move |close_modal: Callback<()>| {
            let cancel_label = label.clone();
            html! {
                <Button
                    button_type={ButtonType::Secondary}
                    onclick={
                        let close_modal = close_modal.clone();
                        Callback::from(move |_| {
                            web_sys::console::log_1(&format!("{cancel_label} Cancel clicked").into());
                            close_modal.emit(());
                        })
                    }
                >
                    { "Cancel" }
                </Button>
            }
        })
    };

    let example = html! {
        <div class="max-w-xl mx-auto space-y-8">
            // Auto-close on 2xx, auto-closes modal
            <FormModal
                modal_button={ModalButtonConfig {
                    button_text: "Fetch and Auto-Close".into(),
                    button_type: ButtonType::Primary,
                    modal_title: "HTTP Fetch (Auto-Close)".into(),
                    modal_size: ModalSize::Large,
                    is_open: false,
                    on_modal_close: None,
                }}
                onsubmit={onsubmit.clone()}
                submit_label={"Fetch"}
                auto_close_on_success={true}
                extra_footer_buttons={Some(extra_buttons("Auto-Close".to_string()))}
            >
                <FormInputs />
            </FormModal>

            // Manual-close example
            <FormModal
                modal_button={ModalButtonConfig {
                    button_text: "Fetch (Manual Close)".into(),
                    button_type: ButtonType::Primary,
                    modal_title: "HTTP Fetch (Manual)".into(),
                    modal_size: ModalSize::Large,
                    is_open: false,
                    on_modal_close: None,
                }}
                onsubmit={onsubmit.clone()}
                submit_label={"Fetch"}
                auto_close_on_success={false}
                extra_footer_buttons={Some(extra_buttons("Manual".to_string()))}
            >
                <FormInputs />
            </FormModal>

            // Display response body
            <div class="text-sm whitespace-pre-wrap text-gray-700 dark:text-gray-300 border rounded p-2">
                { (*response_text).clone() }
            </div>
        </div>
    };

    html! {
        <DemoComponent
            github_demo_path="form/form_modal_demo_section.rs"
            github_source_path="form/form_modal.rs"
            title="FormModal HTTP Fetch Demo"
            description={Some(html! {
                <p>{"This demo shows how to drive success/error notifications from real HTTP responses using the httpstat.us API."}</p>
            })}
            example={example}
            usage_code={r#"
<FormModal
    modal_button={ModalButtonConfig {
        button_text: "Fetch and Auto-Close".into(),
        button_type: ButtonType::Primary,
        modal_title: "HTTP Fetch (Auto-Close)".into(),
        modal_size: ModalSize::Large,
        is_open: false,
        on_modal_close: None,
    }}
    onsubmit={onsubmit}
    submit_label="Fetch"
    auto_close_on_success={true}
    extra_footer_buttons={Some(extra_buttons)}
>
    <Input id="status" label="Status Code" input_type={InputType::Number} placeholder="Enter status code" />
</FormModal>
"#}
            props_table={None}
        />
    }
}
