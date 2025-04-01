use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::window;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct CopyToClipboardProps {
    pub label: String,
    pub value: String,
}

#[function_component(CopyToClipboard)]
pub fn copy_to_clipboard(props: &CopyToClipboardProps) -> Html {
    let CopyToClipboardProps { label, value } = props.clone();

    let onclick = {
        let value = value.clone();
        Callback::from(move |_| {
            let value = value.clone();
            spawn_local(async move {
                if let Some(clipboard) = window().map(|w| w.navigator().clipboard()) {
                    let promise = clipboard.write_text(&value);
                    if JsFuture::from(promise).await.is_ok() {
                        web_sys::console::log_1(&"Copied to clipboard".into());
                    }
                }
            });
        })
    };

    html! {
        <div>
            <label class="block text-sm font-medium text-gray-700">{ label.clone() }</label>
            <div class="flex items-center space-x-2 mt-1">
                <input
                    type="text"
                    readonly=true
                    value={value.clone()}
                    class="w-full text-sm border rounded px-2 py-1 bg-gray-100"
                />
                <button
                    class="text-sm bg-blue-600 text-white px-3 py-1 rounded hover:bg-blue-700 transition"
                    onclick={onclick}
                >
                    { "Copy" }
                </button>
            </div>
        </div>
    }
}
