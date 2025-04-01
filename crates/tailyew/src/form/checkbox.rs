use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct CheckboxProps {
    pub id: String,
    pub label: String,
    #[prop_or(false)]
    pub checked: bool, // Default value is false if not provided
    #[prop_or_default]
    pub description: Option<String>, // Optional description text below the label
    #[prop_or(false)]
    pub disabled: bool,
    #[prop_or_default]
    pub on_change: Option<Callback<bool>>, // Callback when the checkbox state changes
}

#[function_component(Checkbox)]
pub fn checkbox(props: &CheckboxProps) -> Html {
    let CheckboxProps {
        id,
        label,
        checked,
        description,
        disabled,
        on_change,
    } = props.clone();

    let state = use_state(|| checked);

    let onchange = {
        let state = state.clone();
        let on_change = on_change.clone();
        Callback::from(move |_| {
            let new_state = !*state;
            state.set(new_state);
            if let Some(callback) = &on_change {
                callback.emit(new_state);
            }
        })
    };

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
        if *state {
            "bg-primary border-primary text-white focus:ring-primary dark:bg-primary-dark dark:border-primary-dark dark:focus:ring-primary-dark"
        } else {
            "bg-white border-gray-300 text-gray-900 focus:ring-primary dark:bg-gray-800 dark:border-gray-600 dark:text-gray-400 dark:focus:ring-primary-dark"
        },
    );

    let label_classes = classes!(
        "text-lg",
        "ml-2",
        "cursor-pointer",
        "transition",
        "duration-150",
        if *state {
            "text-gray-900 dark:text-gray-300"
        } else {
            "text-gray-700 dark:text-gray-400"
        }
    );

    let description_classes = classes!(
        "text-sm",
        "mt-1",
        "ml-2",
        "text-gray-500",
        "dark:text-gray-400",
    );

    html! {
        <div class="flex flex-col space-y-1">
            <div class="flex items-center">
                <input
                    id={id.clone()}
                    name={id.clone()}
                    type="checkbox"
                    checked={*state}
                    class={checkbox_classes}
                    onchange={onchange.clone()}
                    disabled={disabled}
                />
                <label for={id.clone()} class={label_classes}>{ label }</label>
            </div>
            {
                if let Some(description) = description {
                    html! { <p class={description_classes}>{ description }</p> }
                } else {
                    html! {}
                }
            }
        </div>
    }
}
