html! {
  <AppBar
    title={Some(AttrValue::from("TailYew"))}
    logo_url={Some(AttrValue::from(LOGO_IMAGE_URL))}
    nested_list={vec![
      NestedItem::with_select("Home", "home"),
      NestedItem::with_select("Docs", "docs"),
      NestedItem::with_select("Components", "components"),
      NestedItem::with_external_link("GitHub", "github", "https://github.com/apjames93/tailyew"),
      NestedItem::with_children(
          "Account",
          vec![
              NestedItem::with_select("Login", "login"),
              NestedItem::with_select("Sign up", "signup"),
          ],
      ),
      NestedItem::with_children(
          "Settings",
          vec![
              NestedItem::with_content(
                  html! { <ThemeToggle /> },
                  "theme-toggle",
              ),
          ],
      ),
  ]}
    position={AppBarPosition::Static}
    on_select={Some(Callback::from(|value: AttrValue| {
      log::info!("selected {value}");
    }))}
  />
}
