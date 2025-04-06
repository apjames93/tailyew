use crate::Route;
use tailyew::atoms::{Button, ButtonType, Typo};
use tailyew::molecules::{AppBar, AppBarPosition, ThemeToggle};
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
              html! { <Link<Route> to={Route::DemoPage { component: "accordion".into() }}>
                <Typo>{"Components"}</Typo>
              </Link<Route>> },
              html! { <Link<Route> to={Route::LandingPage}><Typo>{ "About" }</Typo></Link<Route>> },
          ]}
          actions={vec![
              html! { <Link<Route> to={Route::DemoPage { component: "button".into() }}>
                  <Button button_type={ButtonType::Primary}>{ "Docs" }</Button>
              </Link<Route>> },
              html!{ <ThemeToggle /> }
          ]}
      />
    }
}
