use yew::prelude::*;

#[function_component(CircularProgressIndicator)]
pub fn circular_progress_indicator() -> Html {
    html! {
        <div class="w-16 h-16 border-4 border-t-4 border-transparent border-t-blue-500 rounded-full animate-spin">
        </div>
    }
}
