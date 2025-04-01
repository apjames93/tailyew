use yew::prelude::*;

const LOGO_IMAGE_URL: &str = "/images/logo.png";
const TAILYEW_IMAGE_URL: &str = "/images/TailYew.png";

#[function_component(LandingPage)]
pub fn landing() -> Html {
    html! {
        <div class="container mx-auto p-4">
            <h1>{"TailYew"}</h1>
            <img src={TAILYEW_IMAGE_URL} width="500" height="600" />
            <img src={LOGO_IMAGE_URL} width="50" height="50" />
        </div>
    }
}
