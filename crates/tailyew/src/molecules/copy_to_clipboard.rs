use gloo::timers::callback::Timeout;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::window;
use yew::prelude::*;

use crate::atoms::{Button, ButtonType};
use crate::system::use_themed_classes;

#[derive(Properties, PartialEq, Clone)]
pub struct CopyToClipboardProps {
    /// The string value to copy to the clipboard
    pub value: AttrValue,

    /// The label shown before copying
    #[prop_or_else(|| "Copy".into())]
    pub copy_text: String,

    /// The label shown after copying
    #[prop_or_else(|| "Copied!".into())]
    pub copied_text: String,

    /// Optional button type before copying
    #[prop_or(ButtonType::Primary)]
    pub button_type: ButtonType,

    /// Optional button type after copying
    #[prop_or(ButtonType::Secondary)]
    pub copied_button_type: ButtonType,

    /// Optional extra classes for the button
    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub children: Children,
}

#[component(CopyToClipboard)]
pub fn copy_to_clipboard(props: &CopyToClipboardProps) -> Html {
    let CopyToClipboardProps {
        value,
        copy_text,
        copied_text,
        button_type,
        copied_button_type,
        class,
        children,
    } = props.clone();
    let button_class =
        use_themed_classes("CopyToClipboard", "root", Classes::default(), class.clone());

    let copied = use_state(|| false);

    let onclick = {
        let value = value.clone();
        let copied = copied.clone();

        Callback::from(move |_| {
            let value = value.clone();
            let copied = copied.clone();

            spawn_local(async move {
                if let Some(clipboard) = window().map(|w| w.navigator().clipboard()) {
                    if JsFuture::from(clipboard.write_text(&value)).await.is_ok() {
                        copied.set(true);
                        Timeout::new(2000, move || copied.set(false)).forget();
                    }
                }
            });
        })
    };

    let current_label = if *copied {
        copied_text.clone()
    } else {
        copy_text.clone()
    };
    let current_type = if *copied {
        copied_button_type.clone()
    } else {
        button_type.clone()
    };

    let content = if children.is_empty() {
        html! { current_label }
    } else {
        Html::from_iter(children.iter())
    };

    html! {
        <Button
            button_type={current_type}
            on_click={onclick}
            class={button_class}
        >
            { content }
        </Button>
    }
}
