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
    pub overlay_class: Option<String>,
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
        overlay_class,
        height,
        children,
    } = props.clone();

    let style = if let Some(url) = background_image_url {
        format!(
            "background-image: url('{}'); background-size: cover; background-position: center; height: {};",
            url, height
        )
    } else {
        format!("height: {};", height)
    };

    let base_title_class = classes!("text-white", "text-4xl", "font-bold", title_class);
    let base_subtitle_class = classes!("text-white", "text-lg", "mt-2", subtitle_class);
    let base_overlay_class = classes!(
        overlay_class.unwrap_or_else(|| "bg-black bg-opacity-50 p-6 rounded-lg".to_string())
    );

    html! {
        <Section
            class="text-white text-center flex items-center justify-center"
            style={style}
        >
            <header class={base_overlay_class}>
                <Typo tag={TagType::H1} class={base_title_class}>
                    { title }
                </Typo>

                { if let Some(subtitle) = subtitle {
                    html! {
                        <Typo tag={TagType::P} class={base_subtitle_class}>
                            { subtitle }
                        </Typo>
                    }
                } else {
                    html! {}
                }}

                <Section class="mt-4">
                    { children }
                </Section>
            </header>
        </Section>
    }
}
