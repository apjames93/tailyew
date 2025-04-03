use wasm_bindgen::JsCast;
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
    let canvas_ref = use_node_ref();
    let data = props.data.clone(); // Clone once for drawing

    // Draw on first mount
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

            // Bars
            let bar_width = 50.0;
            let bar_gap = 20.0;

            for (i, item) in data.iter().enumerate() {
                let x = i as f64 * (bar_width + bar_gap) + 50.0;
                let y = 400.0 - item.value;

                ctx.set_fill_style_str(&item.color);
                ctx.fill_rect(x, y, bar_width, item.value);
            }

            // Axis text
            ctx.set_fill_style_str("#000");
            ctx.set_font("12px Arial");

            for i in (0..=500).step_by(50) {
                let x = 50.0 + i as f64;
                ctx.begin_path();
                ctx.move_to(x, 400.0);
                ctx.line_to(x, 405.0);
                ctx.stroke();
                let _ = ctx.fill_text(&format!("{}", i), x - 10.0, 420.0);
            }

            for i in (0..=350).step_by(50) {
                let y = 400.0 - i as f64;
                ctx.begin_path();
                ctx.move_to(45.0, y);
                ctx.line_to(50.0, y);
                ctx.stroke();
                let _ = ctx.fill_text(&format!("{}", i), 15.0, y + 5.0);
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
                { for props.data.iter().map(|item| html! {
                    <div class="flex items-center space-x-2">
                        <div style={format!("width: 20px; height: 20px; background-color: {};", item.color)}></div>
                        <span>{ &item.label }</span>
                    </div>
                })}
            </div>
        </div>
    }
}
