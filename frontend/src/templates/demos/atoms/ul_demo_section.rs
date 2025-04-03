use crate::templates::demos::DemoComponent;
use tailyew::atoms::{MarkerType, Ul};
use tailyew::organisms::table::Column;
use yew::prelude::*;

const USAGE_CODE: &str = include_str!("ul_usage.rs");

#[function_component(UlDemoSection)]
pub fn ul_demo_section() -> Html {
    let example: Html = include!("ul_usage.rs");

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "children".into(),
                "class".into(),
                "spacing".into(),
                "marker_type".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "Children".into(),
                "Classes".into(),
                "Classes".into(),
                "MarkerType".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "List items inside the UL.".into(),
                "Optional extra utility classes.".into(),
                "Tailwind spacing utility like `space-y-2`.".into(),
                "Bullet marker style: Disc, Decimal, or None.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            title="Ul Component"
            description={Some(html! {
                <p>{"The `Ul` component wraps an unordered list with customizable spacing and marker style using Tailwind classes."}</p>
            })}
            example={example}
            usage_code={USAGE_CODE}
            props_table={Some(props_table)}
        />
    }
}
