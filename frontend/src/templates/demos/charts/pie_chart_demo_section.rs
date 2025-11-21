use crate::templates::demos::DemoComponent;
use tailyew::charts::{PieChartComponent, PieChartData};
use tailyew::organisms::table::Column;

use yew::prelude::*;

const USAGE_CODE: &str = include_str!("pie_chart_usage.rs");

#[function_component(PieChartDemoSection)]
pub fn pie_chart_demo_section() -> Html {
    let example: Html = include!("pie_chart_usage.rs");

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
            github_demo_path="charts/pie_chart_demo_section.rs"
            github_source_path="charts/pie_chart_component.rs"
            title="PieChartComponent"
            description={Some(html! {
                <p>{"The `PieChartComponent` renders a canvas-based pie chart with a legend. Each slice is drawn based on its relative value, and color-coded for clarity."}</p>
            })}
            example={example}
            usage_code={USAGE_CODE}
            props_table={Some(props_table)}
        />
    }
}
