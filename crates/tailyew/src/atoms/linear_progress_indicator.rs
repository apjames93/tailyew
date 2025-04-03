use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LinearProgressProps {
    /// Progress percent from 0–100
    pub progress: usize,

    /// Optional additional classes for the inner progress bar
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(LinearProgressIndicator)]
pub fn linear_progress_indicator(props: &LinearProgressProps) -> Html {
    let progress = props.progress.clamp(0, 100); // safe bounds

    let bar_classes = classes!(
        "h-full",
        "transition-all",
        "duration-500",
        "bg-gradient-to-r",
        "from-blue-500",
        "to-green-500",
        props.class.clone()
    );

    html! {
        <div class="w-full bg-gray-300 dark:bg-gray-600 rounded-full h-4 overflow-hidden">
            <div
                class={bar_classes}
                style={format!("width: {}%", progress)}
            ></div>
        </div>
    }
}
