use crate::templates::demos::DemoComponent;
use tailyew::atoms::{Button, ButtonType};
use tailyew::organisms::{table::Column, Card};
use yew::prelude::*;

const HERO_IMAGE_URL: &str = "/static/images/TailYew.png";

#[component(CardDemoSection)]
pub fn card_demo_section() -> Html {
    let hover_effect_classes = classes!(
        "transition-transform",
        "hover:scale-105",
        "hover:shadow-2xl",
        "duration-300",
        "ease-in-out"
    );

    let example = html! {
        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
            <Card
                title="Rustacean Adventures"
                subtitle={Some("Exploring the wild with Rust")}
                description={Some("Dive into performance and safety with modern systems programming.")}
                image_url={Some(HERO_IMAGE_URL.to_string())}
                image_class={hover_effect_classes.clone()}
                image_alt="TailYew"
            >
                <Button button_type={ButtonType::Primary}>
                    { "Read More" }
                </Button>
            </Card>

            <Card
                title="Yew Component Demos"
                description={Some("Reusable, interactive UI blocks built with Tailwind + Yew.")}
            />

            <Card
                title="Dark Mode Ready"
                subtitle={Some("Sleek and responsive design")}
                description={Some("Cards adapt beautifully to light and dark themes out of the box.")}
                image_url={Some(HERO_IMAGE_URL.to_string())}
                class={hover_effect_classes.clone()}
            />
        </div>
    };

    let usage_code = r#"
<Card
    title="Rustacean Adventures"
    subtitle={Some("Exploring the wild with Rust")}
    description={Some("Dive into performance and safety with modern systems programming.")}
    image_url={Some(HERO_IMAGE_URL.to_string())}
    image_class={classes!(\"transition-transform\", \"hover:scale-105\", \"hover:shadow-2xl\", \"duration-300\", \"ease-in-out\")}
    class={classes!(\"transition-transform\", \"hover:scale-105\", \"hover:shadow-2xl\", \"duration-300\", \"ease-in-out\")}
>
    <Button button_type={ButtonType::Primary}>
        { "Read More" }
    </Button>
</Card>
"#;

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "id".into(),
                "title".into(),
                "subtitle".into(),
                "description".into(),
                "image_url".into(),
                "image_class".into(),
                "children".into(),
                "class".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "Option<String>".into(),
                "String".into(),
                "Option<String>".into(),
                "Option<String>".into(),
                "Option<String>".into(),
                "Classes".into(),
                "Children".into(),
                "Classes".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "Optional DOM ID.".into(),
                "Main title of the card.".into(),
                "Smaller subtitle text.".into(),
                "Description content.".into(),
                "Optional image to show above the card.".into(),
                "Additional Tailwind classes for the image element.".into(),
                "Optional children content (e.g. button).".into(),
                "Additional Tailwind classes for the card wrapper.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            github_demo_path="organisms/card_demo_section.rs"
            github_source_path="organisms/card.rs"
            title="Card Component"
            description={Some(html! {
                <p>{"The `Card` component is a reusable, responsive layout block that can include an image, title, description, and interactive children."}</p>
            })}
            example={example}
            usage_code={usage_code}
            props_table={Some(props_table)}
        />
    }
}
