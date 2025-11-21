use crate::templates::demos::DemoComponent;
use tailyew::atoms::{Image, Typo};
use tailyew::organisms::table::Column;
use yew::prelude::*;

#[function_component(ImageDemoSection)]
pub fn image_demo_section() -> Html {
    let example = html! {
        <div class="space-y-6">
            <Image
                src="/static/images/TailYew.png"
                alt="TailYew Logo"
                class="rounded shadow-md"
                width={Some("200px".to_string())}
            />

            <Image
                src="/static/images/TailYew.png"
                alt="Sales performance chart"
                aria_describedby="chart-desc"
            />

            <Image
                src="/static/images/TailYew.png"
                alt=""
                class="opacity-50"
                aria_label="Decorative swirl pattern"
            />
        </div>
    };

    let usage_code = r#"
<Image
    src="/static/images/TailYew.png"
    alt="TailYew Logo"
    class="rounded shadow-md"
    width={Some("200px".to_string())}
/>

<Image
    src="/static/images/chart.png"
    alt="Sales performance chart"
    aria_describedby="chart-desc"
    width={Some("100%".to_string())}
/>
<p id="chart-desc" class="text-sm text-gray-500">
    {"This chart shows monthly sales growth from January to December."}
</p>

<Image
    src="/static/images/decorative.svg"
    alt=""
    aria_label="Decorative swirl pattern"
    class="opacity-50"
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
                "aria_label".into(),
                "aria_describedby".into(),
                "role".into(),
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
                "Option<String>".into(),
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
                "Screen reader label (used when alt is empty or overridden).".into(),
                "ID of an element that describes this image.".into(),
                "Optional role attribute (e.g. \"presentation\").".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            github_demo_path="atoms/image_demo_section.rs"
            github_source_path="atoms/image.rs"
            title="Image Component"
            description={Some(html! {
                <Typo>
                    {"The "}
                    <code>{"Image"}</code>
                    {" component wraps a standard "}
                    <code>{"<img>"}</code>
                    {" tag with sensible Tailwind defaults, ARIA support, and optional sizing/styling."}
                </Typo>
            })}
            example={example}
            usage_code={usage_code}
            props_table={Some(props_table)}
        />
    }
}
