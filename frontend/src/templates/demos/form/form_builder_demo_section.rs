use crate::templates::demos::DemoComponent;
use tailyew::atoms::{Button, ButtonType, TagType, Typo};
use tailyew::form::*;
use tailyew::organisms::table::Column;
use web_sys::SubmitEvent;
use yew::prelude::*;

const USAGE_CODE: &str = r#"
let form_values = use_state(|| "".to_string());

let onsubmit_callback = {
    let form_values = form_values.clone();
    Callback::from(move |e: SubmitEvent| {
        e.prevent_default();
        let fields = vec!["name", "email", "password", "state", "favorite_color"];
        let mut values = String::new();
        for field in fields {
            let value = e_input_value(field, &e);
            values.push_str(&format!("{}: {}\n", field, value));
        }
        let checked = e_checkbox_checked("accept_terms", &e);
        values.push_str(&format!("accept_terms: {}\n", checked));
        form_values.set(values);
    })
};

html! {
    <FormBuilder config={example_config} onsubmit={onsubmit_callback} />
};
"#;

#[function_component(FormBuilderDemoSection)]
pub fn form_builder_demo_section() -> Html {
    let form_values = use_state(|| "".to_string());

    let extra_footer_buttons = Some(Callback::from(|close_modal: Callback<()>| {
        html! {
            <Button
                button_type={ButtonType::Ghost}
                onclick={Callback::from(move |_| {
                    close_modal.emit(());
                })}
            >
                { "Cancel" }
            </Button>
        }
    }));

    let onsubmit = {
        let form_values = form_values.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let fields = vec!["name", "email", "password", "state", "favorite_color"];
            let mut values = String::new();
            for field in fields {
                let value = e_input_value(field, &e);
                values.push_str(&format!("{}: {}\n", field, value));
            }
            let checked = e_checkbox_checked("accept_terms", &e);
            values.push_str(&format!("accept_terms: {}\n", checked));
            form_values.set(values);
        })
    };

    let base_inputs = vec![
        InputFieldConfig {
            id: "name".into(),
            label: "Name".into(),
            input_type: FieldType::Input(InputType::Text),
            placeholder: Some("Enter your name".into()),
            default_value: Some("Buddy".into()),
            required: true,
            options: None,
            col_span: Some(1),
        },
        InputFieldConfig {
            id: "email".into(),
            label: "Email".into(),
            input_type: FieldType::Input(InputType::Email),
            placeholder: Some("Enter your email".into()),
            default_value: None,
            required: true,
            options: None,
            col_span: Some(1),
        },
        InputFieldConfig {
            id: "password".into(),
            label: "Password".into(),
            input_type: FieldType::Input(InputType::Password),
            placeholder: Some("Create a password".into()),
            default_value: None,
            required: true,
            options: None,
            col_span: Some(2),
        },
        InputFieldConfig {
            id: "state".into(),
            label: "State".into(),
            input_type: FieldType::StateDropdown,
            placeholder: None,
            default_value: Some("CO".into()),
            required: true,
            options: None,
            col_span: Some(2),
        },
        InputFieldConfig {
            id: "favorite_color".into(),
            label: "Favorite Color".into(),
            input_type: FieldType::Select,
            placeholder: None,
            default_value: None,
            required: true,
            options: Some(vec![
                SelectOption {
                    label: "Red".into(),
                    value: "red".into(),
                },
                SelectOption {
                    label: "Blue".into(),
                    value: "blue".into(),
                },
                SelectOption {
                    label: "Green".into(),
                    value: "green".into(),
                },
            ]),
            col_span: Some(2),
        },
        InputFieldConfig {
            id: "accept_terms".into(),
            label: "Accept Terms".into(),
            input_type: FieldType::Checkbox,
            placeholder: None,
            default_value: Some("false".into()),
            required: true,
            options: None,
            col_span: Some(2),
        },
    ];

    let example_config_inline = FormBuilderConfig {
        button_label: Some("Register Inline".into()),
        error_message: None,
        success_message: None,
        inputs: base_inputs.clone(),
        modal: false,
        modal_title: None,
        auto_close_on_success: true,
    };

    let example_config_modal = FormBuilderConfig {
        button_label: Some("Register Modal".into()),
        error_message: None,
        success_message: None,
        inputs: base_inputs.clone(),
        modal: true,
        modal_title: Some("Register User".into()),
        auto_close_on_success: true,
    };

    let example_config_modal_cancel = FormBuilderConfig {
        button_label: Some("Modal with Cancel".into()),
        error_message: None,
        success_message: None,
        inputs: base_inputs,
        modal: true,
        modal_title: Some("Register (Cancel Button)".into()),
        auto_close_on_success: false, // ✨ Only this one disables auto close
    };

    let example = html! {
        <section class="max-w-4xl mx-auto p-6 bg-white dark:bg-gray-800 rounded-lg shadow-lg space-y-10">
            <Typo tag={TagType::H1}>{ "FormBuilder Component Demo" }</Typo>

            <Typo tag={TagType::H2}>{ "Inline Form" }</Typo>
            <FormBuilder
                config={example_config_inline}
                onsubmit={onsubmit.clone()}
            />

            <Typo tag={TagType::H2}>{ "Modal Form" }</Typo>
            <FormBuilder
                config={example_config_modal}
                onsubmit={onsubmit.clone()}
            />

            <Typo tag={TagType::H2}>{ "Modal Form (with Cancel Button)" }</Typo>
            <FormBuilder
                config={example_config_modal_cancel}
                onsubmit={onsubmit}
                extra_footer_buttons={extra_footer_buttons}
            />

            <div class="mt-6 text-sm text-gray-600 whitespace-pre-wrap dark:text-gray-300">
                { (*form_values).clone() }
            </div>
        </section>
    };

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec!["config", "onsubmit", "extra_footer_buttons"]
                .into_iter()
                .map(|s| html! { s })
                .collect(),
        },
        Column {
            header: "Type".into(),
            values: vec![
                "FormBuilderConfig",
                "Callback<SubmitEvent>",
                "Option<Callback<Callback<()>, Html>>",
            ]
            .into_iter()
            .map(|s| html! { s })
            .collect(),
        },
        Column {
            header: "Description".into(),
            values: vec![
                "Configuration for fields, behavior, and modal.",
                "Callback fired when form is submitted.",
                "Optional extra footer buttons rendered with submit.",
            ]
            .into_iter()
            .map(|s| html! { s })
            .collect(),
        },
    ];

    html! {
        <DemoComponent
            title="FormBuilder Component"
            description={Some(html! {
                <p>{"The `FormBuilder` allows creating fully dynamic, config-driven forms using TailYew components. It supports both inline and modal forms, customizable fields, success/error feedback, and flexible footer buttons."}</p>
            })}
            example={example}
            usage_code={USAGE_CODE}
            props_table={Some(props_table)}
        />
    }
}
