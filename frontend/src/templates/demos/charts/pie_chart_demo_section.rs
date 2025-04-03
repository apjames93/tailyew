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
            values: vec!["data".into(), "chart_id".into()],
        },
        Column {
            header: "Type".into(),
            values: vec!["Vec<PieChartData>".into(), "String".into()],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "List of segments with label, value, and color.".into(),
                "Unique ID used for canvas element and scoped styles.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
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
