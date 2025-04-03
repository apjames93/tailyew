// frontend/src/templates/demos/a_demo/a_component_usage.rs

html! {
    <div class="flex flex-col items-start gap-2">
        <A href="https://example.com">{ "Visit Example" }</A>
        <A href="https://example.com" target="_blank">{ "Open in new tab" }</A>
        <A
            href="#"
            class="text-red-500"
            onclick={Callback::from(|_| web_sys::console::log_1(&"Anchor clicked!".into()))}
        >
            { "Custom class with click" }
        </A>
    </div>
}
