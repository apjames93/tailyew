use crate::templates::demos::{DemoComponent, FormInputs};
use gloo_net::http::Request;
use tailyew::atoms::{Button, ButtonType, TagType, Typo};
use tailyew::form::*;
use tailyew::organisms::table::Column;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use web_sys::SubmitEvent;
use yew::prelude::*;

const USAGE_CODE: &str = r#"
let form_values = use_state(|| "".to_string());
let response_text = use_state(|| "".to_string());
let error_message = use_state(|| None::<String>);
let success_message = use_state(|| None::<String>);

let onsubmit_callback = {
    let form_values = form_values.clone();
    let response_text = response_text.clone();
    let error_message = error_message.clone();
    let success_message = success_message.clone();
    Callback::from(move |e: SubmitEvent| {
        e.prevent_default();
        // Collect form values
        let mut values = String::new();
        let fields = vec![
            "email", "password", "search", "color", "range", "date", "age",
            "time", "textarea", "select", "gender", "file_upload", "phone", "language",
        ];
        for field in fields {
            let v = e_input_value(field, &e);
            values.push_str(&format!("{}: {}\n", field, v));
        }
        let checked = e_checkbox_checked("checkbox", &e);
        values.push_str(&format!("checkbox: {}\n", checked));
        form_values.set(values);

        // Fetch HTTP status code
        let code = e_input_value("status", &e);
        let response_text = response_text.clone();
        let error_message = error_message.clone();
        let success_message = success_message.clone();
        spawn_local(async move {
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
                        success_message.set(Some(format!("Success {}: {}", status, text)));
                    } else {
                        error_message.set(Some(format!("Error {}: {}", status, text)));
                    }
                }
                Err(err) => {
                    error_message.set(Some(format!("Network error: {}", err)));
                }
            }
        });
    })
};
"#;

#[function_component(FormDemoSection)]
pub fn form_demo_section() -> Html {
    // Shared state
    let form_values = use_state(|| "".to_string());
    let response_text = use_state(|| "".to_string());
    let error_message = use_state(|| None::<String>);
    let success_message = use_state(|| None::<String>);

    // Build onsubmit callback
    let onsubmit = {
        let form_values = form_values.clone();
        let response_text = response_text.clone();
        let error_message = error_message.clone();
        let success_message = success_message.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            // Collect form values
            let mut values = String::new();
            let fields = vec![
                "email",
                "password",
                "search",
                "color",
                "range",
                "date",
                "age",
                "time",
                "textarea",
                "select",
                "gender",
                "file_upload",
                "phone",
                "language",
            ];
            for field in &fields {
                let v = e_input_value(field, &e);
                values.push_str(&format!("{}: {}\n", field, v));
            }
            let checked = e_checkbox_checked("checkbox", &e);
            values.push_str(&format!("checkbox: {}\n", checked));
            form_values.set(values);

            // Fetch HTTP status code
            let code = e_input_value("status", &e);
            let response_text = response_text.clone();
            let error_message = error_message.clone();
            let success_message = success_message.clone();
            spawn_local(async move {
                error_message.set(None);
                success_message.set(None);
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
                            success_message.set(Some(format!("Success {}: {}", status, text)));
                        } else {
                            error_message.set(Some(format!("Error {}: {}", status, text)));
                        }
                    }
                    Err(err) => {
                        error_message.set(Some(format!("Network error: {}", err)));
                    }
                }
            });
        })
    };

    // Extra footer action button
    let extra_footer_buttons = Some(Callback::from(move |_| {
        html! {
            <Button
                button_type={ButtonType::Ghost}
                onclick={Callback::from(move |_| console::log_1(&"Extra Action Clicked".into()))}
            >
                { "Extra Action" }
            </Button>
        }
    }));

    // Render example
    let example = html! {
        <section class="max-w-4xl mx-auto p-6 bg-white dark:bg-gray-800 rounded-lg shadow-lg space-y-6">
            <Typo tag={TagType::H1}>{ "Form with HTTP Fetch Demo" }</Typo>
            <Form
                onsubmit_callback={onsubmit.clone()}
                button_label="Submit and Fetch"
                error_message={(*error_message).clone()}
                success_message={(*success_message).clone()}
                extra_footer_buttons={extra_footer_buttons.clone()}
            >
                <FormInputs />
                <div class="mt-4 text-sm whitespace-pre-wrap text-gray-700 dark:text-gray-300">
                    { (*form_values).clone() }
                </div>
                <div class="mt-2 text-sm whitespace-pre-wrap p-2 border rounded bg-gray-50 dark:bg-gray-700">
                    { (*response_text).clone() }
                </div>
            </Form>
        </section>
    };

    // Props table now includes form_class
    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "children",
                "onsubmit_callback",
                "form_class",
                "show_submit_button",
                "loading",
                "button_label",
                "id",
                "error_message",
                "on_error_clear",
                "success_message",
                "on_success_clear",
                "extra_footer_buttons",
            ]
            .into_iter()
            .map(|s| html! {s})
            .collect(),
        },
        Column {
            header: "Type".into(),
            values: vec![
                "Children",
                "Callback<SubmitEvent>",
                "Classes",
                "bool",
                "bool",
                "String",
                "Option<String>",
                "Option<String>",
                "Option<Callback<()>>",
                "Option<String>",
                "Option<Callback<()>>",
                "Option<Callback<Callback<()>, Html>>",
            ]
            .into_iter()
            .map(|s| html! {s})
            .collect(),
        },
        Column {
            header: "Description".into(),
            values: vec![
                "Form content (fields and layout).",
                "Callback on submit event.",
                "Tailwind classes for form styling.",
                "Toggle visibility of submit button.",
                "Disable button when loading.",
                "Text for the submit button.",
                "Optional form element ID.",
                "Error banner message.",
                "Success banner message.",
                "Optional extra footer buttons.",
            ]
            .into_iter()
            .map(|s| html! {s})
            .collect(),
        },
    ];

    html! {
        <DemoComponent
            github_demo_path="form/form_demo_section.rs"
            github_source_path="form/form_container.rs"
            title="Form HTTP Fetch Demo"
            description={Some(html! {<p>{"Submit the form to collect all field values and then fetch the given HTTP status code from httpstat.us."}</p>})}
            example={example}
            usage_code={USAGE_CODE}
            props_table={Some(props_table)}
        />
    }
}
