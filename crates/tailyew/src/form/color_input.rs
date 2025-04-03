use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ColorInputProps {
    pub id: String,
    pub label: String,
    #[prop_or("#000000".into())]
    pub value: String,
    #[prop_or_default]
    pub on_change: Option<Callback<String>>,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(ColorInput)]
pub fn color_input(props: &ColorInputProps) -> Html {
    let ColorInputProps {
        id,
        label,
        value,
        on_change,
        class,
    } = props;

    let color = use_state(|| value.clone());

    let handle_input = {
        let color = color.clone();
        let on_change = on_change.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let new_color = input.value();
            color.set(new_color.clone());
            if let Some(cb) = &on_change {
                cb.emit(new_color);
            }
        })
    };

    let input_classes = classes!(
        "w-16",
        "h-10",
        "p-1",
        "border",
        "rounded-lg",
        "shadow-sm",
        "focus:ring-2",
        "focus:ring-green-500",
        "focus:border-green-500",
        "transition",
        "duration-150",
        "bg-white",
        "dark:bg-gray-800",
        "dark:border-gray-600",
        "dark:text-gray-200",
        class.clone()
    );

    let preview_classes = classes!(
        "w-10",
        "h-10",
        "rounded-full",
        "border",
        "shadow-sm",
        "transition",
        "duration-150",
        "border-gray-300",
        "dark:border-gray-600"
    );

    let label_classes = classes!(
        "text-lg",
        "font-semibold",
        "text-gray-700",
        "dark:text-gray-200"
    );

    let description_classes = classes!("text-gray-600", "dark:text-gray-400");

    html! {
        <div class="flex flex-col space-y-2">
            <label for={id.to_string()} class={label_classes}>{ label.clone() }</label>
            <div class="flex items-center space-x-4">
                <input
                    id={id.clone()}
                    name={id.clone()}
                    type="color"
                    value={(*color).clone()}
                    class={input_classes}
                    oninput={handle_input}
                    aria-label={label.clone()}
                />
                <span
                    class={preview_classes}
                    style={format!("background-color: {};", *color)}
                ></span>
                <p class={description_classes}>{ format!("Selected color: {}", *color) }</p>
            </div>
        </div>
    }
}
