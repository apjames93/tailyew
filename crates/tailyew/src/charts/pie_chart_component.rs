use super::{
    chart_helpers::{apply_theme_styles, get_theme_styles, use_get_chart_theme},
    chart_legend::{ChartLegend, LegendItem},
};
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
    pub chart_id: String,
}

#[function_component(PieChartComponent)]
pub fn pie_chart_component(props: &PieChartProps) -> Html {
    let canvas_ref = use_node_ref();
    let data = props.data.clone();
    let theme = use_get_chart_theme();

    {
        let canvas_ref = canvas_ref.clone();
        let theme = theme.clone();
        let data = data.clone();

        use_effect_with((theme.clone(), data.clone()), move |_| {
            if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                if let Ok(ctx) = canvas
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<CanvasRenderingContext2d>()
                {
                    let styles = get_theme_styles(&theme);
                    let center_x = 200.0;
                    let center_y = 200.0;
                    let radius = 200.0;

                    ctx.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

                    let total: f64 = data.iter().map(|d| d.value).sum();
                    let mut start_angle = 0.0;

                    for item in &data {
                        let end_angle =
                            start_angle + (item.value / total) * std::f64::consts::PI * 2.0;
                        ctx.set_fill_style_str(&item.color);
                        ctx.begin_path();
                        ctx.move_to(center_x, center_y);
                        ctx.arc(center_x, center_y, radius, start_angle, end_angle)
                            .unwrap();
                        ctx.close_path();
                        ctx.fill();
                        start_angle = end_angle;
                    }

                    // Outline circle
                    ctx.set_line_width(2.0);
                    apply_theme_styles(&ctx, &styles);
                    ctx.begin_path();
                    ctx.arc(center_x, center_y, radius, 0.0, std::f64::consts::PI * 2.0)
                        .unwrap();
                    ctx.stroke();
                }
            }

            || ()
        });
    }

    let legend_items = props
        .data
        .iter()
        .map(|d| LegendItem {
            label: d.label.clone(),
            value: Some(d.value),
            color: d.color.clone(),
        })
        .collect::<Vec<LegendItem>>();

    html! {
        <div class="flex flex-row items-start gap-6">
            <canvas
                ref={canvas_ref}
                id={format!("pie-chart-canvas-{}", props.chart_id)}
                width="400"
                height="400"
                class="mb-4"
            />
            <ChartLegend items={legend_items} />
        </div>
    }
}
