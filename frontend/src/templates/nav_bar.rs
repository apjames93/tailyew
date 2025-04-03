use crate::Route;
use tailyew::atoms::{Button, ButtonType};
use tailyew::molecules::{AppBar, AppBarPosition};
use yew::prelude::*;
use yew_router::prelude::Link;

#[function_component(NavBar)]
pub fn nav_bar() -> Html {
    html! {
      <AppBar
          title={Some("TailYew")}
          position={AppBarPosition::Top}
          logo_url={Some("/images/logo.png")}
          links={vec![
              html! { <Link<Route> to={Route::DemoPage { component: "accordion".into() }}>{ "Components" }</Link<Route>> },
              html! { <Link<Route> to={Route::LandingPage}>{ "About" }</Link<Route>> },
          ]}
          actions={vec![
              html! { <Link<Route> to={Route::DemoPage { component: "button".into() }}>
                  <Button button_type={ButtonType::Primary}>{ "Docs" }</Button>
              </Link<Route>> },
          ]}
      />
    }
}
