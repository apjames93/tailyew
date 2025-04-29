html! {
  <AppBar
    title={Some(AttrValue::from("TailYew"))}
    logo_url={Some(AttrValue::from(LOGO_IMAGE_URL))}
    nested_list={vec![
      NestedItem::with_html(html! { "Home" }, "home"),
      NestedItem::with_html(html! { "Docs" }, "docs"),
      NestedItem::with_html(html! { "Components" }, "components"),
      NestedItem::with_html(html! { "GitHub" }, "github"),
      NestedItem::with_children(
          "Account",
          vec![
              NestedItem::with_html(
                  html! { <Button button_type={ButtonType::Primary}>{ "Login" }</Button> },
                  "login",
              ),
              NestedItem::with_html(
                  html! { <Button button_type={ButtonType::Secondary}>{ "Sign up" }</Button> },
                  "signup",
              ),
          ],
      ),
      NestedItem::with_children(
          "Settings",
          vec![
              NestedItem::with_html(
                  html! { <ThemeToggle /> },
                  "theme-toggle",
              ),
          ],
      ),
  ]}
    position={AppBarPosition::Static}
  />
}
