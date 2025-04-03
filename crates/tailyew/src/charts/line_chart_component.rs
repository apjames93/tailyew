use wasm_bindgen::JsCast;
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
    let canvas_ref = use_node_ref();
    let lines = props.lines.clone();

    use_effect({
        let canvas_ref = canvas_ref.clone();
        move || {
            let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() else {
                return;
            };
            let Ok(context) = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
            else {
                return;
            };

            // Clear canvas
            context.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

            // Axes
            context.set_fill_style_str("#000");
            context.set_line_width(1.0);

            // X
            context.begin_path();
            context.move_to(50.0, 400.0);
            context.line_to(550.0, 400.0);
            context.stroke();

            // Y
            context.begin_path();
            context.move_to(50.0, 400.0);
            context.line_to(50.0, 50.0);
            context.stroke();

            // Axis labels
            context.set_fill_style_str("#000");
            context.set_font("12px Arial");

            for i in (0..=500).step_by(50) {
                let x = 50.0 + i as f64;
                context.begin_path();
                context.move_to(x, 400.0);
                context.line_to(x, 405.0);
                context.stroke();
                let _ = context.fill_text(&i.to_string(), x - 10.0, 420.0);
            }

            for i in (0..=350).step_by(50) {
                let y = 400.0 - i as f64;
                context.begin_path();
                context.move_to(45.0, y);
                context.line_to(50.0, y);
                context.stroke();
                let _ = context.fill_text(&i.to_string(), 15.0, y + 5.0);
            }

            // Lines
            for line in lines.iter() {
                if line.points.len() < 2 {
                    continue;
                }

                context.begin_path();
                context.set_line_width(2.0);
                #[allow(deprecated)]
                context.set_stroke_style(&line.color.clone().into()); // bug: when updating to set_fill_style_str we lose the line colors 

                let start = &line.points[0];
                context.move_to(50.0 + start.x, 400.0 - start.y);

                for point in &line.points[1..] {
                    context.line_to(50.0 + point.x, 400.0 - point.y);
                }

                context.stroke();
            }
        }
    });

    html! {
        <div>
            <canvas
                ref={canvas_ref}
                width="600"
                height="450"
                style="display: block; margin-bottom: 1rem;"
            />
            <div class="legend space-y-1">
                { for props.lines.iter().map(|line| html! {
                    <div class="flex items-center space-x-2">
                        <div
                            class="h-1"
                            style={format!("width: 20px; background-color: {};", line.color)}
                        ></div>
                        <span>{ &line.label }</span>
                    </div>
                })}
            </div>
        </div>
    }
}
