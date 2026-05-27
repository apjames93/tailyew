use serde_json::Value;
use tailyew::form::{FormSubmitCallback, async_callback, e_input_value};
use web_sys::SubmitEvent;
use yew::prelude::*;

pub(super) fn json_submit_handler(
    id: &'static str,
    state: UseStateHandle<Value>,
) -> FormSubmitCallback {
    async_callback({
        let state = state.clone();
        move |e: SubmitEvent| {
            let state = state.clone();
            async move {
                let json_str = e_input_value(id, &e);
                match serde_json::from_str(&json_str) {
                    Ok(value) => {
                        state.set(value);
                        Ok(None)
                    }
                    Err(err) => {
                        web_sys::console::error_1(&format!("Invalid JSON: {err}").into());
                        Err("Invalid JSON".into())
                    }
                }
            }
        }
    })
}

pub(super) fn submitted_json_preview(value: &Value) -> Html {
    if value == &Value::Null {
        return html! {};
    }

    html! {
        <div class="rounded-lg border border-gray-200 bg-gray-50 p-3 text-left dark:border-gray-700 dark:bg-gray-900/60">
            <p class="mb-2 text-xs font-semibold uppercase tracking-wide text-gray-500 dark:text-gray-400">
                { "Submitted JSON" }
            </p>
            <pre class="max-h-72 overflow-auto whitespace-pre-wrap text-xs text-gray-800 dark:text-gray-100">
                { serde_json::to_string_pretty(value).unwrap_or_else(|_| value.to_string()) }
            </pre>
        </div>
    }
}
