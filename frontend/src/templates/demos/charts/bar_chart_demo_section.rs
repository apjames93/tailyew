use crate::templates::demos::DemoComponent;
use tailyew::charts::{BarChartComponent, BarChartData, LegendPosition};
use tailyew::organisms::table::Column;
use yew::prelude::*;

const USAGE_CODE: &str = include_str!("bar_chart_usage.rs");

#[component(BarChartDemoSection)]
pub fn bar_chart_demo_section() -> Html {
    let example: Html = include!("bar_chart_usage.rs");

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
            github_demo_path="charts/bar_chart_demo_section.rs"
            github_source_path="charts/bar_chart_component.rs"
            title="BarChartComponent"
            description={Some(html! {
                <p>{"The `BarChartComponent` renders a canvas-based bar chart with labeled axes and a custom legend. It supports dynamic data input and color styling per bar."}</p>
            })}
            example={example}
            usage_code={USAGE_CODE}
            props_table={Some(props_table)}
        />
    }
}
