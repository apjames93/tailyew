// src/organisms/card.rs

use crate::atoms::{Image, TagType, Typo};
use crate::system::use_themed_classes;
use yew::prelude::*;

/// Properties for the Card component
#[derive(Properties, PartialEq, Clone)]
pub struct CardProps {
    #[prop_or_default]
    pub id: Option<AttrValue>,
    pub title: String,
    #[prop_or_default]
    pub subtitle: Option<String>,
    #[prop_or_default]
    pub description: Option<String>,
    #[prop_or_default]
    pub image_url: Option<AttrValue>,
    #[prop_or_default]
    pub image_alt: Option<AttrValue>,
    #[prop_or_default]
    pub image_class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[component(Card)]
pub fn card(props: &CardProps) -> Html {
    let CardProps {
        id,
        title,
        subtitle,
        description,
        image_url,
        image_alt,
        image_class,
        children,
        class,
    } = props;

    let root_defaults = classes!(
        "rounded-lg",
        "shadow-lg",
        "overflow-hidden",
        "bg-white",
        "dark:bg-gray-800",
    );
    let card_classes = use_themed_classes("Card", "root", root_defaults, class.clone());
    let image_classes = use_themed_classes(
        "Card",
        "image",
        classes!("w-full", "h-48", "object-cover"),
        image_class.clone(),
    );
    let body_classes = use_themed_classes(
        "Card",
        "body",
        classes!("p-6", "space-y-2"),
        Classes::default(),
    );

    html! {
        <div id={id.clone()} class={card_classes}>
            if let Some(url) = image_url {
                <Image
                    src={url.clone()}
                    alt={image_alt.clone().unwrap_or_else(|| AttrValue::from("Card Image"))}
                    class={image_classes}
                />
            }

            <div class={body_classes}>
                <Typo tag={TagType::H3}>{ html! { title.clone() } }</Typo>

                if let Some(sub) = subtitle {
                    <Typo tag={TagType::H4} class="text-sm text-gray-500 dark:text-gray-400">{ html! { sub.clone() } }</Typo>
                }

                if let Some(desc) = description {
                    <Typo tag={TagType::P} class="text-sm text-gray-600 dark:text-gray-300">{ html! { desc.clone() } }</Typo>
                }

                { for children.iter() }
            </div>
        </div>
    }
}
