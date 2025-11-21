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
pub struct BubbleChartPoint {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub color: String,
    pub label: String,
}

#[derive(Properties, PartialEq, Clone)]
pub struct BubbleChartProps {
    pub points: Vec<BubbleChartPoint>,
    #[prop_or_default]
    pub legend_position: LegendPosition,
}

const MARGIN_LEFT: f64 = 50.0;
const MARGIN_RIGHT: f64 = 20.0;
const MARGIN_TOP: f64 = 20.0;
const MARGIN_BOTTOM: f64 = 40.0;

#[derive(Clone)]
struct BubbleGeometry {
    cx: f64,
    cy: f64,
    radius: f64,
    index: usize,
}

fn draw_chart(
    canvas: &HtmlCanvasElement,
    points: &[BubbleChartPoint],
    theme: &str,
    geoms_ref: &Rc<RefCell<Vec<BubbleGeometry>>>,
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

    let max_x = points
        .iter()
        .fold(0.0_f64, |acc, p| acc.max(p.x + p.radius))
        .max(1.0);
    let max_y = points
        .iter()
        .fold(0.0_f64, |acc, p| acc.max(p.y + p.radius))
        .max(1.0);

    apply_theme_styles(&ctx, &styles);

    // X-axis
    ctx.begin_path();
    ctx.move_to(y_axis_x, x_axis_y);
    ctx.line_to(y_axis_x + plot_width, x_axis_y);
    ctx.stroke();

    // Y-axis
    ctx.begin_path();
    ctx.move_to(y_axis_x, x_axis_y);
    ctx.line_to(y_axis_x, MARGIN_TOP);
    ctx.stroke();

    // Axis labels (0,25%,50%,75%,100%)
    let steps = 4;
    for i in 0..=steps {
        let frac = i as f64 / steps as f64;
        let x_value = max_x * frac;
        let y_value = max_y * frac;

        let x = y_axis_x + frac * plot_width;
        ctx.begin_path();
        ctx.move_to(x, x_axis_y);
        ctx.line_to(x, x_axis_y + 5.0);
        ctx.stroke();
        ctx.set_text_align("center");
        let _ = ctx.fill_text(&format!("{:.0}", x_value), x, x_axis_y + 18.0);

        let y = x_axis_y - frac * plot_height;
        ctx.begin_path();
        ctx.move_to(y_axis_x - 5.0, y);
        ctx.line_to(y_axis_x, y);
        ctx.stroke();
        ctx.set_text_align("right");
        let _ = ctx.fill_text(&format!("{:.0}", y_value), y_axis_x - 8.0, y + 4.0);
    }

    {
        let mut geoms = geoms_ref.borrow_mut();
        geoms.clear();

        for (idx, point) in points.iter().enumerate() {
            let cx = y_axis_x + (point.x / max_x) * plot_width;
            let cy = x_axis_y - (point.y / max_y) * plot_height;

            ctx.set_fill_style_str(&point.color);
            ctx.begin_path();
            let _ = ctx.arc(cx, cy, point.radius, 0.0, std::f64::consts::PI * 2.0);
            ctx.fill();

            geoms.push(BubbleGeometry {
                cx,
                cy,
                radius: point.radius,
                index: idx,
            });
        }
    }
}

#[function_component(BubbleChartComponent)]
pub fn bubble_chart_component(props: &BubbleChartProps) -> Html {
    let canvas_ref = use_node_ref();
    let theme = use_get_chart_theme();
    let chart_container_ref = use_node_ref();

    let points = props.points.clone();

    let geoms_ref = use_mut_ref(Vec::<BubbleGeometry>::new);
    let hovered_index = use_state(|| None::<usize>);
    let hover_pos = use_state(|| None::<(f64, f64)>);

    let container_width = use_container_width(&chart_container_ref);

    let draw_width = if container_width > 50.0 {
        container_width
    } else {
        300.0
    };
    let draw_height = draw_width * 0.75;

    {
        let canvas_ref = canvas_ref.clone();
        let theme = theme.clone();
        let points = points.clone();
        let geoms_ref = geoms_ref.clone();

        use_effect_with(
            (theme.clone(), points.clone(), draw_width, draw_height),
            move |_| {
                if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                    draw_chart(&canvas, &points, &theme, &geoms_ref);
                }

                || ()
            },
        );
    }

    let on_mouse_move = {
        let canvas_ref = canvas_ref.clone();
        let geoms_ref = geoms_ref.clone();
        let hovered_index = hovered_index.clone();
        let hover_pos = hover_pos.clone();

        Callback::from(move |event: MouseEvent| {
            if canvas_ref.cast::<HtmlCanvasElement>().is_some() {
                let x = event.offset_x() as f64;
                let y = event.offset_y() as f64;

                let geoms = geoms_ref.borrow();
                let found = geoms
                    .iter()
                    .find(|g| {
                        let dx = x - g.cx;
                        let dy = y - g.cy;
                        dx * dx + dy * dy <= g.radius * g.radius
                    })
                    .map(|g| g.index);

                if let Some(idx) = found {
                    if *hovered_index != Some(idx) {
                        hovered_index.set(Some(idx));
                    }
                    hover_pos.set(Some((x, y - 8.0)));
                } else {
                    if hovered_index.is_some() {
                        hovered_index.set(None);
                    }
                    hover_pos.set(None);
                }
            }
        })
    };

    let on_mouse_leave = {
        let hovered_index = hovered_index.clone();
        let hover_pos = hover_pos.clone();
        Callback::from(move |_| {
            hovered_index.set(None);
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
                if let (Some(idx), Some((x, y))) = (*hovered_index, *hover_pos) {
                    let point = &points[idx];
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
                            { format!("{}: ({:.0}, {:.0}) r={:.0}", point.label, point.x, point.y, point.radius) }
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
            items={props.points.iter().map(|p| LegendItem {
                label: p.label.clone(),
                value: Some(p.radius),
                color: p.color.clone(),
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
