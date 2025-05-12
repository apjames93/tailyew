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
    #[prop_or("white".to_string())]
    pub text_color: String,
    #[prop_or_default]
    pub children: Children,

    // a11y additions
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub aria_labelledby: Option<String>,
    #[prop_or_default]
    pub aria_describedby: Option<String>,
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
        text_color,
        children,
        id,
        aria_labelledby,
        aria_describedby,
    } = props.clone();

    let style = if let Some(url) = background_image_url {
        format!(
            "background-image: url('{}'); background-size: cover; background-position: center; height: {};",
            url, height
        )
    } else {
        format!("height: {};", height)
    };

    let base_title_class = classes!("text-4xl", "font-bold", title_class.unwrap_or_default());

    let base_subtitle_class = classes!("text-lg", "mt-2", subtitle_class.unwrap_or_default());

    let base_overlay_class = classes!(
        overlay_class.unwrap_or_else(|| "bg-black bg-opacity-50 p-6 rounded-lg".to_string())
    );

    let text_style = format!("color: {}", text_color);

    // Fallback generated IDs
    let title_id = format!("{}-title", id.clone().unwrap_or_else(|| "hero".to_string()));
    let subtitle_id = format!(
        "{}-subtitle",
        id.clone().unwrap_or_else(|| "hero".to_string())
    );

    let resolved_aria_labelledby = aria_labelledby.unwrap_or_else(|| title_id.clone());
    let resolved_aria_describedby = if subtitle.is_some() {
        aria_describedby.unwrap_or_else(|| subtitle_id.clone())
    } else {
        aria_describedby.unwrap_or_default()
    };

    html! {
        <Section
            id={id.clone()}
            class="text-center flex items-center justify-center"
            style={style}
        >
            <header
                class={base_overlay_class}
                role="banner"
                aria-labelledby={resolved_aria_labelledby.clone()}
                aria-describedby={if resolved_aria_describedby.is_empty() { None } else { Some(resolved_aria_describedby.clone()) }}
            >
                <Typo
                    tag={TagType::H1}
                    id={Some(title_id.clone())}
                    class={base_title_class}
                    style={text_style.clone()}
                >
                    { title }
                </Typo>

                {
                    if let Some(subtitle) = subtitle {
                        html! {
                            <Typo
                                tag={TagType::P}
                                id={Some(subtitle_id.clone())}
                                class={base_subtitle_class}
                                style={text_style.clone()}
                            >
                                { subtitle }
                            </Typo>
                        }
                    } else {
                        html! {}
                    }
                }

                <Section class="mt-4">
                    { children }
                </Section>
            </header>
        </Section>
    }
}
