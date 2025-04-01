// src/molecules/hero_header.rs

use crate::atoms::{Section, TagType, Typo};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct HeroHeaderProps {
    pub title: String,
    #[prop_or_default]
    pub subtitle: Option<String>,
    #[prop_or_default]
    pub background_image_url: Option<String>,
    #[prop_or_default]
    pub title_class: Option<String>,
    #[prop_or_default]
    pub subtitle_class: Option<String>,
    #[prop_or("50vh".to_string())]
    pub height: String,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(HeroHeader)]
pub fn hero_header(props: &HeroHeaderProps) -> Html {
    let HeroHeaderProps {
        title,
        subtitle,
        background_image_url,
        title_class,
        subtitle_class,
        height,
        children,
    } = props.clone();

    // Create style with optional background image and custom height
    let style = if let Some(url) = background_image_url {
        format!(
            "background-image: url('{}'); background-size: cover; background-position: center; height: {};",
            url, height
        )
    } else {
        format!("height: {};", height)
    };

    html! {
        <Section class="text-white text-center flex items-center justify-center" style={style}>
            <Section class="bg-black bg-opacity-50 rounded">
                // Title with Typo component and custom class support
                <Typo tag={TagType::H1} class={classes!("text-white", title_class)}>
                    { title }
                </Typo>

                // Optional Subtitle with Typo component and custom class support
                { if let Some(subtitle) = subtitle {
                    html! {
                        <Typo tag={TagType::P} class={classes!("text-white", subtitle_class)}>
                        { subtitle }
                        </Typo>
                    }
                } else {
                    html! {}
                }}

                <Section>
                    { children.clone() }
                </Section>
            </Section>
        </Section>
    }
}
