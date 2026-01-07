use yew::prelude::*;

/// A generic image component with default styles for TailYew
#[derive(Properties, PartialEq, Clone)]
pub struct ImageProps {
    /// Image URL (required)
    pub src: AttrValue,

    /// Optional alt text ("" for decorative images)
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

    /// Optional ARIA label override
    #[prop_or_default]
    pub aria_label: Option<String>,

    /// Optional ARIA describedby ID
    #[prop_or_default]
    pub aria_describedby: Option<String>,

    /// Optional role (e.g. "presentation", "img")
    #[prop_or_default]
    pub role: Option<String>,
}

#[component(Image)]
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

    // Auto role="presentation" for alt="" unless overridden
    let resolved_role = if props.alt.is_empty() && props.role.is_none() {
        Some("presentation".into())
    } else {
        props.role.clone()
    };

    html! {
        <img
            src={props.src.clone()}
            alt={props.alt.clone()}
            class={classes!("max-w-full", "h-auto", props.class.clone())}
            style={style}
            aria-label={props.aria_label.clone()}
            aria-describedby={props.aria_describedby.clone()}
            role={resolved_role}
        />
    }
}
