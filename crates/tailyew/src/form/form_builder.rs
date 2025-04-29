use crate::{form::*, ButtonType, ModalSize};
use serde::Deserialize;
use web_sys::SubmitEvent;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties, Deserialize)]
pub struct FormBuilderConfig {
    #[serde(default)]
    pub button_label: Option<String>,

    #[serde(default)]
    pub error_message: Option<String>,

    #[serde(default)]
    pub success_message: Option<String>,

    #[serde(default)]
    pub inputs: Vec<InputFieldConfig>,

    #[serde(default)]
    pub modal: bool,

    #[serde(default)]
    pub modal_title: Option<String>,

    #[serde(default)]
    #[prop_or(true)]
    pub auto_close_on_success: bool,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct InputFieldConfig {
    pub id: String,
    pub label: String,
    pub input_type: FieldType,
    pub placeholder: Option<String>,
    pub default_value: Option<String>,
    pub required: bool,
    pub options: Option<Vec<SelectOption>>, // for select or radio group
    pub col_span: Option<u8>,               // 1 or 2
}

#[derive(Clone, PartialEq, Deserialize)]
pub enum FieldType {
    Input(InputType),
    Textarea,
    Select,
    RadioGroup,
    Checkbox,
    ColorInput,
    FileInput,
    PhoneInput,
    RangeInput,
    StateDropdown,
}

#[derive(Properties, PartialEq, Clone)]
pub struct FormBuilderProps {
    pub config: FormBuilderConfig,
    pub onsubmit: Callback<SubmitEvent>,
    #[prop_or_default]
    pub extra_footer_buttons: Option<Callback<Callback<()>, Html>>,
}

#[function_component(FormBuilder)]
pub fn form_builder(props: &FormBuilderProps) -> Html {
    let FormBuilderProps {
        config,
        onsubmit,
        extra_footer_buttons,
    } = props;

    let button_label = config
        .button_label
        .clone()
        .unwrap_or_else(|| "Submit".to_string());
    let modal_title = config
        .modal_title
        .clone()
        .unwrap_or_else(|| button_label.clone());

    let success_message = use_state(|| None::<String>); // dynamic success

    let internal_onsubmit = {
        let success_message = success_message.clone();
        let onsubmit = onsubmit.clone();
        Callback::from(move |e: SubmitEvent| {
            onsubmit.emit(e);
            success_message.set(Some("Form submitted successfully.".to_string()));
            // ✨ dynamic success
        })
    };

    let form_content = html! {
        <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
            { for config.inputs.iter().map(render_field) }
        </div>
    };

    if config.modal {
        html! {
            <FormModal
                modal_button={ModalButtonConfig {
                    button_text: button_label.clone(),
                    button_type: ButtonType::Primary,
                    modal_title,
                    modal_size: ModalSize::Large,
                    is_open: false,
                    on_modal_close: None,
                }}
                onsubmit={internal_onsubmit}
                loading={false}
                error_message={config.error_message.clone()}
                success_message={(*success_message).clone()}
                submit_label={button_label}
                extra_footer_buttons={extra_footer_buttons.clone()}
                auto_close_on_success={config.auto_close_on_success}
            >
                { form_content }
            </FormModal>
        }
    } else {
        html! {
            <Form
                onsubmit_callback={internal_onsubmit}
                button_label={button_label}
                error_message={config.error_message.clone()}
                success_message={(*success_message).clone()}
                extra_footer_buttons={extra_footer_buttons.clone()}
            >
                { form_content }
            </Form>
        }
    }
}

fn render_field(field: &InputFieldConfig) -> Html {
    let col_span = match field.col_span.unwrap_or(2) {
        1 => "col-span-1",
        _ => "col-span-2",
    };

    html! {
        <div class={classes!(col_span)}>
            {
                match &field.input_type {
                    FieldType::Input(input_type) => html! {
                        <Input
                            id={field.id.clone()}
                            label={field.label.clone()}
                            placeholder={field.placeholder.clone().unwrap_or_default()}
                            input_type={input_type.clone()}
                            default_value={field.default_value.clone().unwrap_or_default()}
                            required={field.required}
                        />
                    },
                    FieldType::Textarea => html! {
                        <Textarea
                            id={field.id.clone()}
                            label={field.label.clone()}
                            placeholder={field.placeholder.clone().unwrap_or_default()}
                            default_value={field.default_value.clone().unwrap_or_default()}
                            required={field.required}
                        />
                    },
                    FieldType::Select => html! {
                        <Select
                            id={field.id.clone()}
                            options={field.options.clone().unwrap_or_default()}
                            default_value={field.default_value.clone().unwrap_or_default()}
                            required={field.required}
                        />
                    },
                    FieldType::RadioGroup => html! {
                        <RadioGroup
                            id={field.id.clone()}
                            label={field.label.clone()}
                            options={field.options.clone().unwrap_or_default()
                                .into_iter()
                                .map(|o| (o.value, o.label))
                                .collect::<Vec<(String, String)>>()}
                            default_value={field.default_value.clone().unwrap_or_default()}
                        />
                    },
                    FieldType::Checkbox => html! {
                        <Checkbox
                            id={field.id.clone()}
                            label={field.label.clone()}
                            checked={field.default_value.clone().unwrap_or_default() == "true"}
                            required={field.required}
                        />
                    },
                    FieldType::ColorInput => html! {
                        <ColorInput
                            id={field.id.clone()}
                            label={field.label.clone()}
                            value={field.default_value.clone().unwrap_or("#000000".to_string())}
                        />
                    },
                    FieldType::FileInput => html! {
                        <FileInput
                            id={field.id.clone()}
                            label={field.label.clone()}
                        />
                    },
                    FieldType::PhoneInput => html! {
                        <PhoneInput
                            id={field.id.clone()}
                            label={field.label.clone()}
                            placeholder={field.placeholder.clone().unwrap_or("123-456-7890".to_string())}
                            default_value={field.default_value.clone().unwrap_or_default()}
                        />
                    },
                    FieldType::RangeInput => html! {
                        <RangeInput
                            id={field.id.clone()}
                            label={field.label.clone()}
                            default_value={field.default_value.clone().unwrap_or("50".to_string())}
                        />
                    },
                    FieldType::StateDropdown => html! {
                        <StateDropdown
                            id={field.id.clone()}
                            default_value={field.default_value.clone().unwrap_or("CO".to_string())}
                        />
                    },
                }
            }
        </div>
    }
}
