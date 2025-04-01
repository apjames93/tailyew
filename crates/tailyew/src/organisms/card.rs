// src/organisms/card.rs

use crate::atoms::{TagType, Typo};
use yew::prelude::*;

/// Properties for the Card component
#[derive(Properties, PartialEq, Clone)]
pub struct CardProps {
    #[prop_or_default]
    pub id: Option<String>,
    pub title: String, // Main title of the card
    #[prop_or_default]
    pub subtitle: Option<String>, // Optional subtitle for the card
    #[prop_or_default]
    pub description: Option<String>, // Optional description for the card content
    #[prop_or_default]
    pub image_url: Option<String>, // Optional image URL for the card
    #[prop_or_default]
    pub children: Children, // Additional content for the card
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

    // Combine the base styles with any additional classes provided as props
    let classes = classes!(
        // "max-w-sm",
        "rounded-lg",
        "shadow-lg",
        "overflow-hidden",
        "bg-white",
        "dark:bg-gray-800",
        "transition-transform",
        "transform",
        "hover:scale-105",
        "hover:shadow-2xl",
        "duration-300",
        "ease-in-out",
        class.clone()
    );

    html! {
        <div id={id.clone().unwrap_or_default()} class={classes}>
            // Optional Card Image
            {
                if let Some(url) = image_url {
                    html! {
                        <img src={url.clone()} alt="Card Image" class="w-full h-48 object-cover transition-transform duration-300 ease-in-out hover:scale-105" />
                    }
                } else {
                    html! { <></> }
                }
            }

            <div class="p-6">
                <Typo tag={TagType::H3}>
                    { title.clone() }
                </Typo>

                { if let Some(sub) = subtitle.clone() {
                    html! {
                        <Typo tag={TagType::H4}>
                            { sub }
                        </Typo>
                    }
                } else {
                    html! { <></> }
                }}

                { if let Some(desc) = description.clone() {
                    html! {
                        <Typo tag={TagType::P}>
                            { desc }
                        </Typo>
                    }
                } else {
                    html! { <></> }
                }}

                // Render additional children content passed to the Card component
                { for children.iter() }
            </div>
        </div>
    }
}
