use crate::Route;
use tailyew::molecules::{AppBar, AppBarPosition, ThemeToggle};
use tailyew::organisms::NestedItem;
use yew::{AttrValue, prelude::*};
use yew_router::prelude::use_navigator;

#[component(NavBar)]
pub fn nav_bar() -> Html {
    let navigator = use_navigator();
    let on_select = Callback::from(move |value: AttrValue| {
        let Some(navigator) = navigator.clone() else {
            return;
        };

        match value.as_str() {
            "about" => navigator.push(&Route::LandingPage),
            "docs" => navigator.push(&Route::DemoPage {
                component: "button".into(),
            }),
            _ => {}
        }
    });

    let nested_list = vec![
        NestedItem::with_select("About", "about"),
        NestedItem::with_select("Docs", "docs"),
        NestedItem::with_content(html! { <ThemeToggle /> }, "theme_toggle"),
        NestedItem::with_content(
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
          on_select={Some(on_select)}
          logo_on_click={Callback::from(|_| {
            let _ = web_sys::window()
                .unwrap()
                .open_with_url("https://github.com/apjames93/tailyew");
        })}
      />
    }
}
