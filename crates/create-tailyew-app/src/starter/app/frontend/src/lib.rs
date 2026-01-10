// frontend/src/lib.rs

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;
mod pages;
mod templates;
mod helpers;
use pages::{AppRouter, LandingPage, NotFoundPage, LoginPage};
use tailyew::system::{InitTheme, Theme};

/// Define the application routes and implement the `Routable` trait
#[derive(Clone, Routable, PartialEq, Debug)]
pub enum Route {
    #[at("/")]
    LandingPage,
    #[at("/login")]
    LoginPage,
    #[not_found]
    #[at("/404")]
    NotFoundPage,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::LandingPage => html! { <LandingPage /> },
        Route::LoginPage => html! { <LoginPage /> },
        Route::NotFoundPage => html! { <NotFoundPage /> },
    }
}

#[component(App)]
pub fn app() -> Html {
    let my_theme = Theme {
        name: "system".into(),
        class: classes!(),
    };

    html! {
        <BrowserRouter>
            <InitTheme theme={Some(my_theme)}>
                <AppRouter />
            </InitTheme>
        </BrowserRouter>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}

pub mod app {
    pub use super::App;
}
