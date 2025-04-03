// frontend/src/templates/demos/a_component_demo_section.rs

use tailyew::atoms::a::A;
use tailyew::organisms::table::Column;
use yew::prelude::*;

use crate::templates::demos::DemoComponent;

// ✅ Load the same file for both rendering and display
const USAGE_CODE: &str = include_str!("./a_component_usage.rs");

#[function_component(AComponentDemoSection)]
pub fn a_component_demo_section() -> Html {
    let example: Html = include!("./a_component_usage.rs");

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "href".into(),
                "children".into(),
                "target".into(),
                "class".into(),
                "onclick".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "String".into(),
                "Children".into(),
                "Option<String>".into(),
                "Classes".into(),
                "Option<Callback<MouseEvent>>".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "URL or anchor target.".into(),
                "Content rendered inside the anchor tag.".into(),
                "Optional target attribute (e.g. \"_blank\").".into(),
                "Additional Tailwind or custom class names.".into(),
                "Optional click handler that prevents default behavior.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            title="Anchor (A) Component"
            description={Some(html! {
                <p>{"The `A` component wraps a standard HTML anchor tag with Tailwind styles and optional props like `target`, `class`, and `onclick`."}</p>
            })}
            example={example}
            usage_code={USAGE_CODE}
            props_table={Some(props_table)}
        />
    }
}
