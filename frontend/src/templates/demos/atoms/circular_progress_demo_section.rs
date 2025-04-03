// frontend/src/templates/demos/circular_progress_demo/circular_progress_demo_section.rs

use crate::templates::demos::DemoComponent;
use tailyew::atoms::CircularProgressIndicator;
use tailyew::organisms::table::Column;
use yew::prelude::*;

const USAGE_CODE: &str = include_str!("circular_progress_usage.rs");

#[function_component(CircularProgressDemoSection)]
pub fn circular_progress_demo_section() -> Html {
    let example: Html = include!("circular_progress_usage.rs");

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec!["size_class".into(), "color_class".into(), "class".into()],
        },
        Column {
            header: "Type".into(),
            values: vec!["Classes".into(), "Classes".into(), "Classes".into()],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "Sets the width and height of the spinner.".into(),
                "Controls the color of the spinning top border.".into(),
                "Optional custom Tailwind or utility classes.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            title="CircularProgressIndicator"
            description={Some(html! {
                <p>{"The `CircularProgressIndicator` component shows a spinning loader with customizable size and color."}</p>
            })}
            example={example}
            usage_code={USAGE_CODE}
            props_table={Some(props_table)}
        />
    }
}
