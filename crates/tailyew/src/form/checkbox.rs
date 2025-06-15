use crate::form_deserializer::{de_attr, de_option_attr};
use serde::Deserialize;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone, Default, Deserialize)]
pub struct CheckboxProps {
    /// the field’s ID/name
    #[prop_or_default]
    #[serde(default, deserialize_with = "de_attr")]
    pub id: AttrValue,

    /// the visible label
    #[prop_or_default]
    #[serde(default, deserialize_with = "de_attr")]
    pub label: AttrValue,

    /// initial checked state
    #[prop_or(false)]
    #[serde(default)]
    pub checked: bool,

    /// mark as required
    #[prop_or(false)]
    #[serde(default)]
    pub required: bool,

    /// optional helper text under the checkbox
    #[prop_or_default]
    #[serde(default, deserialize_with = "de_option_attr")]
    pub description: Option<AttrValue>,

    /// disable interaction
    #[prop_or(false)]
    #[serde(default)]
    pub disabled: bool,

    /// programmatic callback (skipped in serde)
    #[prop_or_default]
    #[serde(skip)]
    pub on_change: Option<Callback<bool>>,
}

#[function_component(Checkbox)]
pub fn checkbox(props: &CheckboxProps) -> Html {
    let CheckboxProps {
        id,
        label,
        checked,
        required,
        description,
        disabled,
        on_change,
    } = props.clone();

    let handle_change = {
        let on_change = on_change.clone();
        Callback::from(move |e: Event| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            if let Some(cb) = &on_change {
                cb.emit(input.checked());
            }
        })
    };

    // build all your classes as before
    let checkbox_classes = classes!(
        "h-4",
        "w-4",
        "border-2",
        "rounded",
        "focus:ring-2",
        "transition",
        "duration-150",
        "cursor-pointer",
        "outline-none",
        if checked {
            "bg-primary border-primary text-white focus:ring-primary \
             dark:bg-primary-dark dark:border-primary-dark dark:focus:ring-primary-dark"
        } else {
            "bg-white border-gray-300 text-gray-900 focus:ring-primary \
             dark:bg-gray-800 dark:border-gray-600 dark:text-gray-400 dark:focus:ring-primary-dark"
        },
        if disabled {
            "opacity-60 cursor-not-allowed"
        } else {
            ""
        },
    );

    let label_classes = classes!(
        "text-lg",
        "ml-2",
        "cursor-pointer",
        "transition",
        "duration-150",
        if checked {
            "text-gray-900 dark:text-gray-300"
        } else {
            "text-gray-700 dark:text-gray-400"
        },
        if disabled { "opacity-70" } else { "" },
    );

    html! {
        <div class="flex flex-col space-y-1">
            <div class="flex items-center">
                <input
                    id={id.clone()}
                    name={id.clone()}
                    type="checkbox"
                    checked={checked}
                    required={required}
                    disabled={disabled}
                    class={checkbox_classes}
                    onchange={handle_change}
                />
                <label for={id.clone()} class={label_classes}>
                    { label.clone() }
                </label>
            </div>
            {
                if let Some(desc) = &description {
                    html! {
                        <p class="text-sm mt-1 ml-6 text-gray-500 dark:text-gray-400">
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
