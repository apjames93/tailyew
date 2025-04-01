// cp/organisms/pie_chart_component.rs

use serde::Deserialize;
use wasm_bindgen::prelude::*;
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
    pub chart_id: String,
}

#[function_component(PieChartComponent)]
pub fn pie_chart_component(props: &PieChartProps) -> Html {
    let data = props.data.clone();
    let data_for_legend = props.data.clone();
    let chart_id = format!("pie-chart-canvas-{}", props.chart_id.clone());
    let chart_id_copy = chart_id.clone();

    use_effect(move || {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let canvas = document
            .get_element_by_id(&chart_id_copy)
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        let total: f64 = data.iter().map(|d| d.value).sum();
        let mut start_angle = 0.0;

        for item in data.iter() {
            let end_angle = start_angle + (item.value / total) * std::f64::consts::PI * 2.0;
            context.set_fill_style_str(&item.color);
            context.begin_path();
            context.move_to(200.0, 200.0);
            context
                .arc(200.0, 200.0, 200.0, start_angle, end_angle)
                .unwrap();
            context.close_path();
            context.fill();
            start_angle = end_angle;
        }

        || ()
    });

    html! {
        <div>
            <canvas id={chart_id} width="400" height="400"></canvas>
            <div class="legend">
                { for data_for_legend.iter().map(|item| html! {
                    <div style="display: flex; align-items: center;">
                        <div style={format!("width: 20px; height: 20px; background-color: {}; margin-right: 10px;", item.color)}></div>
                        <span>{ format!("{}: {}", item.label, item.value) }</span>
                    </div>
                })}
            </div>
        </div>
    }
}
