use crate::templates::demos::DemoComponent;
use tailyew::atoms::Spacer;
use tailyew::organisms::table::Column;
use yew::prelude::*;

const USAGE_CODE: &str = include_str!("spacer_usage.rs");

#[function_component(SpacerDemoSection)]
pub fn spacer_demo_section() -> Html {
    let example: Html = include!("spacer_usage.rs");

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec!["size".into(), "horizontal".into(), "class".into()],
        },
        Column {
            header: "Type".into(),
            values: vec!["u32".into(), "bool".into(), "Classes".into()],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "Size of the spacer in pixels (height or width).".into(),
                "If true, creates horizontal space instead of vertical.".into(),
                "Optional Tailwind utility classes.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            title="Spacer Component"
            description={Some(html! {
                <p>{"The `Spacer` component creates vertical or horizontal space in layouts. Useful for consistent spacing between UI elements."}</p>
            })}
            example={example}
            usage_code={USAGE_CODE}
            props_table={Some(props_table)}
        />
    }
}
