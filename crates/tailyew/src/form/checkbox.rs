use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct CheckboxProps {
    pub id: String,
    pub label: String,
    #[prop_or(false)]
    pub checked: bool,
    #[prop_or_default]
    pub description: Option<String>,
    #[prop_or(false)]
    pub disabled: bool,
    #[prop_or_default]
    pub on_change: Option<Callback<bool>>,
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
    } = props;

    // Determine styling based on state
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
        if *checked {
            "bg-primary border-primary text-white focus:ring-primary dark:bg-primary-dark dark:border-primary-dark dark:focus:ring-primary-dark"
        } else {
            "bg-white border-gray-300 text-gray-900 focus:ring-primary dark:bg-gray-800 dark:border-gray-600 dark:text-gray-400 dark:focus:ring-primary-dark"
        },
        if *disabled {
            "opacity-60 cursor-not-allowed"
        } else {
            ""
        }
    );

    let label_classes = classes!(
        "text-lg",
        "ml-2",
        "cursor-pointer",
        "transition",
        "duration-150",
        if *checked {
            "text-gray-900 dark:text-gray-300"
        } else {
            "text-gray-700 dark:text-gray-400"
        },
        if *disabled { "opacity-70" } else { "" }
    );

    let description_html = description.as_ref().map(|desc| {
        html! {
            <p class="text-sm mt-1 ml-6 text-gray-500 dark:text-gray-400">{ desc }</p>
        }
    });

    let handle_change = {
        let on_change = on_change.clone();
        Callback::from(move |e: Event| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            if let Some(cb) = &on_change {
                cb.emit(input.checked());
            }
        })
    };

    html! {
        <div class="flex flex-col space-y-1">
            <div class="flex items-center">
                <input
                    id={id.clone()}
                    name={id.clone()}
                    type="checkbox"
                    checked={*checked}
                    class={checkbox_classes}
                    onchange={handle_change}
                    disabled={*disabled}
                />
                <label for={id.clone()} class={label_classes}>
                    { label }
                </label>
            </div>
            { description_html }
        </div>
    }
}
