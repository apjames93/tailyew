use wasm_bindgen::prelude::*;
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
    let points = props.points.clone();

    use_effect(move || {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let canvas = document
            .get_element_by_id("scatter-plot-canvas")
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

        // Draw points
        for point in points.iter() {
            context.set_fill_style_str(&point.color);
            context.begin_path();
            context
                .arc(
                    50.0 + point.x,
                    400.0 - point.y,
                    5.0,
                    0.0,
                    std::f64::consts::PI * 2.0,
                )
                .unwrap();
            context.fill();
        }

        || ()
    });

    html! {
        <div>
            <canvas id="scatter-plot-canvas" width="600" height="450"></canvas>
        </div>
    }
}
