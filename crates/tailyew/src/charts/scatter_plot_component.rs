use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ScatterPlotPoint {
    pub x: f64,
    pub y: f64,
    pub color: String,
}

#[derive(Properties, PartialEq, Clone)]
pub struct ScatterPlotProps {
    pub points: Vec<ScatterPlotPoint>,
}

#[function_component(ScatterPlotComponent)]
pub fn scatter_plot_component(props: &ScatterPlotProps) -> Html {
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

            // Axes
            ctx.set_fill_style_str("#000");
            ctx.set_line_width(1.0);

            // X-axis
            ctx.begin_path();
            ctx.move_to(50.0, 400.0);
            ctx.line_to(550.0, 400.0);
            ctx.stroke();

            // Y-axis
            ctx.begin_path();
            ctx.move_to(50.0, 400.0);
            ctx.line_to(50.0, 50.0);
            ctx.stroke();

            // Scales
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

            // Points
            for point in &points {
                ctx.set_fill_style_str(&point.color);
                ctx.begin_path();
                let _ = ctx.arc(
                    50.0 + point.x,
                    400.0 - point.y,
                    5.0,
                    0.0,
                    std::f64::consts::PI * 2.0,
                );
                ctx.fill();
            }
        }
    });

    html! {
        <div class="flex flex-col items-center">
            <canvas
                ref={canvas_ref}
                width="600"
                height="450"
                style="display: block; margin-bottom: 1rem;"
            />
        </div>
    }
}
