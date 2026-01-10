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
    pub title_class: Classes,
    #[prop_or_default]
    pub subtitle_class: Classes,
    #[prop_or("50vh".to_string())]
    pub height: String,
    #[prop_or_default]
    pub overlay_class: Classes,
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

#[component(HeroHeader)]
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

    let style = if let Some(url) = background_image_url.clone() {
        format!(
            "background-image: url('{}'); background-size: cover; background-position: center; background-repeat: no-repeat; min-height: {}; height: auto;",
            url, height
        )
    } else {
        format!("min-height: {}; height: auto;", height)
    };

    let base_title_class = classes!(
        "text-2xl",
        "sm:text-3xl",
        "md:text-4xl",
        "lg:text-5xl",
        "font-bold",
        "leading-tight",
        title_class.clone()
    );

    let base_subtitle_class = classes!(
        "text-base",
        "sm:text-lg",
        "mt-2",
        "sm:mt-3",
        "leading-relaxed",
        subtitle_class.clone()
    );

    let base_overlay_class = classes!(
        "bg-black",
        "bg-opacity-50",
        "p-4",
        "sm:p-6",
        "md:p-8",
        "rounded-md",
        "sm:rounded-lg",
        "max-w-3xl",
        "w-full",
        "mx-auto",
        "px-4",
        "sm:px-6",
        "text-center",
        "space-y-4",
        "sm:space-y-5",
        overlay_class.clone()
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

    let section_class = classes!(
        "text-center",
        "flex",
        "items-center",
        "justify-center",
        "w-full",
        "px-4",
        "sm:px-6",
        "py-10",
        "md:py-14",
        background_image_url.map(|_| "bg-cover bg-center bg-no-repeat")
    );

    html! {
        <Section
            id={id.clone()}
            class={section_class}
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
                    { html! { title } }
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
                                { html! { subtitle } }
                            </Typo>
                        }
                    } else {
                        html! {}
                    }
                }

                <Section class="mt-4 sm:mt-6 w-full">
                    { children }
                </Section>
            </header>
        </Section>
    }
}
