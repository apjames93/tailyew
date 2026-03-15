use crate::system::use_themed_classes;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct AvatarProps {
    /// The image URL to display
    #[prop_or_default]
    pub src: Option<AttrValue>,

    /// Fallback text (e.g. initials) if image fails or is not provided
    #[prop_or_default]
    pub fallback: Option<AttrValue>,

    /// Alt text for accessibility
    #[prop_or_default]
    pub alt: Option<AttrValue>,

    /// Tailwind sizing (e.g. w-10 h-10 or w-16 h-16)
    #[prop_or_else(|| "w-12 h-12".into())]
    pub size: Classes,

    /// Optional additional classes
    #[prop_or_default]
    pub class: Classes,

    /// Optional click callback
    #[prop_or_default]
    pub on_click: Option<Callback<MouseEvent>>,
}

#[component(Avatar)]
pub fn avatar(props: &AvatarProps) -> Html {
    let AvatarProps {
        src,
        fallback,
        alt,
        size,
        class,
        on_click,
    } = props.clone();

    let root_defaults = classes!(
        "inline-flex",
        "items-center",
        "justify-center",
        "rounded-full",
        "overflow-hidden",
        "bg-gray-200",
        "dark:bg-gray-700",
        "text-white",
        "font-semibold",
        "uppercase",
        "aspect-square",
        size.clone(),
    );
    let mut base_classes = use_themed_classes("Avatar", "root", root_defaults, class.clone());

    if on_click.is_some() {
        base_classes.push("cursor-pointer");
    }

    let image_classes = use_themed_classes(
        "Avatar",
        "image",
        classes!(base_classes.clone(), "object-cover"),
        Classes::default(),
    );

    if let Some(src) = src {
        html! {
            <img
                src={src}
                alt={alt.unwrap_or_else(|| AttrValue::from("Avatar"))}
                class={image_classes}
                onclick={on_click}
            />
        }
    } else {
        html! {
            <div
                class={base_classes}
                onclick={on_click}
                role="img"
                aria-label={alt.clone().unwrap_or_else(|| AttrValue::from("Avatar"))}
            >
                <span class="text-base leading-none text-center">
                    { fallback.unwrap_or_else(|| "?".into()) }
                </span>
            </div>
        }
    }
}
