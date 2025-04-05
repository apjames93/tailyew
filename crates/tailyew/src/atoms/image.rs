use yew::prelude::*;

/// A generic image component with default styles for TailYew
#[derive(Properties, PartialEq, Clone)]
pub struct ImageProps {
    /// Image URL (required)
    pub src: AttrValue,

    /// Optional alt text
    #[prop_or_default]
    pub alt: AttrValue,

    /// Optional Tailwind classes
    #[prop_or_default]
    pub class: Classes,

    /// Optional image height (CSS style)
    #[prop_or_default]
    pub height: Option<String>,

    /// Optional image width (CSS style)
    #[prop_or_default]
    pub width: Option<String>,
}

#[function_component(Image)]
pub fn image(props: &ImageProps) -> Html {
    let style = format!(
        "{}{}",
        props
            .width
            .as_ref()
            .map(|w| format!("width:{};", w))
            .unwrap_or_default(),
        props
            .height
            .as_ref()
            .map(|h| format!("height:{};", h))
            .unwrap_or_default()
    );

    html! {
        <img
            src={props.src.clone()}
            alt={props.alt.clone()}
            class={classes!("max-w-full", "h-auto", props.class.clone())}
            style={style}
        />
    }
}
