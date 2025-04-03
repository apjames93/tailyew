use crate::templates::demos::DemoComponent;
use tailyew::charts::{BubbleChartComponent, BubbleChartPoint};
use tailyew::organisms::table::Column;
use yew::prelude::*;

const USAGE_CODE: &str = include_str!("bubble_chart_usage.rs");

#[function_component(BubbleChartDemoSection)]
pub fn bubble_chart_demo_section() -> Html {
    let example: Html = include!("bubble_chart_usage.rs");

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec!["points".into()],
        },
        Column {
            header: "Type".into(),
            values: vec!["Vec<BubbleChartPoint>".into()],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "List of points with `x`, `y`, `radius`, and `color` to render each bubble.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            title="BubbleChartComponent"
            description={Some(html! {
                <p>{"The `BubbleChartComponent` renders a dynamic scatter plot using `<canvas>`, with bubbles positioned by (x, y), colored individually, and sized via `radius`."}</p>
            })}
            example={example}
            usage_code={USAGE_CODE}
            props_table={Some(props_table)}
        />
    }
}
