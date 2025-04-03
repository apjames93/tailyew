use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct RangeInputProps {
    pub id: String,
    pub label: String,
    #[prop_or_default]
    pub default_value: String,
    #[prop_or(String::from("0"))]
    pub min: String,
    #[prop_or(String::from("100"))]
    pub max: String,
    #[prop_or(String::from("1"))]
    pub step: String,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub on_change: Option<Callback<String>>,
}

#[function_component(RangeInput)]
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
            value.set(new_value.clone());
            if let Some(cb) = &on_change {
                cb.emit(new_value);
            }
        })
    };

    let range_input_classes = classes!(
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
        class
    );

    let value_classes = classes!("text-gray-700", "font-medium", "dark:text-gray-200");

    html! {
        <div class="flex flex-col space-y-2">
            <label for={id.clone()} class="text-lg font-semibold text-gray-700 dark:text-gray-200">
                { label }
            </label>
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
