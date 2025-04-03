use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::window;
use yew::prelude::*;

use crate::atoms::{Button, ButtonType};
use crate::form::{Input, InputType};

#[derive(Properties, PartialEq, Clone)]
pub struct CopyToClipboardProps {
    pub label: String,
    pub value: String,
}

#[function_component(CopyToClipboard)]
pub fn copy_to_clipboard(props: &CopyToClipboardProps) -> Html {
    let CopyToClipboardProps { label, value } = props.clone();
    let copied = use_state(|| false);

    let onclick = {
        let value = value.clone();
        let copied = copied.clone();

        Callback::from(move |_| {
            let value = value.clone();
            let copied = copied.clone();

            spawn_local(async move {
                // wrap clipboard in Some() to match `and_then` signature
                if let Some(clipboard) = window().map(|w| w.navigator().clipboard()) {
                    let result = JsFuture::from(clipboard.write_text(&value)).await;
                    if result.is_ok() {
                        copied.set(true);
                        web_sys::console::log_1(&"Copied to clipboard".into());
                    }
                }
            });
        })
    };

    html! {
        <div class="flex flex-col space-y-2">
            <Input
                id="clipboard-value"
                label={label}
                input_type={InputType::Text}
                default_value={value.clone()}
                placeholder="Click Copy to copy this value"
                required={false}
                disabled={true}
                class="w-full"
            />

            <div class="flex items-center gap-2">
                <Button
                    button_type={ButtonType::Primary}
                    onclick={onclick}
                    class="whitespace-nowrap"
                >
                    { "Copy" }
                </Button>

                {
                    if *copied {
                        html! {
                            <span class="text-xs text-green-600 dark:text-green-400">{ "Copied!" }</span>
                        }
                    } else {
                        html! {}
                    }
                }
            </div>
        </div>
    }
}
