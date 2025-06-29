use crate::{
    form::{Form, FormSubmitCallback},
    FormModal, ModalButtonConfig,
};
use yew::prelude::*;
pub mod render_field;
pub use render_field::*;

/// Only the modal‐specific bits stay here:
#[derive(Properties, PartialEq, Clone)]
pub struct ModalConfig {
    /// the button that opens the modal
    pub modal_button: ModalButtonConfig,
    /// auto-close on success?
    #[prop_or(true)]
    pub auto_close_on_success: bool,
    /// callback when the form inside the modal succeeds
    #[prop_or_default]
    pub on_success: Option<Callback<()>>,
    /// callback when the form inside the modal errors
    #[prop_or_default]
    pub on_error: Option<Callback<()>>,
}
#[derive(Properties, PartialEq, Clone)]
pub struct FormBuilderProps {
    /// your submit handler
    pub onsubmit: FormSubmitCallback,

    /// any extra footer buttons you want
    #[prop_or_default]
    pub extra_footer_buttons: Option<Callback<Callback<()>, Html>>,

    /// if present, will render inside a modal
    #[prop_or_default]
    pub modal_config: Option<ModalConfig>,

    /// text for your submit button (inline & modal)
    #[prop_or_default]
    pub button_label: Option<String>,

    /// the full list of fields to render
    pub inputs: Vec<RenderFieldProps>,

    /// grid container classes (default: 2-col)
    #[prop_or("grid grid-cols-1 sm:grid-cols-2 gap-4".into())]
    pub container_class: Classes,

    /// classes applied to *every* input wrapper
    #[prop_or("col-span-1".into())]
    pub input_class: Classes,
    #[prop_or_default]
    pub disabled: bool,
}

#[function_component(FormBuilder)]
pub fn form_builder(props: &FormBuilderProps) -> Html {
    let FormBuilderProps {
        onsubmit,
        extra_footer_buttons,
        modal_config,
        button_label,
        inputs,
        container_class,
        input_class,
        disabled,
    } = props.clone();

    let submit_text = button_label.unwrap_or_else(|| "Submit".into());

    // build all the <RenderField/>s, merging per-field + global classes
    let form_fields = html! {
        <div class={container_class}>
            { for inputs.into_iter().map(|rf| {
                // combine the global `input_class` with any per-field `rf.class`
                let wrapper = classes!(input_class.clone(), rf.class.clone());
                html! {
                    <div class={wrapper}>
                        <RenderField ..rf />
                    </div>
                }
            }) }
        </div>
    };

    if let Some(mc) = modal_config {
        html! {
            <FormModal
                modal_button={mc.modal_button}
                onsubmit={onsubmit}
                submit_label={submit_text.clone()}
                extra_footer_buttons={extra_footer_buttons.clone()}
                auto_close_on_success={mc.auto_close_on_success}
                on_success={mc.on_success}
                on_error={mc.on_error}
                disabled={disabled}
            >
                { form_fields }
            </FormModal>
        }
    } else {
        html! {
            <Form
                onsubmit_callback={onsubmit}
                button_label={submit_text.clone()}
                extra_footer_buttons={extra_footer_buttons.clone()}
                disabled={disabled}
            >
                { form_fields }
            </Form>
        }
    }
}
