use crate::templates::demos::DemoComponent;
use tailyew::atoms::{TagType, Typo};
use tailyew::organisms::table::Column;
use yew::prelude::*;

const USAGE_CODE: &str = include_str!("typo_usage.rs");

#[function_component(TypoDemoSection)]
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
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "Children".into(),
                "TagType".into(),
                "Classes".into(),
                "Option<String>".into(),
                "Option<String>".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "The content rendered inside the tag.".into(),
                "Determines which semantic tag to render (H1–H5, P, Span).".into(),
                "Optional Tailwind or custom class overrides.".into(),
                "Optional inline style applied to the element.".into(),
                "Optional HTML ID for targeting or linking.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
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
