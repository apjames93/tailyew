use crate::templates::demos::DemoComponent;
use tailyew::atoms::{Button, ButtonType};
use tailyew::molecules::HeroHeader;
use tailyew::organisms::table::Column;
use yew::prelude::*;

const HERO_IMAGE_URL: &str = "/images/TailYew.png";

#[function_component(HeroHeaderDemoSection)]
pub fn hero_header_demo_section() -> Html {
    let example = html! {
        <HeroHeader
            title="Welcome to TailYew"
            subtitle={Some("Build elegant Rust+Yew apps with beautiful defaults".to_string())}
            background_image_url={Some(HERO_IMAGE_URL.to_string())}
            height={"60vh".to_string()}
        >
            <div class="mt-4 flex justify-center gap-4">
                <Button button_type={ButtonType::Primary}>{ "Get Started" }</Button>
                <Button button_type={ButtonType::Secondary}>{ "View Components" }</Button>
            </div>
        </HeroHeader>
    };

    let usage_code = r#"
        <HeroHeader
            title="Welcome to TailYew"
            subtitle={Some("Build elegant Rust+Yew apps with beautiful defaults".to_string())}
            background_image_url={Some("https://source.unsplash.com/1600x900/?technology".to_string())}
            height={"60vh".to_string()}
        >
            <div class="mt-4 flex justify-center gap-4">
                <Button button_type={ButtonType::Primary}>{ "Get Started" }</Button>
                <Button button_type={ButtonType::Secondary}>{ "View Components" }</Button>
            </div>
        </HeroHeader>
    "#;

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "title".into(),
                "subtitle".into(),
                "background_image_url".into(),
                "title_class".into(),
                "subtitle_class".into(),
                "height".into(),
                "overlay_class".into(),
                "children".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "String".into(),
                "Option<String>".into(),
                "Option<String>".into(),
                "Option<String>".into(),
                "Option<String>".into(),
                "String".into(),
                "Option<String>".into(),
                "Children".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "Main headline text.".into(),
                "Optional subtitle under the title.".into(),
                "Optional URL for the background image.".into(),
                "Override the title text styles.".into(),
                "Override the subtitle text styles.".into(),
                "Set the height of the header (e.g., 60vh).".into(),
                "Overlay div style (defaults to semi-transparent black).".into(),
                "Optional child content below subtitle.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            title="HeroHeader Component"
            description={Some(html! {
                <p>{"The `HeroHeader` component renders a bold full-width section with an optional background image and overlay content. Great for landing pages or major section dividers."}</p>
            })}
            example={example}
            usage_code={usage_code}
            props_table={Some(props_table)}
        />
    }
}
