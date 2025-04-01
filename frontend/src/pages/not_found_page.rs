use yew::prelude::*;

const TAILYEW_IMAGE_URL: &str = "/images/TailYew.png";

#[function_component(NotFoundPage)]
pub fn not_found() -> Html {
    html! {
        <div class="container mx-auto p-4">
            <img src={TAILYEW_IMAGE_URL} width="500" height="600" />
        </div>
    }
}
