use crate::form_deserializer::{de_attr, de_classes};
use serde::Deserialize;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone, Default, Deserialize)]
pub struct RadioGroupProps {
    /// the shared name/id for the group
    #[prop_or_default]
    #[serde(default, deserialize_with = "de_attr")]
    pub id: AttrValue,

    /// optional visual label for the whole group
    #[prop_or_default]
    #[serde(default, deserialize_with = "de_attr")]
    pub label: AttrValue,

    /// list of (value, label) pairs
    #[prop_or_default]
    #[serde(default)]
    pub options: Vec<(String, String)>,

    /// which value is selected by default
    #[prop_or_default]
    #[serde(default, deserialize_with = "de_attr")]
    pub default_value: AttrValue,

    /// any extra CSS classes to apply to the container
    #[prop_or_default]
    #[serde(default, deserialize_with = "de_classes")]
    pub class: Classes,

    /// programmatic callback on change (not deserialized from JSON)
    #[prop_or_default]
    #[serde(skip)]
    pub on_change: Option<Callback<String>>,
}

#[component(RadioGroup)]
pub fn radio_group(props: &RadioGroupProps) -> Html {
    let RadioGroupProps {
        id,
        label,
        options,
        default_value,
        class,
        on_change,
    } = props.clone();

    // state for the selected value
    let selected = use_state(|| default_value.clone());

    // when any radio button changes
    let onchange = {
        let selected = selected.clone();
        let on_change = on_change.clone();
        Callback::from(move |e: Event| {
            let new_val = e.target_unchecked_into::<HtmlInputElement>().value();
            selected.set(new_val.clone().into());
            if let Some(cb) = &on_change {
                cb.emit(new_val);
            }
        })
    };

    // merge your custom classes into the container
    let container_classes = classes!("flex", "flex-col", "space-y-4", class.clone());

    let label_classes = classes!(
        "text-lg",
        "font-semibold",
        "text-gray-700",
        "dark:text-gray-300"
    );
    let item_classes = classes!("flex", "items-center", "space-x-2");
    let input_classes = classes!(
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
    let text_classes = classes!("text-gray-700", "dark:text-gray-400");

    html! {
        <div class={container_classes}>
            { // only show a legend/label if non-empty
              if !label.is_empty() {
                html! { <label class={label_classes.clone()}>{ label.clone() }</label> }
              } else {
                html!{}
              }
            }

            <div class="flex flex-col space-y-2">
                { for options.iter().map(|(value, text)| {
                    let checked = *selected == *value;
                    html! {
                        <div class={item_classes.clone()}>
                            <input
                                type="radio"
                                id={format!("{}-{}", id, value)}
                                name={id.clone()}
                                value={value.clone()}
                                checked={checked}
                                aria-checked={checked.to_string()}
                                onchange={onchange.clone()}
                                class={input_classes.clone()}
                            />
                            <label
                                for={format!("{}-{}", id, value)}
                                class={text_classes.clone()}
                            >
                                { text }
                            </label>
                        </div>
                    }
                }) }
            </div>
        </div>
    }
}
