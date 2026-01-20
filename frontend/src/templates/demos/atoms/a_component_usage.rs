html! {
    <div class="flex flex-col items-start gap-2">
        <A href="https://example.com">{ "Visit Example" }</A>

        <A href="https://example.com" target="_blank">
            { "Open in new tab" }
        </A>

        <A
            href="#"
            class="text-red-500"
            on_click={Callback::from(|_| web_sys::console::log_1(&"Anchor clicked!".into()))}
        >
            { "Custom class with click" }
        </A>

        <A
            href="https://example.com/docs"
            aria_label="Visit the documentation page"
        >
            <span class="sr-only">{ "Docs" }</span>
            { "📘 Docs" }
        </A>

        <A
            href="#help"
            aria_describedby="help-desc"
        >
            { "What is this?" }
        </A>

        <A
            href="https://example.com/role"
            role="button"
            tabindex={0}
        >
            { "Simulated Button Link" }
        </A>
    </div>
}
