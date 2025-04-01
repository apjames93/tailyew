use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct SelectOption {
    pub label: String,
    pub value: String,
}

#[derive(Properties, PartialEq, Clone)]
pub struct SelectProps {
    pub id: String,
    pub options: Vec<SelectOption>,
    #[prop_or_default]
    pub default_value: String,
    #[prop_or_default]
    pub label: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or(true)]
    pub required: bool,
    #[prop_or_default]
    pub on_change: Option<Callback<String>>,
    #[prop_or(false)]
    pub disabled: bool,
}

#[function_component(Select)]
pub fn select(props: &SelectProps) -> Html {
    let SelectProps {
        class,
        default_value,
        id,
        label,
        options,
        required,
        on_change,
        disabled,
    } = props;

    let selected_value = use_state(|| default_value.clone());

    let onchange = {
        let selected_value = selected_value.clone();
        let on_change = on_change.clone();
        Callback::from(move |e: Event| {
            let select_element = e.target_unchecked_into::<HtmlInputElement>();
            let value = select_element.value();
            selected_value.set(value.clone());

            // Call the on_change callback if it is provided
            if let Some(on_change_callback) = &on_change {
                on_change_callback.emit(value);
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
            { label.as_ref().map(|label_text| html! {
                <label for={id.clone()} class="text-lg font-semibold text-gray-700 dark:text-gray-200">{ label_text }</label>
            }) }
            <select
                id={id.clone()}
                class={select_classes}
                onchange={onchange}
                value={(*selected_value).clone()}
                required={*required}
                disabled={*disabled}
            >
                <option selected={selected_value.is_empty()}  value="" disabled={true} class="text-gray-700 dark:text-gray-300">{ "Please select an option" }</option>
                { for options.iter().map(|option| {
                    html! {
                        <option
                            value={option.value.clone()}
                            class="text-gray-700 dark:text-gray-300"
                            selected={option.value == *selected_value}
                        >
                            { option.label.clone() }
                        </option>
                    }
                }) }
            </select>
        </div>
    }
}
