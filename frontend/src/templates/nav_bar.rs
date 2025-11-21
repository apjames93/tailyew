use crate::Route;
use tailyew::molecules::{AppBar, AppBarPosition, ThemeToggle};
use tailyew::organisms::NestedItem;
use yew::prelude::*;
use yew_router::prelude::Link;

#[function_component(NavBar)]
pub fn nav_bar() -> Html {
    let nested_list = vec![
        NestedItem::with_html(
            html! { <Link<Route> to={Route::LandingPage} classes="block w-full">
                { "About" }
            </Link<Route>> },
            "about",
        ),
        NestedItem::with_html(
            html! { <Link<Route> to={Route::DemoPage { component: "button".into() }} classes="block w-full">
                { "Docs" }
            </Link<Route>> },
            "docs",
        ),
        NestedItem::with_html(html! { <ThemeToggle /> }, "theme_toggle"),
        NestedItem::with_html(
            html! {
               <iframe src="https://ghbtns.com/github-btn.html?user=apjames93&repo=tailyew&type=star&count=true&size=large"
               frameborder="0" scrolling="0" width="160" height="30" title="GitHub"></iframe>
            },
            "github",
        ),
    ];

    html! {
      <AppBar
          title={Some("TailYew")}
          logo_url={Some("/static/images/logo.png")}
          position={AppBarPosition::Top}
          nested_list={nested_list}
          logo_onclick={Callback::from(|_| {
            let _ = web_sys::window()
                .unwrap()
                .open_with_url("https://github.com/apjames93/tailyew");
        })}
      />
    }
}
