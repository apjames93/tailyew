html! {
  <ul class="space-y-2">
      <Li active=true>{ "Active item" }</Li>
      <Li hover=true>{ "Hoverable item" }</Li>
      <Li bordered=true>{ "Bordered item" }</Li>
      <Li with_icon=true icon={html! { <span class="material-icons">{"check"}</span> }}>
          { "Item with icon" }
      </Li>
      <Li onclick={Callback::from(|_| web_sys::console::log_1(&"Item clicked!".into()))}>
          { "Clickable item" }
      </Li>
  </ul>
}
