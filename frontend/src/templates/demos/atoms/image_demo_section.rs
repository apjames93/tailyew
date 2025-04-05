// frontend/src/templates/demos/image_demo_section.rs

use crate::templates::demos::DemoComponent;
use tailyew::atoms::Image;
use tailyew::organisms::table::Column;
use yew::prelude::*;

#[function_component(ImageDemoSection)]
pub fn image_demo_section() -> Html {
    let example = html! {
        <Image
            src="/images/TailYew.png"
            alt="TailYew Logo"
            class="rounded shadow-md"
            width={Some("200px".to_string())}
        />
    };

    let usage_code = r#"
<Image
    src="/images/TailYew.png"
    alt="TailYew Logo"
    class="rounded shadow-md"
    width={Some("200px".to_string())}
/>
"#;

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "src".into(),
                "alt".into(),
                "class".into(),
                "height".into(),
                "width".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "AttrValue".into(),
                "AttrValue".into(),
                "Classes".into(),
                "Option<String>".into(),
                "Option<String>".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "The image source URL.".into(),
                "Alternative text for accessibility.".into(),
                "Tailwind utility classes for styling.".into(),
                "Optional height (e.g., \"100px\").".into(),
                "Optional width (e.g., \"200px\").".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            title="Image Component"
            description={Some(html! {
                <p>{ "The `Image` component wraps a standard <img> tag with sensible Tailwind defaults and optional sizing props." }</p>
            })}
            example={example}
            usage_code={usage_code}
            props_table={Some(props_table)}
        />
    }
}
