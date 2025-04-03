html! {
  <AppBar
    title={Some(AttrValue::from("TailYew"))}
    logo_url={Some(AttrValue::from("https://yew.rs/logo.svg"))}
    position={AppBarPosition::Top}
    links={vec![
        html! { <a href="#" class="text-sm font-medium hover:underline">{"Docs"}</a> },
        html! { <a href="#" class="text-sm font-medium hover:underline">{"Components"}</a> },
        html! { <a href="#" class="text-sm font-medium hover:underline">{"GitHub"}</a> },
    ]}
    actions={vec![
        html! { <Button button_type={ButtonType::Secondary}>{"Login"}</Button> },
        html! { <Button button_type={ButtonType::Primary}>{"Sign up"}</Button> },
    ]}
  />
}
