use crate::templates::demos::DemoComponent;
use tailyew::atoms::{IconPosition, Li, MarkerType, Section, TagType, Typo, Ul};
use tailyew::organisms::table::Column;
use yew::prelude::*;

const USAGE_CODE: &str = include_str!("li_usage.rs");

#[component(LiDemoSection)]
pub fn li_demo_section() -> Html {
    let example: Html = include!("li_usage.rs");

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "children".into(),
                "class".into(),
                "active".into(),
                "hover".into(),
                "icon_position".into(),
                "icon".into(),
                "bordered".into(),
                "background".into(),
                "on_click".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "Children".into(),
                "Classes".into(),
                "bool".into(),
                "bool".into(),
                "IconPosition".into(),
                "Option<Html>".into(),
                "bool".into(),
                "Classes".into(),
                "Option<Callback<MouseEvent>>".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "Content inside the list item.".into(),
                "Additional class names.".into(),
                "Highlights the item as active.".into(),
                "Adds a hover background effect.".into(),
                "Aligns the optional icon to the left or right of the content.".into(),
                "HTML content rendered inside the optional icon slot.".into(),
                "Adds a bottom border.".into(),
                "Controls background utility classes.".into(),
                "Optional click handler.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            github_demo_path="atoms/li_demo_section.rs"
            github_source_path="atoms/li.rs"
            title="Li Component"
            description={Some(html! {
                <p>{"The `Li` component is a styled list item that supports hover, active, icons, borders, and more."}</p>
            })}
            example={example}
            usage_code={USAGE_CODE}
            props_table={Some(props_table)}
        />
    }
}
