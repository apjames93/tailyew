html! {
    <div class="space-y-4">
        <Button button_type={ButtonType::Primary} onclick={Callback::from(|_| console::log_1(&"Primary Click".into()))}>{ "Primary" }</Button>
        <Button button_type={ButtonType::Secondary}>{ "Secondary" }</Button>
        <Button button_type={ButtonType::Danger}>{ "Danger" }</Button>
        <Button button_type={ButtonType::Button} disabled=true>{ "Disabled" }</Button>
    </div>
}