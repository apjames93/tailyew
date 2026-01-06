use crate::templates::demos::DemoComponent;
use tailyew::atoms::{TagType, Typo};
use tailyew::molecules::Accordion;
use tailyew::organisms::table::Column;
use yew::prelude::*;

#[component(AccordionDemoSection)]
pub fn accordion_demo_section() -> Html {
    let example = html! {
        <div class="space-y-4">
            <Accordion
                title="What is TailYew?"
                default_open={true}
                heading_tag={TagType::H3}
            >
                <Typo>{"TailYew is a UI component library for Rust + Yew, styled with Tailwind CSS."}</Typo>
            </Accordion>

            <Accordion
                title="Can I customize components?"
                heading_tag={TagType::H3}
                content_class="bg-yellow-50 dark:bg-yellow-900"
            >
                <Typo>{"Yes! You can extend or override any component using standard Rust and Yew patterns."}</Typo>
            </Accordion>

            <Accordion
                title="Compact mode example"
                compact={true}
                default_open={true}
            >
                <Typo>{"Compact accordions remove padding and styling for use in tighter layouts (like sidebars)."}</Typo>
            </Accordion>
        </div>
    };

    let usage_code = r#"
<Accordion title="What is TailYew?" default_open={true} heading_tag={TagType::H3}>
    <Typo>{"TailYew is a UI component library for Rust + Yew, styled with Tailwind CSS."}</Typo>
</Accordion>

<Accordion title="Compact mode" compact={true}>
    <Typo>{"Useful in nested UIs like sidebars."}</Typo>
</Accordion>
"#;

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "title".into(),
                "children".into(),
                "class".into(),
                "content_class".into(),
                "heading_tag".into(),
                "default_open".into(),
                "compact".into(),
                "arrow".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "AttrValue".into(),
                "Children".into(),
                "Classes".into(),
                "Classes".into(),
                "TagType".into(),
                "bool".into(),
                "bool".into(),
                "Option<Html>".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "The title of the accordion header.".into(),
                "Content shown when expanded.".into(),
                "Classes for the outer wrapper.".into(),
                "Additional Tailwind classes for content wrapper.".into(),
                "Tag used for the heading (e.g. H2, Span).".into(),
                "Initial open state.".into(),
                "Enable compact styling for dense layouts.".into(),
                "Optional icon to replace the default arrow.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            github_demo_path="molecules/accordion_demo_section.rs"
            github_source_path="molecules/accordion.rs"
            title="Accordion Component"
            description={Some(html! {
                <Typo>{"The `Accordion` component toggles visibility of its children. It supports light/dark themes, custom heading tags, compact layouts, and custom arrow icons."}</Typo>
            })}
            example={example}
            usage_code={usage_code}
            props_table={Some(props_table)}
        />
    }
}
