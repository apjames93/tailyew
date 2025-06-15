use crate::templates::demos::DemoComponent;
use gloo_net::http::Request;
use tailyew::{
    e_form_builder_values, Button, ButtonType, CheckboxProps, ColorInputProps, Column,
    FileInputProps, FormBuilder, FormValue, InputProps, InputType, ModalButtonConfig, ModalConfig,
    ModalSize, PhoneInputProps, RadioGroupProps, RangeInputProps, RenderFieldProps,
    SearchInputProps, SelectOption, SelectProps, StateDropdownProps, TagType, TextareaProps, Typo,
};
use wasm_bindgen_futures::spawn_local;
use web_sys::SubmitEvent;
use yew::prelude::*;

const USAGE_CODE: &str = r#"
let onsubmit = Callback::from(|e: SubmitEvent| { /* … */ });
let inputs = vec![ /* RenderFieldProps … */ ];
html! {
  <FormBuilder
    onsubmit={onsubmit}
    button_label={Some("My Button".into())}
    error_message={Some("Oops!".into())}
    success_message={Some("Yay!".into())}
    inputs={inputs}
    modal_config={Some( ModalConfig { /* … */ } )}
  />
}
"#;

#[function_component(FormBuilderDemoSection)]
pub fn form_builder_demo_section() -> Html {
    // 1) Shared state for displaying submitted values and banners
    let form_values = use_state(|| "".to_string());
    let response_text = use_state(|| "".to_string());
    let error_message = use_state(|| None::<String>);
    let success_message = use_state(|| None::<String>);

    // 2) Gather & fetch on submit
    let onsubmit = {
        let form_values = form_values.clone();
        let response_text = response_text.clone();
        let error_message = error_message.clone();
        let success_message = success_message.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            // 2a) Gather all form values in one pass
            let ids = [
                "status",
                "username",
                "language",
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
                "state",
                "accept_terms",
            ];
            let fields: Vec<RenderFieldProps> = ids
                .iter()
                .map(|&id| {
                    let mut rf = RenderFieldProps::default();
                    if id == "accept_terms" {
                        rf.checkbox = Some(CheckboxProps {
                            id: id.into(),
                            ..Default::default()
                        });
                    } else {
                        rf.input = Some(InputProps {
                            id: id.into(),
                            ..Default::default()
                        });
                    }
                    rf
                })
                .collect();

            let all = e_form_builder_values(&e, &fields);
            let mut out = String::new();
            for (k, v) in &all {
                match v {
                    FormValue::Text(s) => out.push_str(&format!("{}: {}\n", k, s)),
                    FormValue::Checked(b) => out.push_str(&format!("{}: {}\n", k, b)),
                }
            }
            form_values.set(out);

            // 2b) Fetch HTTP status code
            let code = match all.get("status") {
                Some(FormValue::Text(c)) => c.clone(),
                _ => "".into(),
            };

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
                        let st = resp.status();
                        let txt = resp.text().await.unwrap_or_default();
                        response_text.set(txt.clone());
                        if (200..300).contains(&st) {
                            success_message.set(Some(format!("Success {}: {}", st, txt)));
                        } else {
                            error_message.set(Some(format!("Error {}: {}", st, txt)));
                        }
                    }
                    Err(err) => {
                        error_message.set(Some(format!("Network error: {}", err)));
                    }
                }
            });
        })
    };

    // 3) Define all of the form fields
    let inputs = vec![
        RenderFieldProps {
            input: Some(InputProps {
                id: "status".into(),
                label: "Status Code".into(),
                input_type: InputType::Number,
                placeholder: "e.g. 200 or 500".into(),
                default_value: "200".into(),
                ..Default::default()
            }),
            ..Default::default()
        },
        RenderFieldProps {
            input: Some(InputProps {
                id: "username".into(),
                label: "Username".into(),
                input_type: InputType::Text,
                placeholder: "e.g. buddy_guy".into(),
                default_value: "".into(),
                required: true,
                pattern: Some("^[a-z0-9_-]{3,16}$".into()),
                error_title: Some(
                    "Use 3–16 lowercase letters, numbers, underscores, or dashes.".into(),
                ),
                ..Default::default()
            }),
            ..Default::default()
        },
        RenderFieldProps {
            search: Some(SearchInputProps {
                id: "language".into(),
                label: Some("Language".into()),
                placeholder: Some("Choose a language".into()),
                items: vec![
                    SelectOption {
                        label: "Rust".into(),
                        value: "rust".into(),
                    },
                    SelectOption {
                        label: "Go".into(),
                        value: "go".into(),
                    },
                    SelectOption {
                        label: "TypeScript".into(),
                        value: "typescript".into(),
                    },
                ],
                required: true,
                debounce_ms: 300,
                error_title: "Please select a value from the list.".into(),
                ..Default::default()
            }),
            ..Default::default()
        },
        RenderFieldProps {
            input: Some(InputProps {
                id: "email".into(),
                label: "Email".into(),
                input_type: InputType::Email,
                placeholder: "Enter email".into(),
                default_value: "".into(),
                ..Default::default()
            }),
            ..Default::default()
        },
        RenderFieldProps {
            input: Some(InputProps {
                id: "password".into(),
                label: "Password".into(),
                input_type: InputType::Password,
                placeholder: "Enter password".into(),
                default_value: "".into(),
                ..Default::default()
            }),
            ..Default::default()
        },
        RenderFieldProps {
            input: Some(InputProps {
                id: "search".into(),
                label: "Search".into(),
                input_type: InputType::Search,
                placeholder: "Search...".into(),
                default_value: "".into(),
                ..Default::default()
            }),
            ..Default::default()
        },
        RenderFieldProps {
            color: Some(ColorInputProps {
                id: "color".into(),
                label: "Pick a Color".into(),
                value: "#00ffcc".into(),
                ..Default::default()
            }),
            ..Default::default()
        },
        RenderFieldProps {
            range: Some(RangeInputProps {
                id: "range".into(),
                label: "Volume".into(),
                min: "0".into(),
                max: "100".into(),
                step: "5".into(),
                default_value: "50".into(),
                ..Default::default()
            }),
            ..Default::default()
        },
        RenderFieldProps {
            input: Some(InputProps {
                id: "date".into(),
                label: "Date of Birth".into(),
                input_type: InputType::Date,
                placeholder: "YYYY-MM-DD".into(),
                default_value: "".into(),
                ..Default::default()
            }),
            ..Default::default()
        },
        RenderFieldProps {
            input: Some(InputProps {
                id: "age".into(),
                label: "Age".into(),
                input_type: InputType::Number,
                placeholder: "Enter age".into(),
                default_value: "".into(),
                ..Default::default()
            }),
            ..Default::default()
        },
        RenderFieldProps {
            input: Some(InputProps {
                id: "time".into(),
                label: "Time".into(),
                input_type: InputType::Time,
                placeholder: "HH:MM".into(),
                default_value: "".into(),
                ..Default::default()
            }),
            ..Default::default()
        },
        RenderFieldProps {
            textarea: Some(TextareaProps {
                id: "textarea".into(),
                label: "Description".into(),
                default_value: "".into(),
                placeholder: "Write something…".into(),
                ..Default::default()
            }),
            ..Default::default()
        },
        RenderFieldProps {
            select: Some(SelectProps {
                id: "select".into(),
                label: "Select Option".into(),
                options: vec![
                    SelectOption {
                        label: "Option 1".into(),
                        value: "1".into(),
                    },
                    SelectOption {
                        label: "Option 2".into(),
                        value: "2".into(),
                    },
                    SelectOption {
                        label: "Option 3".into(),
                        value: "3".into(),
                    },
                ],
                default_value: "2".into(),
                ..Default::default()
            }),
            ..Default::default()
        },
        RenderFieldProps {
            radio: Some(RadioGroupProps {
                id: "gender".into(),
                label: "Select Gender".into(),
                options: vec![
                    ("male".into(), "Male".into()),
                    ("female".into(), "Female".into()),
                    ("other".into(), "Other".into()),
                ],
                default_value: "female".into(),
                ..Default::default()
            }),
            ..Default::default()
        },
        RenderFieldProps {
            file: Some(FileInputProps {
                id: "file_upload".into(),
                label: "Upload File".into(),
                ..Default::default()
            }),
            ..Default::default()
        },
        RenderFieldProps {
            checkbox: Some(CheckboxProps {
                id: "accept_terms".into(),
                label: "Accept Terms".into(),
                checked: false,
                required: false,
                ..Default::default()
            }),
            ..Default::default()
        },
        RenderFieldProps {
            phone: Some(PhoneInputProps {
                id: "phone".into(),
                label: "Phone".into(),
                placeholder: "123-456-7890".into(),
                default_value: "".into(),
                ..Default::default()
            }),
            ..Default::default()
        },
        RenderFieldProps {
            state: Some(StateDropdownProps {
                id: "state".into(),
                default_value: "NY".into(),
                ..Default::default()
            }),
            ..Default::default()
        },
    ];

    // 4) Modal config
    let modal_cfg = ModalConfig {
        modal_button: ModalButtonConfig {
            button_text: "Register Modal".into(),
            button_type: ButtonType::Primary,
            modal_title: "Register User".into(),
            modal_size: ModalSize::Large,
            is_open: false,
            on_modal_close: None,
        },
        auto_close_on_success: true,
        on_success: None,
        on_error: None,
    };

    // 5) Build props table
    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "onsubmit",
                "inputs",
                "button_label",
                "error_message",
                "success_message",
                "extra_footer_buttons",
                "modal_config",
                "modal_config.modal_button",
                "modal_config.auto_close_on_success",
                "modal_config.on_success",
                "modal_config.on_error",
            ]
            .into_iter()
            .map(Html::from)
            .collect(),
        },
        Column {
            header: "Type".into(),
            values: vec![
                "Callback<SubmitEvent>",
                "Vec<RenderFieldProps>",
                "Option<String>",
                "Option<String>",
                "Option<String>",
                "Option<Callback<Callback<()>, Html>>",
                "Option<ModalConfig>",
                "ModalButtonConfig",
                "bool",
                "Option<Callback<()>>",
                "Option<Callback<()>>",
            ]
            .into_iter()
            .map(Html::from)
            .collect(),
        },
        Column {
            header: "Description".into(),
            values: vec![
                "Submit handler",
                "Fields to render",
                "Inline button label",
                "Inline error banner",
                "Inline success banner",
                "Extra footer buttons slot",
                "Modal settings",
                "Launch-button config",
                "Auto-close on success",
                "Callback after modal success",
                "Callback after modal error",
            ]
            .into_iter()
            .map(Html::from)
            .collect(),
        },
    ];

    // 6) Render everything
    html! {
        <DemoComponent
            github_demo_path="form/form_builder_demo_section.rs"
            github_source_path="form/form_builder.rs"
            title="FormBuilder Component"
            description={Some(html! {
                <p>{"FormBuilder can render inline or in a modal; it wires up banners, callbacks and extra buttons all in one go."}</p>
            })}
            example={html! {
                <section class="max-w-4xl mx-auto p-6 space-y-6 bg-white dark:bg-gray-800 rounded-lg shadow-lg">
                    <Typo tag={TagType::H2}>{"Inline Form"}</Typo>
                    <FormBuilder
                        onsubmit={onsubmit.clone()}
                        inputs={inputs.clone()}
                        button_label={Some("Register Inline".to_string())}
                        error_message={(*error_message).clone()}
                        success_message={(*success_message).clone()}
                        extra_footer_buttons={None::<Callback<Callback<()>, Html>>}
                    />

                    <Typo tag={TagType::H2}>{"Modal Form"}</Typo>
                    <FormBuilder
                        onsubmit={onsubmit.clone()}
                        inputs={inputs.clone()}
                        button_label={Some("Register".to_string())}
                        error_message={(*error_message).clone()}
                        success_message={(*success_message).clone()}
                        extra_footer_buttons={None::<Callback<Callback<()>, Html>>}
                        modal_config={Some(modal_cfg.clone())}
                    />

                    <Typo tag={TagType::H2}>{"Modal w/ Cancel"}</Typo>
                    <FormBuilder
                        onsubmit={onsubmit}
                        inputs={inputs}
                        button_label={Some("Register".to_string())}
                        error_message={(*error_message).clone()}
                        success_message={(*success_message).clone()}
                        extra_footer_buttons={Some(Callback::from(move |close: Callback<()>| html! {
                            <Button
                                button_type={ButtonType::Ghost}
                                onclick={Callback::from(move |_| close.emit(()))}
                            >
                                {"Cancel"}
                            </Button>
                        }))}
                        modal_config={Some(modal_cfg)}
                    />

                    <pre class="mt-4 text-sm whitespace-pre-wrap text-gray-600 dark:text-gray-300">
                        { (*form_values).clone() }
                    </pre>
                </section>
            }}
            usage_code={USAGE_CODE}
            props_table={Some(props_table)}
        />
    }
}
