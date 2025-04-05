html! {
    <div class="space-y-4">
        <Button button_type={ButtonType::Primary} onclick={Callback::from(|_| console::log_1(&"Primary Click".into()))}>{ "Primary" }</Button>
        <Button button_type={ButtonType::Secondary}>{ "Secondary" }</Button>
        <Button button_type={ButtonType::Danger}>{ "Danger" }</Button>
        <Button button_type={ButtonType::Button} disabled=true>{ "Disabled" }</Button>
        <Button button_type={ButtonType::Ghost}>{ "Ghost" }</Button>

        <Button
            button_type={ButtonType::Icon}
            class="p-2 rounded-md"
        >
            <svg class="h-6 w-6" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" d="M4 6h16M4 12h16M4 18h16"/>
            </svg>
        </Button>

    </div>
}