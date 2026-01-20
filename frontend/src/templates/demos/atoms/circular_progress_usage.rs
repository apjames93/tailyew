html! {
  <div class="flex items-center gap-6">
      <CircularProgressIndicator />
      <CircularProgressIndicator size_class={classes!("w-12", "h-12")} color_class={classes!("border-t-green-500")} />
      <CircularProgressIndicator size_class={classes!("w-6", "h-6")} color_class={classes!("border-t-red-500")} />
  </div>
}
