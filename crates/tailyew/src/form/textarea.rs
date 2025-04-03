use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct TextareaProps {
    pub id: String,
    pub label: String,
    #[prop_or_default]
    pub default_value: String,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub container_class: Option<String>,
    #[prop_or(false)]
    pub required: bool,
    #[prop_or_default]
    pub on_change: Option<Callback<String>>,
    #[prop_or(5)]
    pub rows: usize,
}

#[function_component(Textarea)]
pub fn textarea(props: &TextareaProps) -> Html {
    let TextareaProps {
        id,
        label,
        default_value,
        placeholder,
        class,
        container_class,
        required,
        on_change,
        rows,
    } = props;

    let value = use_state(|| default_value.clone());

    let oninput = {
        let value = value.clone();
        let on_change = on_change.clone();
        Callback::from(move |e: InputEvent| {
            let textarea: HtmlTextAreaElement = e.target_unchecked_into();
            let val = textarea.value();
            value.set(val.clone());
            if let Some(cb) = &on_change {
                cb.emit(val);
            }
        })
    };

    let div_class = container_class
        .clone()
        .unwrap_or_else(|| "flex flex-col space-y-2".into());

    let textarea_classes = classes!(
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
        "dark:bg-gray-800",
        "dark:border-gray-600",
        "dark:text-gray-200",
        "dark:focus:ring-green-400",
        class.clone()
    );

    let label_classes = classes!(
        "text-lg",
        "font-semibold",
        "text-gray-700",
        "dark:text-gray-200"
    );

    html! {
        <div class={div_class}>
            <label for={id.clone()} class={label_classes}>{ label.clone() }</label>
            <textarea
                id={id.clone()}
                placeholder={placeholder.clone()}
                oninput={oninput}
                value={(*value).clone()}
                class={textarea_classes}
                rows={format!("{}", rows)}
                required={*required}
            />

        </div>
    }
}
