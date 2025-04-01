use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct LineChartPoint {
    pub x: f64,
    pub y: f64,
}

#[derive(Properties, PartialEq, Clone)]
pub struct LineChartData {
    pub label: String,
    pub points: Vec<LineChartPoint>,
    pub color: String,
}

#[derive(Properties, PartialEq, Clone)]
pub struct LineChartProps {
    pub lines: Vec<LineChartData>,
}

#[function_component(LineChartComponent)]
pub fn line_chart_component(props: &LineChartProps) -> Html {
    let lines = props.lines.clone();
    let lines_for_legend = props.lines.clone();

    use_effect(move || {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let canvas = document
            .get_element_by_id("line-chart-canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        // Clear the canvas before drawing
        context.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

        // Draw x and y axes
        context.set_stroke_style_str("#000");
        context.set_line_width(1.0);

        // X-axis
        context.begin_path();
        context.move_to(50.0, 400.0);
        context.line_to(550.0, 400.0);
        context.stroke();

        // Y-axis
        context.begin_path();
        context.move_to(50.0, 400.0);
        context.line_to(50.0, 50.0);
        context.stroke();

        // Draw x-axis and y-axis scales
        context.set_fill_style_str("#000");
        context.set_font("12px Arial");

        // X-axis scale
        for i in (0..=500).step_by(50) {
            let x = 50.0 + i as f64;
            context.begin_path();
            context.move_to(x, 400.0);
            context.line_to(x, 405.0);
            context.stroke();
            context
                .fill_text(&format!("{}", i), x - 10.0, 420.0)
                .unwrap();
        }

        // Y-axis scale
        for i in (0..=350).step_by(50) {
            let y = 400.0 - i as f64;
            context.begin_path();
            context.move_to(45.0, y);
            context.line_to(50.0, y);
            context.stroke();
            context.fill_text(&format!("{}", i), 15.0, y + 5.0).unwrap();
        }

        // Draw lines with individual colors for each line
        for line in lines.iter() {
            context.set_line_width(2.0);
            context.begin_path();

            if let Some(start_point) = line.points.first() {
                context.set_stroke_style_str(&line.color);
                context.move_to(50.0 + start_point.x, 400.0 - start_point.y);
            }

            for point in &line.points[1..] {
                context.line_to(50.0 + point.x, 400.0 - point.y);
            }

            context.stroke();
        }

        || ()
    });

    html! {
        <div>
            <canvas id="line-chart-canvas" width="600" height="450"></canvas>
            <div class="legend" style="margin-top: 10px;">
                { for lines_for_legend.iter().map(|line| html! {
                    <div style="display: flex; align-items: center; margin-bottom: 5px;">
                        <div style={format!("width: 20px; height: 2px; background-color: {}; margin-right: 10px;", line.color)}></div>
                        <span>{ &line.label }</span>
                    </div>
                })}
            </div>
        </div>
    }
}
