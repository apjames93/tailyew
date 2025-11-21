use crate::templates::demos::DemoComponent;
use tailyew::charts::{ScatterPlotComponent, ScatterPlotPoint};
use tailyew::organisms::table::Column;
use yew::prelude::*;

const USAGE_CODE: &str = include_str!("scatter_plot_usage.rs");

#[function_component(ScatterPlotDemoSection)]
pub fn scatter_plot_demo_section() -> Html {
    let example: Html = include!("scatter_plot_usage.rs");

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
            github_demo_path="charts/scatter_plot_demo_section.rs"
            github_source_path="charts/scatter_plot_component.rs"
            title="ScatterPlotComponent"
            description={Some(html! {
                <p>{"The `ScatterPlotComponent` renders a canvas-based scatter plot using a list of colored data points. Axes and tick marks are included for scale."}</p>
            })}
            example={example}
            usage_code={USAGE_CODE}
            props_table={Some(props_table)}
        />
    }
}
