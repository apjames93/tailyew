use crate::templates::demos::DemoComponent;
use tailyew::charts::{LineChartComponent, LineChartData, LineChartPoint};
use tailyew::organisms::table::Column;
use yew::prelude::*;

const USAGE_CODE: &str = include_str!("line_chart_usage.rs");

#[function_component(LineChartDemoSection)]
pub fn line_chart_demo_section() -> Html {
    let example: Html = include!("line_chart_usage.rs");

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
            github_demo_path="charts/line_chart_demo_section.rs"
            github_source_path="charts/line_chart_component.rs"
            title="LineChartComponent"
            description={Some(html! {
                <p>{"The `LineChartComponent` renders one or more connected line plots on a canvas using (x, y) data points and displays a color-coded legend."}</p>
            })}
            example={example}
            usage_code={USAGE_CODE}
            props_table={Some(props_table)}
        />
    }
}
