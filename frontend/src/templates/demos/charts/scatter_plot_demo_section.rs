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
            values: vec!["points".into()],
        },
        Column {
            header: "Type".into(),
            values: vec!["Vec<ScatterPlotPoint>".into()],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "List of points with `x`, `y`, and `color` used to render dots on the chart."
                    .into(),
            ],
        },
    ];

    html! {
        <DemoComponent
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
