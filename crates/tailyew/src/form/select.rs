use crate::form::Label;
use crate::form_deserializer::*;
use serde::Deserialize;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone, Deserialize)]
pub struct SelectOption {
    pub label: String,
    pub value: String,
}

#[derive(Debug, PartialEq, Clone, Default, Deserialize)]
pub enum SelectSize {
    Small,
    #[default]
    Medium,
}

#[derive(Properties, PartialEq, Clone, Default, Deserialize)]
pub struct SelectProps {
    #[prop_or_default]
    #[serde(default, deserialize_with = "de_attr")]
    pub id: AttrValue,

    #[prop_or_default]
    #[serde(default, deserialize_with = "de_option_attr")]
    pub name: Option<AttrValue>,

    #[prop_or_default]
    #[serde(default)]
    pub options: Vec<SelectOption>,

    #[prop_or_default]
    #[serde(default, deserialize_with = "de_attr")]
    pub default_value: AttrValue,

    #[prop_or_default]
    #[serde(default, deserialize_with = "de_option_attr")]
    pub value: Option<AttrValue>,

    #[prop_or_default]
    #[serde(default)]
    pub size: SelectSize,

    #[prop_or_default]
    #[serde(default, deserialize_with = "de_attr")]
    pub label: AttrValue,

    #[prop_or_default]
    #[serde(default, deserialize_with = "de_classes")]
    pub class: Classes,

    #[prop_or_default]
    #[serde(default, deserialize_with = "de_classes")]
    pub container_class: Classes,

    #[prop_or_default]
    #[serde(default, deserialize_with = "de_classes")]
    pub label_class: Classes,

    #[prop_or(false)]
    #[serde(default)]
    pub visually_hidden_label: bool,

    #[prop_or_default]
    #[serde(default, rename = "aria-label", deserialize_with = "de_option_attr")]
    pub aria_label: Option<AttrValue>,

    #[prop_or_default]
    #[serde(default, deserialize_with = "de_option_attr")]
    pub error: Option<AttrValue>,

    #[prop_or_default]
    #[serde(default)]
    pub aria_invalid: Option<bool>,

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

#[component(Select)]
pub fn select(props: &SelectProps) -> Html {
    let SelectProps {
        id,
        name,
        label,
        options,
        default_value,
        value: controlled_value,
        size,
        class,
        container_class,
        label_class,
        visually_hidden_label,
        aria_label,
        error,
        aria_invalid,
        required,
        on_change,
        disabled,
    } = props;
    let selected = use_state(|| default_value.clone());

    {
        let selected = selected.clone();
        let controlled_value = controlled_value.clone();

        use_effect_with(controlled_value, move |controlled_value| {
            if let Some(controlled_value) = controlled_value
                && *selected != *controlled_value
            {
                selected.set(controlled_value.clone());
            }
        });
    }

    let onchange = {
        let selected = selected.clone();
        let on_change = on_change.clone();
        Callback::from(move |e: Event| {
            let val = e.target_unchecked_into::<HtmlSelectElement>().value();
            selected.set(val.clone().into());
            if let Some(cb) = &on_change {
                cb.emit(val.clone());
            }
        })
    };

    let size_classes = match size {
        SelectSize::Small => "h-9 rounded-md px-3 py-0 text-sm leading-5",
        SelectSize::Medium => "h-10 rounded-lg px-4 py-0 text-sm leading-5",
    };
    let effective_error = error
        .clone()
        .map(|error| error.to_string())
        .filter(|error| !error.is_empty());
    let error_id = effective_error
        .as_ref()
        .filter(|_| !id.is_empty())
        .map(|_| AttrValue::from(format!("{id}-error")));
    let effective_aria_invalid = aria_invalid.unwrap_or(effective_error.is_some());
    let name_attr = name.clone().unwrap_or_else(|| id.clone());

    let select_classes = classes!(
        "w-full",
        "box-border",
        "border",
        "border-gray-300",
        "bg-white",
        "shadow-sm",
        "transition",
        "duration-150",
        "focus:outline-none",
        "focus:ring-2",
        "focus:ring-primary",
        "focus:border-primary",
        "disabled:cursor-not-allowed",
        "disabled:bg-gray-100",
        "disabled:text-gray-500",
        "dark:bg-gray-800",
        "dark:border-gray-600",
        "dark:text-gray-200",
        "dark:focus:ring-primary-dark",
        "dark:focus:border-primary-dark",
        "dark:disabled:bg-gray-700",
        size_classes,
        effective_error.is_some().then_some("border-red-500"),
        class.clone()
    );

    html! {
        <div class={classes!("flex", "flex-col", container_class.clone())}>
            // only render a label if it's non-empty
            { if !label.is_empty() {
                html! {
                    <Label
                        for_id={id.clone()}
                        text={label.clone()}
                        required={*required}
                        class={classes!(visually_hidden_label.then_some("sr-only"), label_class.clone())}
                    />
                }
            } else {
                html!{}
            }}

            <select
                id={id.clone()}
                name={name_attr}
                class={select_classes}
                onchange={onchange}
                value={controlled_value.clone().unwrap_or_else(|| (*selected).clone())}
                required={*required}
                disabled={*disabled}
                aria-label={aria_label.clone()}
                aria-invalid={AttrValue::from(effective_aria_invalid.to_string())}
                aria-describedby={error_id.clone()}
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
            if let Some(error) = effective_error {
                <p id={error_id} class="mt-1 text-xs font-medium text-red-600 dark:text-red-300">
                    { error }
                </p>
            }
        </div>
    }
}
