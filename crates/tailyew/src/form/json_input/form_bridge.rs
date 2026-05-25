use crate::form::submitted_name;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub(crate) struct JsonBackedHiddenInputProps {
    pub id: AttrValue,
    #[prop_or_default]
    pub name: Option<AttrValue>,
    pub value: AttrValue,
}

#[component(JsonBackedHiddenInput)]
pub(crate) fn json_backed_hidden_input(props: &JsonBackedHiddenInputProps) -> Html {
    // This is the named payload control. It stays separate from the validity
    // proxy so FormData receives only the raw JSON field value.
    html! {
        <input
            type="hidden"
            id={props.id.clone()}
            name={submitted_name(&props.id, &props.name)}
            value={props.value.clone()}
        />
    }
}

#[derive(Properties, PartialEq, Clone)]
pub(crate) struct JsonFormValidityBridgeProps {
    pub id: AttrValue,
    pub label: AttrValue,
    pub is_valid: bool,
    #[prop_or_default]
    pub validation_message: Option<AttrValue>,
    #[prop_or_default]
    pub on_validation_requested: Option<Callback<()>>,
}

#[component(JsonFormValidityBridge)]
pub(crate) fn json_form_validity_bridge(props: &JsonFormValidityBridgeProps) -> Html {
    let input_ref = use_node_ref();
    let validation_message =
        validation_bridge_message(&props.label, props.validation_message.as_ref());
    let aria_label = validation_bridge_aria_label(&props.label);

    {
        let input_ref = input_ref.clone();
        let is_valid = props.is_valid;
        let validation_message = validation_message.clone();

        use_effect_with(
            (is_valid, validation_message),
            move |(is_valid, message)| {
                if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                    input.set_custom_validity(if *is_valid { "" } else { message.as_str() });
                }
            },
        );
    }

    let on_invalid = {
        let on_validation_requested = props.on_validation_requested.clone();

        Callback::from(move |event: Event| {
            event.prevent_default();
            if let Some(on_validation_requested) = &on_validation_requested {
                on_validation_requested.emit(());
            }
        })
    };

    html! {
        // This unnamed proxy participates in native constraint validation but
        // never appears in submitted form data.
        <input
            ref={input_ref}
            id={validation_bridge_id(&props.id)}
            type="text"
            class="sr-only"
            tabindex="-1"
            value="valid"
            aria-label={aria_label}
            oninvalid={on_invalid}
        />
    }
}

pub(crate) fn validation_bridge_id(id: &AttrValue) -> AttrValue {
    AttrValue::from(format!("{}__validity", id.as_str()))
}

pub(crate) fn validation_bridge_message(
    label: &AttrValue,
    validation_message: Option<&AttrValue>,
) -> AttrValue {
    validation_message.cloned().unwrap_or_else(|| {
        let label = label.as_str().trim();
        if label.is_empty() {
            AttrValue::from("Fix validation errors before submitting.")
        } else {
            AttrValue::from(format!(
                "Fix validation errors in {label} before submitting."
            ))
        }
    })
}

pub(crate) fn validation_bridge_aria_label(label: &AttrValue) -> AttrValue {
    let label = label.as_str().trim();
    if label.is_empty() {
        AttrValue::from("JSON field validation status")
    } else {
        AttrValue::from(format!("{label} validation status"))
    }
}
