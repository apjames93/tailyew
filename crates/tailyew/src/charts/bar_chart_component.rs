use super::{
    chart_helpers::{apply_theme_styles, get_theme_styles, use_get_chart_theme},
    chart_legend::{ChartLegend, LegendItem},
};
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
    let data = props.data.clone();
    let theme = use_get_chart_theme();

    // Redraw chart on theme or data change
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

                    ctx.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

                    // Axes
                    ctx.set_line_width(1.0);
                    apply_theme_styles(&ctx, &styles);

                    ctx.begin_path();
                    ctx.move_to(50.0, 400.0);
                    ctx.line_to(550.0, 400.0); // X-axis
                    ctx.stroke();

                    ctx.begin_path();
                    ctx.move_to(50.0, 400.0);
                    ctx.line_to(50.0, 50.0); // Y-axis
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

                    // Reapply styles for labels (in case fill was overwritten)
                    apply_theme_styles(&ctx, &styles);

                    // Labels - X axis
                    for i in (0..=500).step_by(50) {
                        let x = 50.0 + i as f64;
                        ctx.begin_path();
                        ctx.move_to(x, 400.0);
                        ctx.line_to(x, 405.0);
                        ctx.stroke();
                        let _ = ctx.fill_text(&format!("{i}"), x - 10.0, 420.0);
                    }

                    // Labels - Y axis
                    for i in (0..=350).step_by(50) {
                        let y = 400.0 - i as f64;
                        ctx.begin_path();
                        ctx.move_to(45.0, y);
                        ctx.line_to(50.0, y);
                        ctx.stroke();
                        let _ = ctx.fill_text(&format!("{i}"), 15.0, y + 5.0);
                    }
                }
            }

            || ()
        });
    }

    html! {
        <div class="flex flex-row items-start gap-6">
            <canvas
                ref={canvas_ref}
                width="600"
                height="450"
                class="mb-4"
            />
            <ChartLegend
                items={props.data.iter().map(|d| LegendItem {
                    label: d.label.clone(),
                    value: Some(d.value),
                    color: d.color.clone(),
                }).collect::<Vec<LegendItem>>()}
            />
        </div>
    }
}
