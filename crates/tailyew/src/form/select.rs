use crate::form_deserializer::*;
use serde::Deserialize;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone, Deserialize)]
pub struct SelectOption {
    pub label: String,
    pub value: String,
}

#[derive(Properties, PartialEq, Clone, Default, Deserialize)]
pub struct SelectProps {
    #[prop_or_default]
    #[serde(default, deserialize_with = "de_attr")]
    pub id: AttrValue,

    // plain Vec<SelectOption> – no custom deserializer here
    #[prop_or_default]
    #[serde(default)]
    pub options: Vec<SelectOption>,

    #[prop_or_default]
    #[serde(default, deserialize_with = "de_attr")]
    pub default_value: AttrValue,

    #[prop_or_default]
    #[serde(default, deserialize_with = "de_attr")]
    pub label: AttrValue,

    #[prop_or_default]
    #[serde(default, deserialize_with = "de_classes")]
    pub class: Classes,

    #[prop_or(true)]
    #[serde(default)]
    pub required: bool,

    #[prop_or_default]
    #[serde(skip)]
    pub on_change: Option<Callback<String>>,
    #[prop_or(false)]
    #[serde(default)]
    pub disabled: bool,
}

#[function_component(Select)]
pub fn select(props: &SelectProps) -> Html {
    let SelectProps {
        id,
        label,
        options,
        default_value,
        class,
        required,
        on_change,
        disabled,
    } = props;
    let selected = use_state(|| default_value.clone());

    let onchange = {
        let selected = selected.clone();
        let on_change = on_change.clone();
        Callback::from(move |e: Event| {
            let val = e.target_unchecked_into::<HtmlInputElement>().value();
            selected.set(val.clone().into());
            if let Some(cb) = &on_change {
                cb.emit(val.clone());
            }
        })
    };

    let select_classes = classes!(
        "w-full",
        "px-4",
        "py-2",
        "border",
        "rounded-lg",
        "shadow-sm",
        "transition",
        "duration-150",
        "focus:outline-none",
        "focus:ring-2",
        "focus:ring-green-500",
        "focus:border-green-500",
        "dark:bg-gray-700",
        "dark:border-gray-600",
        "dark:text-gray-200",
        "dark:focus:ring-green-400",
        class.clone()
    );

    html! {
        <div class="flex flex-col space-y-2">
            // only render a label if it's non-empty
            { if !label.is_empty() {
                html! {
                    <label for={id.clone()}
                           class="text-lg font-semibold text-gray-700 dark:text-gray-200">
                        { label.clone() }
                    </label>
                }
            } else {
                html!{}
            }}

            <select
                id={id.clone()}
                class={select_classes}
                onchange={onchange}
                value={(*selected).clone()}
                required={*required}
                disabled={*disabled}
            >
                <option
                    value=""
                    disabled=true
                    selected={selected.is_empty()}
                    class="text-gray-700 dark:text-gray-300"
                >
                    { "Please select an option" }
                </option>
                { for options.iter().map(|opt| html! {
                    <option
                        value={opt.value.clone()}
                        selected={opt.value == *selected}
                        class="text-gray-700 dark:text-gray-300"
                    >
                        { opt.label.clone() }
                    </option>
                }) }
            </select>
        </div>
    }
}
