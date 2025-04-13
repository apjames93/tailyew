use super::{
    chart_helpers::{apply_theme_styles, get_theme_styles, use_get_chart_theme},
    chart_legend::{ChartLegend, LegendItem},
};
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct LineChartPoint {
    pub x: f64,
    pub y: f64,
}

#[derive(Properties, PartialEq, Clone)]
pub struct LineChartData {
    pub label: String,
    pub points: Vec<LineChartPoint>,
    pub color: String,
}

#[derive(Properties, PartialEq, Clone)]
pub struct LineChartProps {
    pub lines: Vec<LineChartData>,
}

#[function_component(LineChartComponent)]
pub fn line_chart_component(props: &LineChartProps) -> Html {
    let canvas_ref = use_node_ref();
    let lines = props.lines.clone();
    let theme = use_get_chart_theme();

    {
        let canvas_ref = canvas_ref.clone();
        let theme = theme.clone();
        let lines = lines.clone();

        use_effect_with((theme.clone(), lines.clone()), move |_| {
            if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                if let Ok(ctx) = canvas
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<CanvasRenderingContext2d>()
                {
                    let styles = get_theme_styles(&theme);

                    ctx.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());
                    ctx.set_line_width(1.0);

                    apply_theme_styles(&ctx, &styles);

                    // Axes
                    ctx.begin_path();
                    ctx.move_to(50.0, 400.0);
                    ctx.line_to(550.0, 400.0); // X-axis
                    ctx.stroke();

                    ctx.begin_path();
                    ctx.move_to(50.0, 400.0);
                    ctx.line_to(50.0, 50.0); // Y-axis
                    ctx.stroke();

                    // Tick Labels
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

                    // Lines
                    for line in &lines {
                        if line.points.len() < 2 {
                            continue;
                        }

                        ctx.begin_path();
                        ctx.set_line_width(2.0);
                        ctx.set_stroke_style_str(&line.color);

                        let start = &line.points[0];
                        ctx.move_to(50.0 + start.x, 400.0 - start.y);

                        for point in &line.points[1..] {
                            ctx.line_to(50.0 + point.x, 400.0 - point.y);
                        }

                        ctx.stroke();
                    }

                    // Reapply theme fill style for future text or overlay
                    apply_theme_styles(&ctx, &styles);
                }
            }

            || ()
        });
    }

    let legend_items = props
        .lines
        .iter()
        .map(|line| LegendItem {
            label: line.label.clone(),
            value: Some(line.points.iter().map(|p| p.y).sum::<f64>()),
            color: line.color.clone(),
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
