use crate::templates::demos::DemoComponent;
use tailyew::atoms::{TagType, Typo};
use tailyew::organisms::table::Column;
use yew::prelude::*;

const USAGE_CODE: &str = include_str!("typo_usage.rs");

#[component(TypoDemoSection)]
pub fn typo_demo_section() -> Html {
    let example: Html = include!("typo_usage.rs");

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "children".into(),
                "tag".into(),
                "class".into(),
                "style".into(),
                "id".into(),
                "aria_label".into(),
                "aria_describedby".into(),
                "role".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "Children".into(),
                "TagType".into(),
                "Classes".into(),
                "Option<AttrValue>".into(),
                "Option<AttrValue>".into(),
                "Option<AttrValue>".into(),
                "Option<AttrValue>".into(),
                "Option<AttrValue>".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "The content rendered inside the tag.".into(),
                "Determines which semantic tag to render (H1–H6, P, Span, etc.).".into(),
                "Optional Tailwind or custom class overrides.".into(),
                "Optional inline style applied to the element (use sparingly; prefer classes)."
                    .into(),
                "Optional HTML ID for targeting or linking.".into(),
                "Optional label for screen readers.".into(),
                "ID of an element that describes this one.".into(),
                "ARIA role override (e.g., alert, status, presentation).".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            github_demo_path="atoms/typo_demo_section.rs"
            github_source_path="atoms/typo.rs"
            title="Typo Component"
            description={Some(html! {
                <p>{"The `Typo` component renders semantic HTML tags (like H1–H5, P, or Span) with opinionated styling and customization."}</p>
            })}
            example={example}
            usage_code={USAGE_CODE}
            props_table={Some(props_table)}
        />
    }
}
