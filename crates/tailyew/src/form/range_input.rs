use crate::form::Label;
use crate::form_deserializer::*;
use crate::system::use_themed_classes;
use serde::Deserialize;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone, Default, Deserialize)]
pub struct RangeInputProps {
    #[prop_or_default]
    #[serde(default, deserialize_with = "de_attr")]
    pub id: AttrValue,
    #[prop_or_default]
    #[serde(default, deserialize_with = "de_attr")]
    pub label: AttrValue,
    #[prop_or_default]
    #[serde(default, deserialize_with = "de_attr")]
    pub default_value: AttrValue,
    #[prop_or(String::from("0"))]
    pub min: String,
    #[prop_or(String::from("100"))]
    pub max: String,
    #[prop_or(String::from("1"))]
    pub step: String,
    #[prop_or_default]
    #[serde(default, deserialize_with = "de_classes")]
    pub class: Classes,
    #[prop_or_default]
    #[serde(skip)]
    pub on_change: Option<Callback<String>>,
}

#[component(RangeInput)]
pub fn range_input(props: &RangeInputProps) -> Html {
    let RangeInputProps {
        id,
        label,
        default_value,
        min,
        max,
        step,
        class,
        on_change,
    } = props.clone();

    let value = use_state(|| default_value.clone());

    let oninput = {
        let value = value.clone();
        let on_change = on_change.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let new_value = input.value();
            value.set(new_value.clone().into());
            if let Some(cb) = &on_change {
                cb.emit(new_value);
            }
        })
    };

    let root_classes = use_themed_classes(
        "RangeInput",
        "root",
        classes!("flex", "flex-col", "space-y-2"),
        Classes::default(),
    );

    let range_defaults = classes!(
        "w-full",
        "h-2",
        "rounded-lg",
        "appearance-none",
        "cursor-pointer",
        "transition",
        "duration-150",
        "bg-gray-200",
        "focus:outline-none",
        "focus:ring-2",
        "focus:ring-green-500",
        "dark:bg-gray-700",
        "dark:focus:ring-green-400",
    );
    let range_input_classes =
        use_themed_classes("RangeInput", "input", range_defaults, class.clone());

    let value_classes = classes!("text-gray-700", "font-medium", "dark:text-gray-200");

    html! {
        <div class={root_classes}>
            <Label for_id={id.clone()} text={label.clone()} />
            <div class="flex items-center space-x-4">
                <input
                    id={id.clone()}
                    name={id.clone()}
                    type="range"
                    value={(*value).clone()}
                    min={min}
                    max={max}
                    step={step}
                    class={range_input_classes}
                    oninput={oninput}
                />
                <span class={value_classes}>{ (*value).clone() }</span>
            </div>
        </div>
    }
}
