use crate::system::use_themed_classes;
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
    pub aria_label: Option<AttrValue>,

    /// Optional ARIA describedby ID
    #[prop_or_default]
    pub aria_describedby: Option<AttrValue>,

    /// Optional role (e.g. "presentation", "img")
    #[prop_or_default]
    pub role: Option<AttrValue>,
}

#[component(Image)]
pub fn image(props: &ImageProps) -> Html {
    let image_classes = use_themed_classes(
        "Image",
        "root",
        classes!("max-w-full", "h-auto"),
        props.class.clone(),
    );

    let style = {
        let mut parts = String::new();
        if let Some(w) = &props.width {
            parts.push_str(&format!("width:{};", w));
        }
        if let Some(h) = &props.height {
            parts.push_str(&format!("height:{};", h));
        }
        if parts.is_empty() {
            None
        } else {
            Some(AttrValue::from(parts))
        }
    };

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
                class={image_classes}
                style={style}
                aria-label={props.aria_label.clone()}
            aria-describedby={props.aria_describedby.clone()}
            role={resolved_role}
        />
    }
}
