use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct BarChartData {
    pub label: String,
    pub value: f64,
    pub color: String,
}

#[derive(Properties, PartialEq, Clone)]
pub struct BarChartProps {
    pub data: Vec<BarChartData>,
}

#[function_component(BarChartComponent)]
pub fn bar_chart_component(props: &BarChartProps) -> Html {
    let data = props.data.clone();
    let data_ref = props.data.clone(); // Clone for legend rendering

    use_effect(move || {
        let data_clone = data.clone();
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let canvas = document
            .get_element_by_id("bar-chart-canvas")
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

        // Draw bars
        let bar_width = 50.0;
        let bar_gap = 20.0;

        for (i, item) in data_clone.iter().enumerate() {
            let x = i as f64 * (bar_width + bar_gap) + 50.0;
            let y = 400.0 - item.value;

            context.set_fill_style_str(&item.color);
            context.fill_rect(x, y, bar_width, item.value);
        }

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

        || ()
    });

    html! {
        <div>
            <canvas id="bar-chart-canvas" width="600" height="450"></canvas>
            <div class="legend" style="margin-top: 10px;">
                { for data_ref.iter().map(|item| html! {
                    <div style="display: flex; align-items: center; margin-bottom: 5px;">
                        <div style={format!("width: 20px; height: 20px; background-color: {}; margin-right: 10px;", &item.color)}></div>
                        <span>{ &item.label }</span>
                    </div>
                })}
            </div>
        </div>
    }
}
