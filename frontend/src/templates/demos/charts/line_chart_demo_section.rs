use crate::templates::demos::DemoComponent;
use tailyew::atoms::{TagType, Typo};
use tailyew::charts::{
    LegendPosition, LineChartComponent, LineChartData, LineChartPoint, LineChartTooltipFormatter,
    LineChartValueFormatter,
};
use tailyew::organisms::table::Column;
use yew::prelude::*;

const USAGE_CODE: &str = include_str!("line_chart_usage.rs");

fn demo_line(label: &str, color: &str, points: &[(f64, f64)]) -> LineChartData {
    LineChartData {
        label: label.into(),
        color: color.into(),
        points: points
            .iter()
            .map(|(x, y)| LineChartPoint { x: *x, y: *y })
            .collect(),
    }
}

fn percent_formatter(value: f64) -> String {
    format!("{:.1}%", value * 100.0)
}

#[component(LineChartDemoSection)]
pub fn line_chart_demo_section() -> Html {
    let usage_example: Html = include!("line_chart_usage.rs");

    let example = html! {
        <div class="w-full max-w-6xl mx-auto text-left">
            <div class="grid gap-6 lg:grid-cols-2">
                <div class="rounded-lg border border-gray-200 bg-white p-4 dark:border-gray-700 dark:bg-gray-900/60">
                    <Typo tag={TagType::H4} class="mb-1">{ "Auto (Default)" }</Typo>
                    <Typo tag={TagType::P} class="mb-4 text-xs text-gray-600 dark:text-gray-300">
                        { "No formatter props needed. Fractional values stay readable without forcing whole-number labels." }
                    </Typo>
                    <LineChartComponent
                        legend_position={LegendPosition::Bottom}
                        lines={vec![
                            demo_line(
                                "Requests",
                                "#3b82f6",
                                &[(1.0, 0.125), (2.0, 0.3333), (3.0, 0.52), (4.0, 0.875)],
                            ),
                            demo_line(
                                "Retries",
                                "#10b981",
                                &[(1.0, 0.08), (2.0, 0.14), (3.0, 0.19), (4.0, 0.24)],
                            ),
                        ]}
                    />
                </div>

                <div class="rounded-lg border border-gray-200 bg-white p-4 dark:border-gray-700 dark:bg-gray-900/60">
                    <Typo tag={TagType::H4} class="mb-1">{ "Fixed Precision" }</Typo>
                    <Typo tag={TagType::P} class="mb-4 text-xs text-gray-600 dark:text-gray-300">
                        { "Useful when you want a stable number of decimal places on the axis and in the tooltip." }
                    </Typo>
                    <LineChartComponent
                        legend_position={LegendPosition::Hidden}
                        x_axis_formatter={LineChartValueFormatter::Integer}
                        y_axis_formatter={LineChartValueFormatter::fixed(3)}
                        tooltip_formatter={LineChartTooltipFormatter::value_pair(
                            LineChartValueFormatter::Integer,
                            LineChartValueFormatter::fixed(4),
                        )}
                        lines={vec![
                            demo_line(
                                "Latency",
                                "#f59e0b",
                                &[(1.0, 12.3456), (2.0, 10.2001), (3.0, 8.9012), (4.0, 7.3333)],
                            ),
                        ]}
                    />
                </div>

                <div class="rounded-lg border border-gray-200 bg-white p-4 dark:border-gray-700 dark:bg-gray-900/60">
                    <Typo tag={TagType::H4} class="mb-1">{ "Currency" }</Typo>
                    <Typo tag={TagType::P} class="mb-4 text-xs text-gray-600 dark:text-gray-300">
                        { "A spend-oriented demo with cents on the axis and more precision when hovering." }
                    </Typo>
                    { usage_example.clone() }
                </div>

                <div class="rounded-lg border border-gray-200 bg-white p-4 dark:border-gray-700 dark:bg-gray-900/60">
                    <Typo tag={TagType::H4} class="mb-1">{ "Custom" }</Typo>
                    <Typo tag={TagType::P} class="mb-4 text-xs text-gray-600 dark:text-gray-300">
                        { "Custom formatters let you render percentages or any other domain-specific unit." }
                    </Typo>
                    <LineChartComponent
                        legend_position={LegendPosition::Hidden}
                        x_axis_formatter={LineChartValueFormatter::Integer}
                        y_axis_formatter={LineChartValueFormatter::Custom(percent_formatter)}
                        lines={vec![
                            demo_line(
                                "Cache Hit Rate",
                                "#8b5cf6",
                                &[(1.0, 0.912), (2.0, 0.945), (3.0, 0.968), (4.0, 0.982)],
                            ),
                        ]}
                    />
                </div>
            </div>
        </div>
    };

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "lines".into(),
                "legend_position".into(),
                "x_axis_formatter".into(),
                "y_axis_formatter".into(),
                "tooltip_formatter".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "Vec<LineChartData>".into(),
                "LegendPosition".into(),
                "LineChartValueFormatter".into(),
                "LineChartValueFormatter".into(),
                "LineChartTooltipFormatter".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "Series data for one or more lines, where each series contains a label, color, and `(x, y)` points.".into(),
                "Where to place the legend relative to the chart. Ex: Left, Right, Top, Bottom, Auto. Auto places the legend above the chart on very small screens, and to the right on sm+ screens and is the default".into(),
                "Formats X-axis tick labels. Defaults to decimal-friendly automatic formatting.".into(),
                "Formats Y-axis tick labels. Useful for money, usage, and other fractional values.".into(),
                "Formats tooltip text. By default it reuses the axis formatters, but it can be overridden for higher-precision hover output.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            github_demo_path="charts/line_chart_demo_section.rs"
            github_source_path="charts/line_chart_component.rs"
            title="LineChartComponent"
            description={Some(html! {
                <p>{"The `LineChartComponent` renders one or more connected line plots on a canvas using `(x, y)` data points. It supports decimal-friendly axis and tooltip formatting, so the same component works for spend charts, usage charts, and simple whole-number series."}</p>
            })}
            example={example}
            usage_code={USAGE_CODE}
            props_table={Some(props_table)}
        />
    }
}
