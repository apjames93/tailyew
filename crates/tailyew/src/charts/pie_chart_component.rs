use serde::Deserialize;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone, Deserialize)]
pub struct PieChartData {
    pub label: String,
    pub value: f64,
    pub color: String,
}

#[derive(Properties, PartialEq, Clone)]
pub struct PieChartProps {
    pub data: Vec<PieChartData>,

    /// Used for the canvas `id` and legend scoping if needed
    pub chart_id: String,
}

#[function_component(PieChartComponent)]
pub fn pie_chart_component(props: &PieChartProps) -> Html {
    let canvas_ref = use_node_ref();
    let data = props.data.clone();

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

            // Clear before draw
            ctx.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

            let total: f64 = data.iter().map(|d| d.value).sum();
            let mut start_angle = 0.0;

            for item in &data {
                let end_angle = start_angle + (item.value / total) * std::f64::consts::PI * 2.0;
                ctx.set_fill_style_str(&item.color);
                ctx.begin_path();
                ctx.move_to(200.0, 200.0);
                ctx.arc(200.0, 200.0, 200.0, start_angle, end_angle)
                    .unwrap();
                ctx.close_path();
                ctx.fill();
                start_angle = end_angle;
            }
        }
    });

    html! {
        <div class="flex flex-col items-center">
            <canvas
                ref={canvas_ref}
                id={format!("pie-chart-canvas-{}", props.chart_id)}
                width="400"
                height="400"
                style="margin-bottom: 1rem;"
            />
            <div class="legend space-y-1">
                { for props.data.iter().map(|item| html! {
                    <div class="flex items-center space-x-2">
                        <div
                            class="w-5 h-5 rounded"
                            style={format!("background-color: {};", item.color)}
                        ></div>
                        <span class="text-sm text-gray-700 dark:text-gray-300">{ format!("{}: {}", item.label, item.value) }</span>
                    </div>
                }) }
            </div>
        </div>
    }
}
