// crates/tailyew/src/atoms/circular_progress_indicator.rs

use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct CircularProgressIndicatorProps {
    /// Size classes (e.g. w-8 h-8, w-16 h-16)
    #[prop_or("w-16 h-16".into())]
    pub size_class: Classes,

    /// Color for the top border (spinning part)
    #[prop_or("border-t-blue-500".into())]
    pub color_class: Classes,

    /// Additional custom classes
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(CircularProgressIndicator)]
pub fn circular_progress_indicator(props: &CircularProgressIndicatorProps) -> Html {
    let CircularProgressIndicatorProps {
        size_class,
        color_class,
        class,
    } = props;

    let spinner_classes = classes!(
        "border-4",
        "border-t-4",
        "border-transparent",
        "rounded-full",
        "animate-spin",
        size_class.clone(),
        color_class.clone(),
        class.clone(),
    );

    html! {
        <div class={spinner_classes} aria-label="Loading" role="status"></div>
    }
}
