use crate::templates::demos::DemoComponent;
use tailyew::charts::{BubbleChartComponent, BubbleChartPoint};
use tailyew::organisms::table::Column;
use yew::prelude::*;

const USAGE_CODE: &str = include_str!("bubble_chart_usage.rs");

#[component(BubbleChartDemoSection)]
pub fn bubble_chart_demo_section() -> Html {
    let example: Html = include!("bubble_chart_usage.rs");

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "data".into(),
                "legend_position".into(),
                ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "Vec<BarChartData>".into(),
                "LegendPosition".into(),
                ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "Vector of bar data containing label, value, and color.".into(),
                "Where to place the legend relative to the chart. Ex: Left, Right, Top, Bottom, Auto. Auto places the legend above the chart on very small screens, and to the right on sm+ screens and is the default".into(),
                ],
        },
    ];

    html! {
        <DemoComponent
            github_demo_path="charts/bubble_chart_demo_section.rs"
            github_source_path="charts/bubble_chart_component.rs"
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
