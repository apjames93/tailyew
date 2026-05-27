use crate::form::{Label, join_aria_ids};
use crate::form_deserializer::{de_attr, de_option_attr};
use serde::Deserialize;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone, Default, Deserialize)]
pub struct SwitchProps {
    /// the field’s ID/name
    #[prop_or_default]
    #[serde(default, deserialize_with = "de_attr")]
    pub id: AttrValue,

    /// submitted form field name; defaults to id
    #[prop_or_default]
    #[serde(default, deserialize_with = "de_option_attr")]
    pub name: Option<AttrValue>,

    /// the visible label
    #[prop_or_default]
    #[serde(default, deserialize_with = "de_attr")]
    pub label: AttrValue,

    /// initial on/off state (acts like a default value)
    #[prop_or(false)]
    #[serde(default)]
    pub checked: bool,

    /// mark as required (for HTML5 validation)
    #[prop_or(false)]
    #[serde(default)]
    pub required: bool,

    /// optional helper text under the switch
    #[prop_or_default]
    #[serde(default, deserialize_with = "de_option_attr")]
    pub description: Option<AttrValue>,

    /// preferred helper text alias; falls back to description when omitted
    #[prop_or_default]
    #[serde(default, deserialize_with = "de_option_attr")]
    pub helper_text: Option<AttrValue>,

    /// external error message
    #[prop_or_default]
    #[serde(default, deserialize_with = "de_option_attr")]
    pub error: Option<AttrValue>,

    /// hide the visible label while preserving it for screen readers
    #[prop_or(false)]
    #[serde(default)]
    pub visually_hidden_label: bool,

    /// disable interaction
    #[prop_or(false)]
    #[serde(default)]
    pub disabled: bool,

    /// ARIA attributes
    #[prop_or_default]
    #[serde(
        default,
        rename = "aria-describedby",
        deserialize_with = "de_option_attr"
    )]
    pub aria_describedby: Option<AttrValue>,

    #[prop_or_default]
    #[serde(default)]
    pub aria_invalid: Option<bool>,

    #[prop_or_default]
    #[serde(default, rename = "aria-label", deserialize_with = "de_option_attr")]
    pub aria_label: Option<AttrValue>,

    #[prop_or_default]
    #[serde(
        default,
        rename = "aria-labelledby",
        deserialize_with = "de_option_attr"
    )]
    pub aria_labelledby: Option<AttrValue>,

    /// programmatic callback (skipped in serde)
    #[prop_or_default]
    #[serde(skip)]
    pub on_change: Option<Callback<bool>>,

    #[prop_or_default]
    #[serde(skip)]
    pub on_blur: Option<Callback<FocusEvent>>,
}

#[component(Switch)]
pub fn switch(props: &SwitchProps) -> Html {
    let SwitchProps {
        id,
        name,
        label,
        checked,
        required,
        description,
        helper_text,
        error,
        visually_hidden_label,
        disabled,
        aria_describedby,
        aria_invalid,
        aria_label,
        aria_labelledby,
        on_change,
        on_blur,
    } = props.clone();

    // Internal checked state, seeded from the prop
    let is_checked = use_state(|| checked);

    {
        let is_checked = is_checked.clone();
        use_effect_with(checked, move |checked| {
            is_checked.set(*checked);
        });
    }

    let handle_change = {
        let is_checked = is_checked.clone();
        let on_change = on_change.clone();

        Callback::from(move |e: Event| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            let value = input.checked();

            // update internal state so the UI re-renders
            is_checked.set(value);

            // notify parent if they care
            if let Some(cb) = &on_change {
                cb.emit(value);
            }
        })
    };

    let current = *is_checked;

    // Useful ids for a11y wiring
    let label_id = format!("{}-label", id);
    let helper_id = format!("{}-helper", id);
    let error_id = format!("{}-error", id);
    let helper_text = helper_text.or(description);
    let effective_error = error
        .clone()
        .map(|error| error.to_string())
        .filter(|error| !error.is_empty());

    let effective_aria_label = aria_label.unwrap_or_else(|| label.clone());
    let effective_aria_labelledby =
        aria_labelledby.unwrap_or_else(|| AttrValue::from(label_id.clone()));
    let effective_aria_describedby = join_aria_ids(vec![
        aria_describedby,
        helper_text
            .as_ref()
            .map(|_| AttrValue::from(helper_id.clone())),
        effective_error
            .as_ref()
            .map(|_| AttrValue::from(error_id.clone())),
    ]);
    let effective_aria_invalid = aria_invalid.unwrap_or(effective_error.is_some());
    let name_attr = name.unwrap_or_else(|| id.clone());

    // Track (background) classes
    let track_classes = classes!(
        "relative",
        "inline-flex",
        "h-6",
        "w-11",
        "flex-shrink-0",
        "cursor-pointer",
        "rounded-full",
        "border-2",
        "border-transparent",
        "transition-colors",
        "duration-200",
        "ease-in-out",
        if current {
            "bg-primary dark:bg-primary-dark"
        } else {
            "bg-gray-200 dark:bg-gray-700"
        },
        if disabled {
            "opacity-60 cursor-not-allowed"
        } else {
            ""
        },
    );

    // Thumb (circle) classes
    let thumb_classes = classes!(
        "pointer-events-none",
        "inline-block",
        "h-5",
        "w-5",
        "rounded-full",
        "bg-white",
        "shadow",
        "transform",
        "ring-0",
        "transition",
        "duration-200",
        "ease-in-out",
        if current {
            "translate-x-5"
        } else {
            "translate-x-0"
        },
    );

    let label_classes = classes!(
        "ml-3",
        "cursor-pointer",
        "transition",
        "duration-150",
        visually_hidden_label.then_some("sr-only"),
        if current {
            "text-gray-900 dark:text-gray-300"
        } else {
            "text-gray-700 dark:text-gray-400"
        },
        if disabled { "opacity-70" } else { "" },
    );

    html! {
        <div class="flex flex-col space-y-1">
            <div class="flex items-center">
                // Real checkbox for forms & accessibility
                <input
                    id={id.clone()}
                    name={name_attr}
                    type="checkbox"
                    checked={current}
                    required={required}
                    disabled={disabled}
                    class="sr-only"
                    onchange={handle_change}
                    aria-invalid={AttrValue::from(effective_aria_invalid.to_string())}
                    aria-describedby={effective_aria_describedby.clone()}
                />

                // Visual switch (primary control for keyboard/screen readers)
                <button
                    type="button"
                    role="switch"
                    aria-checked={current.to_string()}
                    aria-disabled={disabled.then_some("true")}
                    aria-required={AttrValue::from(required.to_string())}
                    aria-invalid={AttrValue::from(effective_aria_invalid.to_string())}
                    aria-label={effective_aria_label}
                    aria-labelledby={effective_aria_labelledby.clone()}
                    aria-describedby={effective_aria_describedby.clone()}
                    class={track_classes}
                    onblur={on_blur}
                    onclick={if disabled {
                        Callback::from(|_: MouseEvent| {})
                    } else {
                        // Simulate toggle by dispatching a click on the hidden input
                        let id = id.clone();
                        Callback::from(move |_: MouseEvent| {
                            if let Some(window) = web_sys::window()
                                && let Some(document) = window.document()
                                    && let Some(elem) = document.get_element_by_id(&id)
                                        && let Ok(input) = elem.dyn_into::<HtmlInputElement>() {
                                            // this triggers `onchange`, which updates internal state
                                            input.click();
                                        }
                        })
                    }}
                >
                    <span class={thumb_classes} />
                </button>

                <Label
                    id={Some(AttrValue::from(label_id))}
                    for_id={id.clone()}
                    text={label.clone()}
                    required={required}
                    class={label_classes}
                />
            </div>
            if let Some(helper_text) = &helper_text {
                <p
                    id={helper_id}
                    class="mt-1 ml-14 text-sm text-gray-500 dark:text-gray-400"
                >
                    { helper_text.clone() }
                </p>
            }
            if let Some(error) = effective_error {
                <p
                    id={error_id}
                    class="mt-1 ml-14 text-sm font-medium text-red-600 dark:text-red-300"
                    role="alert"
                >
                    { error }
                </p>
            }
        </div>
    }
}
