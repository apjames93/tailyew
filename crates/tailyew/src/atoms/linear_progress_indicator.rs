use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LinearProgressProps {
    pub progress: usize, // Progress percentage
}

#[function_component(LinearProgressIndicator)]
pub fn linear_progress_indicator(props: &LinearProgressProps) -> Html {
    html! {
        <div class="w-full bg-gray-300 dark:bg-gray-600 rounded-full h-4 overflow-hidden">
            <div
                class="h-full bg-gradient-to-r from-blue-500 to-green-500 transition-all duration-500"
                style={format!("width: {}%", props.progress)}
            ></div>
        </div>
    }
}
