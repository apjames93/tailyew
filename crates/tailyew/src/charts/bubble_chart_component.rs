use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct BubbleChartPoint {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub color: String,
}

#[derive(Properties, PartialEq, Clone)]
pub struct BubbleChartProps {
    pub points: Vec<BubbleChartPoint>,
}

#[function_component(BubbleChartComponent)]
pub fn bubble_chart_component(props: &BubbleChartProps) -> Html {
    let canvas_ref = use_node_ref();
    let points = props.points.clone();

    use_effect({
        let canvas_ref = canvas_ref.clone();
        move || {
            let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() else {
                return;
            };

            let Ok(ctx) = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
            else {
                return;
            };

            // Clear canvas
            ctx.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

            // Draw axes
            ctx.set_fill_style_str("#000");
            ctx.set_line_width(1.0);

            ctx.begin_path();
            ctx.move_to(50.0, 400.0);
            ctx.line_to(550.0, 400.0); // X-axis
            ctx.stroke();

            ctx.begin_path();
            ctx.move_to(50.0, 400.0);
            ctx.line_to(50.0, 50.0); // Y-axis
            ctx.stroke();

            // Axis labels
            ctx.set_fill_style_str("#000");
            ctx.set_font("12px Arial");

            for i in (0..=500).step_by(50) {
                let x = 50.0 + i as f64;
                ctx.begin_path();
                ctx.move_to(x, 400.0);
                ctx.line_to(x, 405.0);
                ctx.stroke();
                let _ = ctx.fill_text(&i.to_string(), x - 10.0, 420.0);
            }

            for i in (0..=350).step_by(50) {
                let y = 400.0 - i as f64;
                ctx.begin_path();
                ctx.move_to(45.0, y);
                ctx.line_to(50.0, y);
                ctx.stroke();
                let _ = ctx.fill_text(&i.to_string(), 15.0, y + 5.0);
            }

            // Draw bubbles
            for point in &points {
                ctx.set_fill_style_str(&point.color);
                ctx.begin_path();
                let _ = ctx.arc(
                    50.0 + point.x,
                    400.0 - point.y,
                    point.radius,
                    0.0,
                    std::f64::consts::PI * 2.0,
                );
                ctx.fill();
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
        </div>
    }
}
