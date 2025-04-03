use crate::templates::demos::DemoComponent;
use tailyew::atoms::LinearProgressIndicator;
use tailyew::organisms::table::Column;
use yew::prelude::*;

const USAGE_CODE: &str = include_str!("linear_progress_usage.rs");

#[function_component(LinearProgressDemoSection)]
pub fn linear_progress_demo_section() -> Html {
    let example: Html = include!("linear_progress_usage.rs");

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec!["progress".into(), "class".into()],
        },
        Column {
            header: "Type".into(),
            values: vec!["usize".into(), "Classes".into()],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "Progress percent between 0 and 100.".into(),
                "Optional Tailwind classes for the inner bar.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            title="LinearProgressIndicator"
            description={Some(html! {
                <p>{"The `LinearProgressIndicator` renders a horizontal progress bar that supports animated width, color gradients, and custom styles."}</p>
            })}
            example={example}
            usage_code={USAGE_CODE}
            props_table={Some(props_table)}
        />
    }
}
