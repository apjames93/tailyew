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
            values: vec!["lines".into()],
        },
        Column {
            header: "Type".into(),
            values: vec!["Vec<LineChartData>".into()],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "An array of lines to plot, each with a label, color, and a list of (x, y) points."
                    .into(),
            ],
        },
    ];

    html! {
        <DemoComponent
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
