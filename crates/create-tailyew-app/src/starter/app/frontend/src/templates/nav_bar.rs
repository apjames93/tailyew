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
            "login" => navigator.push(&Route::LoginPage),
            "about" | "docs" => navigator.push(&Route::LandingPage),
            _ => {}
        }
    });

    let nested_list = vec![
        NestedItem::with_children(
            "Navigation",
            vec![
                NestedItem::with_select("Login", "login"),
                NestedItem::with_select("About", "about"),
            ],
        ),
        NestedItem::with_children(
            "Actions",
            vec![
                NestedItem::with_select("Docs", "docs"),
                NestedItem::with_content(html! { <ThemeToggle /> }, "theme_toggle"),
            ],
        ),
    ];

    html! {
        <AppBar
            title={Some("TailYew")}
            logo_url={Some("static/images/logo.png")}
            position={AppBarPosition::Top}
            nested_list={nested_list}
            on_select={Some(on_select)}
        />
    }
}
