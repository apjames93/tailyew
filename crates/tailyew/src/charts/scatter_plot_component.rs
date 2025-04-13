use super::{
    chart_helpers::{apply_theme_styles, get_theme_styles, use_get_chart_theme},
    chart_legend::{ChartLegend, LegendItem},
};
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ScatterPlotPoint {
    pub x: f64,
    pub y: f64,
    pub color: String,
    pub label: String,
}

#[derive(Properties, PartialEq, Clone)]
pub struct ScatterPlotProps {
    pub points: Vec<ScatterPlotPoint>,
}

#[function_component(ScatterPlotComponent)]
pub fn scatter_plot_component(props: &ScatterPlotProps) -> Html {
    let canvas_ref = use_node_ref();
    let points = props.points.clone();
    let theme = use_get_chart_theme();

    {
        let canvas_ref = canvas_ref.clone();
        let theme = theme.clone();
        let points = points.clone();

        use_effect_with((theme.clone(), points.clone()), move |_| {
            if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                if let Ok(ctx) = canvas
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<CanvasRenderingContext2d>()
                {
                    let styles = get_theme_styles(&theme);
                    ctx.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

                    // Theme-aware styles
                    apply_theme_styles(&ctx, &styles);
                    ctx.set_line_width(1.0);

                    // Axes
                    ctx.begin_path();
                    ctx.move_to(50.0, 400.0);
                    ctx.line_to(550.0, 400.0);
                    ctx.stroke();

                    ctx.begin_path();
                    ctx.move_to(50.0, 400.0);
                    ctx.line_to(50.0, 50.0);
                    ctx.stroke();

                    // Axis labels
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

                    // Reapply label styles in case fill_style was overridden
                    apply_theme_styles(&ctx, &styles);
                }
            }

            || ()
        });
    }

    let legend_items = props
        .points
        .iter()
        .map(|p| LegendItem {
            label: p.label.clone(),
            value: Some(p.y),
            color: p.color.clone(),
        })
        .collect::<Vec<LegendItem>>();

    html! {
        <div class="flex flex-row items-start gap-6">
            <canvas
                ref={canvas_ref}
                width="600"
                height="450"
                class="mb-4"
            />
            <ChartLegend items={legend_items} />
        </div>
    }
}
