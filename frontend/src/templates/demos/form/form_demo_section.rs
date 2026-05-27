use crate::templates::demos::{DemoComponent, FormInputs};
use gloo_net::http::Request;
use tailyew::atoms::{Button, ButtonType, TagType, Typo};
use tailyew::form::*;
use tailyew::organisms::table::Column;
use web_sys::SubmitEvent;
use web_sys::console;
use yew::prelude::*;

const USAGE_CODE: &str = r#"
let submitted = use_state(|| "Submit has not run.".to_owned());

let onsubmit_callback = async_callback({
    let submitted = submitted.clone();

    move |e: SubmitEvent| {
        let submitted = submitted.clone();

        async move {
            // Form has already prevented default browser navigation and
            // reported native validity before this callback runs.
            let name = e_input_value("name", &e);
            let email = e_input_value("email", &e);
            submitted.set(format!("Submitted name={name}, email={email}"));
            Ok(Some("Form submitted.".to_owned()))
        }
    }
});

html! {
    <Form onsubmit_callback={onsubmit_callback}>
        <Input id="name" label="Name" required={true} />
        <Input
            id="email"
            label="Email"
            input_type={InputType::Email}
            required={true}
        />
    </Form>
}
"#;

#[component(FormDemoSection)]
pub fn form_demo_section() -> Html {
    // Shared state
    let form_values = use_state(|| "".to_string());
    let response_text = use_state(|| "".to_string());
    let error_message = use_state(|| None::<String>);
    let success_message = use_state(|| None::<String>);
    let validation_preview = use_state(|| "Submit has not run.".to_owned());

    let validation_submit = async_callback({
        let validation_preview = validation_preview.clone();

        move |e: SubmitEvent| {
            let validation_preview = validation_preview.clone();

            async move {
                let name = e_input_value("validation_name", &e);
                let email = e_input_value("validation_email", &e);
                validation_preview.set(format!("Submitted name={name}, email={email}"));
                Ok(Some("Validation form submitted.".to_owned()))
            }
        }
    });

    // Build onsubmit callback
    let onsubmit_callback = async_callback({
        let form_values = form_values.clone();
        let response_text = response_text.clone();
        let error_message = error_message.clone();
        let success_message = success_message.clone();

        move |e: SubmitEvent| {
            let form_values = form_values.clone();
            let response_text = response_text.clone();
            let error_message = error_message.clone();
            let success_message = success_message.clone();

            async move {
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

                let switch = e_checkbox_checked("switch", &e);
                values.push_str(&format!("switch: {}\n", switch));

                form_values.set(values);
                let code = e_input_value("status", &e);
                let url = format!("https://tools-httpstatus.pickup-services.com/{}", code);

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
                            success_message.set(Some(msg.clone()));
                            Ok(Some(msg))
                        } else {
                            let msg = format!("Error {}: {}", status, text);
                            error_message.set(Some(msg.clone()));
                            Err(msg)
                        }
                    }
                    Err(err) => {
                        let msg = format!("Network error: {}", err);
                        error_message.set(Some(msg.clone()));
                        Err(msg)
                    }
                }
            }
        }
    });

    // Extra footer action button
    let extra_footer_buttons = Some(Callback::from(move |_| {
        html! {
            <Button
                button_type={ButtonType::Ghost}
                on_click={Callback::from(move |_| console::log_1(&"Extra Action Clicked".into()))}
            >
                { "Extra Action" }
            </Button>
        }
    }));

    // Render example
    let example = html! {
        <section class="max-w-4xl mx-auto p-6 bg-white dark:bg-gray-800 rounded-lg shadow-lg space-y-6">
            <Typo tag={TagType::H1}>{ "Form with HTTP Fetch Demo" }</Typo>
            <div class="rounded-lg border border-gray-200 p-4 dark:border-gray-700">
                <Typo tag={TagType::H2}>{ "Native Validation Gate" }</Typo>
                <p class="mb-4 text-sm text-gray-600 dark:text-gray-300">
                    { "The Form checks native validity before calling the submit callback. The preview below only updates after required fields and email format are valid." }
                </p>
                <Form
                    onsubmit_callback={validation_submit}
                    button_label="Submit validation example"
                >
                    <Input
                        id="validation_name"
                        label="Name"
                        placeholder="Enter a name"
                        required={true}
                    />
                    <Input
                        id="validation_email"
                        label="Email"
                        input_type={InputType::Email}
                        default_value="not-an-email"
                        required={true}
                    />
                    <p class="rounded border border-gray-200 bg-gray-50 p-2 text-sm text-gray-700 dark:border-gray-700 dark:bg-gray-900 dark:text-gray-200">
                        { (*validation_preview).clone() }
                    </p>
                </Form>
            </div>
            <Form
                onsubmit_callback={onsubmit_callback.clone()}
                button_label="Submit and Fetch"
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
            description={Some(html! {<p>{"Form prevents invalid native submissions before running callbacks, then valid submissions can collect field values and perform async work."}</p>})}
            example={example}
            usage_code={USAGE_CODE}
            props_table={Some(props_table)}
        />
    }
}
