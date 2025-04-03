use crate::templates::demos::DemoComponent;
use tailyew::atoms::{Button, ButtonType};
use tailyew::organisms::{table::Column, Card};
use yew::prelude::*;

const HERO_IMAGE_URL: &str = "/images/TailYew.png";

#[function_component(CardDemoSection)]
pub fn card_demo_section() -> Html {
    let example = html! {
        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
            <Card
                title="Rustacean Adventures"
                subtitle={Some("Exploring the wild with Rust")}
                description={Some("Dive into performance and safety with modern systems programming.")}
                image_url={Some(HERO_IMAGE_URL.to_string())}
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
            />
        </div>
    };

    let usage_code = r#"
<Card
    title="Rustacean Adventures"
    subtitle={Some("Exploring the wild with Rust")}
    description={Some("Dive into performance and safety with modern systems programming.")}
    image_url={Some(HERO_IMAGE_URL.to_string())}
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
                "Children".into(),
                "Option<String>".into(),
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
                "Optional children content (e.g. button).".into(),
                "Additional Tailwind classes.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
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
