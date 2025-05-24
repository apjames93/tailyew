use crate::Route;
use tailyew::atoms::{Button, ButtonType};
use tailyew::molecules::{AppBar, AppBarPosition, ThemeToggle};
use tailyew::organisms::NestedItem;
use yew::prelude::*;
use yew_router::prelude::Link;

#[function_component(NavBar)]
pub fn nav_bar() -> Html {
    let nested_list = vec![
        NestedItem::with_children(
            "Navigation",
            vec![
                NestedItem::with_html(
                    html! {
                        <Link<Route> to={Route::NotFoundPage}>
                            { "404" }
                        </Link<Route>>
                    },
                    "404",
                ),
                NestedItem::with_html(
                    html! {
                        <Link<Route> to={Route::LandingPage}>
                            { "About" }
                        </Link<Route>>
                    },
                    "about",
                ),
            ],
        ),
        NestedItem::with_children(
            "Actions",
            vec![
                NestedItem::with_html(
                    html! {
                        <Link<Route> to={Route::LandingPage}>
                            <Button button_type={ButtonType::Primary}>{ "Docs" }</Button>
                        </Link<Route>>
                    },
                    "docs",
                ),
                NestedItem::with_html(
                    html! { <ThemeToggle /> },
                    "theme_toggle",
                ),
            ],
        ),
    ];

    html! {
        <AppBar
            title={Some("TailYew")}
            logo_url={Some("/images/logo.png")}
            position={AppBarPosition::Top}
            nested_list={nested_list}
        />
    }
}
