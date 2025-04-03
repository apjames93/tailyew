use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct RadioGroupProps {
    pub id: String,
    pub label: String,
    pub options: Vec<(String, String)>, // (value, label)
    #[prop_or_default]
    pub default_value: String,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub on_change: Option<Callback<String>>,
}

#[function_component(RadioGroup)]
pub fn radio_group(props: &RadioGroupProps) -> Html {
    let RadioGroupProps {
        id,
        label,
        options,
        default_value,
        class,
        on_change,
    } = props.clone();

    let selected_value = use_state(|| default_value.clone());

    let onchange = {
        let selected_value = selected_value.clone();
        let on_change = on_change.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let new_value = input.value();
            selected_value.set(new_value.clone());

            if let Some(cb) = &on_change {
                cb.emit(new_value);
            }
        })
    };

    let container_classes = classes!("flex", "flex-col", "space-y-4", class);

    let label_classes = classes!(
        "text-lg",
        "font-semibold",
        "text-gray-700",
        "dark:text-gray-300"
    );

    let radio_item_classes = classes!("flex", "items-center", "space-x-2");

    let radio_input_classes = classes!(
        "h-4",
        "w-4",
        "text-primary",
        "border-gray-300",
        "focus:ring-2",
        "focus:ring-primary",
        "dark:text-primary-dark",
        "dark:border-gray-600",
        "dark:focus:ring-primary-dark"
    );

    let option_label_classes = classes!("text-gray-700", "dark:text-gray-400");

    html! {
        <div class={container_classes}>
            <label class={label_classes}>{ label }</label>
            <div class="flex flex-col space-y-2">
                { for options.iter().map(|(value, option_label)| {
                    html! {
                        <div class={radio_item_classes.clone()}>
                            <input
                                type="radio"
                                id={format!("{}-{}", id, value)}
                                name={id.clone()}
                                value={value.clone()}
                                checked={*selected_value == *value}
                                aria-checked={(*selected_value == *value).to_string()}
                                onchange={onchange.clone()}
                                class={radio_input_classes.clone()}
                            />
                            <label
                                for={format!("{}-{}", id, value)}
                                class={option_label_classes.clone()}
                            >
                                { option_label }
                            </label>
                        </div>
                    }
                })}
            </div>
        </div>
    }
}
