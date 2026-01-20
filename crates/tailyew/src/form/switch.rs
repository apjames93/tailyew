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
}

#[component(Switch)]
pub fn switch(props: &SwitchProps) -> Html {
    let SwitchProps {
        id,
        label,
        checked,
        required,
        description,
        disabled,
        aria_describedby,
        aria_label,
        aria_labelledby,
        on_change,
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
    let description_id = format!("{}-description", id);

    let effective_aria_label = aria_label.unwrap_or_else(|| label.clone());
    let effective_aria_labelledby =
        aria_labelledby.unwrap_or_else(|| AttrValue::from(label_id.clone()));
    let effective_aria_describedby = aria_describedby.or_else(|| {
        if description.is_some() {
            Some(AttrValue::from(description_id.clone()))
        } else {
            None
        }
    });

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
        "text-lg",
        "ml-3",
        "cursor-pointer",
        "transition",
        "duration-150",
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
                    name={id.clone()}
                    type="checkbox"
                    checked={current}
                    required={required}
                    disabled={disabled}
                    class="sr-only"
                    onchange={handle_change}
                />

                // Visual switch (primary control for keyboard/screen readers)
                <button
                    type="button"
                    role="switch"
                    aria-checked={current.to_string()}
                    aria-disabled={disabled.then_some("true")}
                    aria-required={AttrValue::from(required.to_string())}
                    aria-label={effective_aria_label}
                    aria-labelledby={effective_aria_labelledby.clone()}
                    aria-describedby={effective_aria_describedby.clone()}
                    class={track_classes}
                    onclick={if disabled {
                        Callback::from(|_: MouseEvent| {})
                    } else {
                        // Simulate toggle by dispatching a click on the hidden input
                        let id = id.clone();
                        Callback::from(move |_: MouseEvent| {
                            if let Some(window) = web_sys::window() {
                                if let Some(document) = window.document() {
                                    if let Some(elem) = document.get_element_by_id(&id) {
                                        if let Ok(input) = elem.dyn_into::<HtmlInputElement>() {
                                            // this triggers `onchange`, which updates internal state
                                            input.click();
                                        }
                                    }
                                }
                            }
                        })
                    }}
                >
                    <span class={thumb_classes} />
                </button>

                <label
                    id={label_id}
                    for={id.clone()}
                    class={label_classes}
                >
                    { label.clone() }
                </label>
            </div>
            {
                if let Some(desc) = &description {
                    html! {
                        <p
                            id={description_id}
                            class="text-sm mt-1 ml-14 text-gray-500 dark:text-gray-400"
                        >
                            { desc.clone() }
                        </p>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}
