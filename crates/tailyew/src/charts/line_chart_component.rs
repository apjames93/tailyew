use super::{
    chart_helpers::{
        apply_theme_styles, get_theme_styles, use_container_width, use_get_chart_theme,
    },
    chart_layout::{ChartLayout, LegendPosition},
    chart_legend::{ChartLegend, LegendItem},
};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::events::MouseEvent;
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
    #[prop_or_default]
    pub legend_position: LegendPosition,
}

const MARGIN_LEFT: f64 = 50.0;
const MARGIN_RIGHT: f64 = 20.0;
const MARGIN_TOP: f64 = 20.0;
const MARGIN_BOTTOM: f64 = 40.0;

#[derive(Clone)]
struct PointGeometry {
    x: f64,
    y: f64,
    line_index: usize,
    point_index: usize,
}

fn draw_chart(
    canvas: &HtmlCanvasElement,
    lines: &[LineChartData],
    theme: &str,
    geoms_ref: &Rc<RefCell<Vec<PointGeometry>>>,
) {
    let ctx = match canvas
        .get_context("2d")
        .ok()
        .and_then(|c| c)
        .and_then(|c| c.dyn_into::<CanvasRenderingContext2d>().ok())
    {
        Some(ctx) => ctx,
        None => return,
    };

    let styles = get_theme_styles(theme);
    let width = canvas.width() as f64;
    let height = canvas.height() as f64;

    ctx.clear_rect(0.0, 0.0, width, height);
    ctx.set_line_width(1.0);

    let plot_width = (width - MARGIN_LEFT - MARGIN_RIGHT).max(1.0);
    let plot_height = (height - MARGIN_TOP - MARGIN_BOTTOM).max(1.0);
    let x_axis_y = height - MARGIN_BOTTOM;
    let y_axis_x = MARGIN_LEFT;

    let max_x = lines
        .iter()
        .flat_map(|l| l.points.iter())
        .fold(0.0_f64, |acc, p| acc.max(p.x))
        .max(1.0);
    let max_y = lines
        .iter()
        .flat_map(|l| l.points.iter())
        .fold(0.0_f64, |acc, p| acc.max(p.y))
        .max(1.0);

    apply_theme_styles(&ctx, &styles);

    // Axes
    ctx.begin_path();
    ctx.move_to(y_axis_x, x_axis_y);
    ctx.line_to(y_axis_x + plot_width, x_axis_y);
    ctx.stroke();

    ctx.begin_path();
    ctx.move_to(y_axis_x, x_axis_y);
    ctx.line_to(y_axis_x, MARGIN_TOP);
    ctx.stroke();

    // Axis ticks (0, 25%, 50%, 75%, 100%)
    let steps = 4;
    for i in 0..=steps {
        let frac = i as f64 / steps as f64;

        let x_val = max_x * frac;
        let x = y_axis_x + frac * plot_width;
        ctx.begin_path();
        ctx.move_to(x, x_axis_y);
        ctx.line_to(x, x_axis_y + 5.0);
        ctx.stroke();
        ctx.set_text_align("center");
        let _ = ctx.fill_text(&format!("{:.0}", x_val), x, x_axis_y + 18.0);

        let y_val = max_y * frac;
        let y = x_axis_y - frac * plot_height;
        ctx.begin_path();
        ctx.move_to(y_axis_x - 5.0, y);
        ctx.line_to(y_axis_x, y);
        ctx.stroke();
        ctx.set_text_align("right");
        let _ = ctx.fill_text(&format!("{:.0}", y_val), y_axis_x - 8.0, y + 4.0);
    }

    {
        let mut geoms = geoms_ref.borrow_mut();
        geoms.clear();

        for (line_idx, line) in lines.iter().enumerate() {
            if line.points.len() < 2 {
                continue;
            }

            ctx.begin_path();
            ctx.set_line_width(2.0);
            ctx.set_stroke_style_str(&line.color);

            let to_canvas = |p: &LineChartPoint| -> (f64, f64) {
                let x = y_axis_x + (p.x / max_x) * plot_width;
                let y = x_axis_y - (p.y / max_y) * plot_height;
                (x, y)
            };

            let (start_x, start_y) = to_canvas(&line.points[0]);
            ctx.move_to(start_x, start_y);
            geoms.push(PointGeometry {
                x: start_x,
                y: start_y,
                line_index: line_idx,
                point_index: 0,
            });

            for (point_idx, point) in line.points.iter().enumerate().skip(1) {
                let (x, y) = to_canvas(point);
                ctx.line_to(x, y);
                geoms.push(PointGeometry {
                    x,
                    y,
                    line_index: line_idx,
                    point_index: point_idx,
                });
            }

            ctx.stroke();

            // Draw small markers for hover clarity
            ctx.set_fill_style_str(&line.color);
            for geom in geoms.iter().rev().take(line.points.len()) {
                ctx.begin_path();
                let _ = ctx.arc(geom.x, geom.y, 3.0, 0.0, std::f64::consts::PI * 2.0);
                ctx.fill();
            }
        }
    }

    // Reapply text style for any subsequent text
    apply_theme_styles(&ctx, &styles);
}

#[function_component(LineChartComponent)]
pub fn line_chart_component(props: &LineChartProps) -> Html {
    let canvas_ref = use_node_ref();
    let theme = use_get_chart_theme();
    let chart_container_ref = use_node_ref();

    let lines = props.lines.clone();

    let geoms_ref = use_mut_ref(Vec::<PointGeometry>::new);
    let hovered = use_state(|| None::<(usize, usize)>); // (line_idx, point_idx)
    let hover_pos = use_state(|| None::<(f64, f64)>);

    let container_width = use_container_width(&chart_container_ref);
    let draw_width = if container_width > 50.0 {
        container_width
    } else {
        600.0
    };
    let draw_height = draw_width * 0.75;

    {
        let canvas_ref = canvas_ref.clone();
        let theme = theme.clone();
        let lines = lines.clone();
        let geoms_ref = geoms_ref.clone();

        use_effect_with(
            (theme.clone(), lines.clone(), draw_width, draw_height),
            move |_| {
                if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                    draw_chart(&canvas, &lines, &theme, &geoms_ref);
                }

                || ()
            },
        );
    }

    let on_mouse_move = {
        let geoms_ref = geoms_ref.clone();
        let hovered = hovered.clone();
        let hover_pos = hover_pos.clone();

        Callback::from(move |event: MouseEvent| {
            let x = event.offset_x() as f64;
            let y = event.offset_y() as f64;

            let geoms = geoms_ref.borrow();
            let mut best: Option<(usize, f64)> = None;
            for (idx, g) in geoms.iter().enumerate() {
                let dx = x - g.x;
                let dy = y - g.y;
                let dist2 = dx * dx + dy * dy;
                if dist2 <= 10.0_f64.powi(2) && best.is_none_or(|(_, b)| dist2 < b) {
                    best = Some((idx, dist2));
                }
            }

            if let Some((idx, _)) = best {
                let g = &geoms[idx];
                hovered.set(Some((g.line_index, g.point_index)));
                hover_pos.set(Some((x, y - 8.0)));
            } else {
                if hovered.is_some() {
                    hovered.set(None);
                }
                hover_pos.set(None);
            }
        })
    };

    let on_mouse_leave = {
        let hovered = hovered.clone();
        let hover_pos = hover_pos.clone();
        Callback::from(move |_| {
            hovered.set(None);
            hover_pos.set(None);
        })
    };

    let chart = html! {
        <div ref={chart_container_ref.clone()} class="relative w-full">
            <canvas
                ref={canvas_ref}
                width={draw_width.to_string()}
                height={draw_height.to_string()}
                class="block w-full h-auto"
                onmousemove={on_mouse_move}
                onmouseleave={on_mouse_leave}
            />
            {
                if let (Some((line_idx, point_idx)), Some((x, y))) = (*hovered, *hover_pos) {
                    let line = &lines[line_idx];
                    let point = &line.points[point_idx];
                    html! {
                        <div
                            class="
                                pointer-events-none
                                absolute
                                z-10
                                -translate-x-1/2
                                -translate-y-full
                                px-2 py-1
                                rounded
                                shadow
                                text-[10px] md:text-xs
                                bg-gray-900 text-white
                            "
                            style={format!("left: {x}px; top: {y}px;")}
                        >
                            { format!("{}: ({:.0}, {:.0})", line.label, point.x, point.y) }
                        </div>
                    }
                } else {
                    Html::default()
                }
            }
        </div>
    };

    let legend = html! {
        <ChartLegend
            items={props.lines.iter().map(|line| LegendItem {
                label: line.label.clone(),
                value: Some(line.points.iter().map(|p| p.y).sum::<f64>()),
                color: line.color.clone(),
            }).collect::<Vec<LegendItem>>()}
        />
    };

    html! {
        <ChartLayout
            chart={chart}
            legend={legend}
            legend_position={props.legend_position.clone()}
        />
    }
}
