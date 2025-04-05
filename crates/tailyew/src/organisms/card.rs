// src/organisms/card.rs

use crate::atoms::{Image, TagType, Typo};
use yew::prelude::*;

/// Properties for the Card component
#[derive(Properties, PartialEq, Clone)]
pub struct CardProps {
    #[prop_or_default]
    pub id: Option<String>,
    pub title: String,
    #[prop_or_default]
    pub subtitle: Option<String>,
    #[prop_or_default]
    pub description: Option<String>,
    #[prop_or_default]
    pub image_url: Option<String>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Option<String>,
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    let CardProps {
        id,
        title,
        subtitle,
        description,
        image_url,
        children,
        class,
    } = props;

    let card_classes = classes!(
        "rounded-lg",
        "shadow-lg",
        "overflow-hidden",
        "bg-white",
        "dark:bg-gray-800",
        "transition-transform",
        "hover:scale-105",
        "hover:shadow-2xl",
        "duration-300",
        "ease-in-out",
        class.clone()
    );

    html! {
        <div id={id.clone().unwrap_or_default()} class={card_classes}>
            if let Some(url) = image_url {
                <Image
                    src={url.clone()}
                    alt={"Card Image"}
                    class="w-full h-48 object-cover transition-transform duration-300 ease-in-out hover:scale-105"
                />
            }

            <div class="p-6 space-y-2">
                <Typo tag={TagType::H3}>{ title.clone() }</Typo>

                if let Some(sub) = subtitle {
                    <Typo tag={TagType::H4} class="text-sm text-gray-500 dark:text-gray-400">{ sub.clone() }</Typo>
                }

                if let Some(desc) = description {
                    <Typo tag={TagType::P} class="text-sm text-gray-600 dark:text-gray-300">{ desc.clone() }</Typo>
                }

                { for children.iter() }
            </div>
        </div>
    }
}
