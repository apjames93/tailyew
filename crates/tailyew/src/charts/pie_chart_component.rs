use super::{
    chart_helpers::{
        apply_theme_styles, get_theme_styles, use_container_width, use_get_chart_theme,
    },
    chart_layout::{ChartLayout, LegendPosition},
    chart_legend::{ChartLegend, LegendItem},
};
use serde::Deserialize;
use std::{cell::RefCell, f64::consts::PI, rc::Rc};
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::events::MouseEvent;
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
    #[prop_or_default]
    pub legend_position: LegendPosition,
}

#[derive(Clone)]
struct SliceGeometry {
    start_angle: f64,
    end_angle: f64,
    radius: f64,
    cx: f64,
    cy: f64,
    index: usize,
}

fn draw_chart(
    canvas: &HtmlCanvasElement,
    data: &[PieChartData],
    theme: &str,
    geoms_ref: &Rc<RefCell<Vec<SliceGeometry>>>,
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
    let size = width.min(height);
    let radius = (size * 0.45).max(1.0);
    let cx = width / 2.0;
    let cy = height / 2.0;

    ctx.clear_rect(0.0, 0.0, width, height);

    let total: f64 = data.iter().map(|d| d.value).sum();
    if total <= 0.0 {
        geoms_ref.borrow_mut().clear();
        return;
    }

    let mut start_angle = 0.0;
    {
        let mut geoms = geoms_ref.borrow_mut();
        geoms.clear();

        for (idx, item) in data.iter().enumerate() {
            let end_angle = start_angle + (item.value / total) * PI * 2.0;
            ctx.set_fill_style_str(&item.color);
            ctx.begin_path();
            ctx.move_to(cx, cy);
            let _ = ctx.arc(cx, cy, radius, start_angle, end_angle);
            ctx.close_path();
            ctx.fill();

            geoms.push(SliceGeometry {
                start_angle,
                end_angle,
                radius,
                cx,
                cy,
                index: idx,
            });

            start_angle = end_angle;
        }
    }

    ctx.set_line_width(2.0);
    apply_theme_styles(&ctx, &styles);
    ctx.begin_path();
    let _ = ctx.arc(cx, cy, radius, 0.0, PI * 2.0);
    ctx.stroke();
}

#[function_component(PieChartComponent)]
pub fn pie_chart_component(props: &PieChartProps) -> Html {
    let canvas_ref = use_node_ref();
    let theme = use_get_chart_theme();
    let chart_container_ref = use_node_ref();

    let data = props.data.clone();

    let geoms_ref = use_mut_ref(Vec::<SliceGeometry>::new);
    let hovered_index = use_state(|| None::<usize>);
    let hover_pos = use_state(|| None::<(f64, f64)>);

    let container_width = use_container_width(&chart_container_ref);
    let draw_width = if container_width > 50.0 {
        container_width
    } else {
        300.0
    };
    let draw_height = draw_width; // keep square proportions

    {
        let canvas_ref = canvas_ref.clone();
        let theme = theme.clone();
        let data = data.clone();
        let geoms_ref = geoms_ref.clone();

        use_effect_with(
            (theme.clone(), data.clone(), draw_width, draw_height),
            move |_| {
                if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                    draw_chart(&canvas, &data, &theme, &geoms_ref);
                }

                || ()
            },
        );
    }

    let on_mouse_move = {
        let geoms_ref = geoms_ref.clone();
        let hovered_index = hovered_index.clone();
        let hover_pos = hover_pos.clone();

        Callback::from(move |event: MouseEvent| {
            let x = event.offset_x() as f64;
            let y = event.offset_y() as f64;

            let geoms = geoms_ref.borrow();
            let mut found = None;

            for geom in geoms.iter() {
                let dx = x - geom.cx;
                let dy = y - geom.cy;
                let dist2 = dx * dx + dy * dy;
                if dist2 > geom.radius * geom.radius {
                    continue;
                }
                let mut angle = dy.atan2(dx);
                if angle < 0.0 {
                    angle += PI * 2.0;
                }
                if angle >= geom.start_angle && angle <= geom.end_angle {
                    found = Some(geom.index);
                    break;
                }
            }

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
                id={format!("pie-chart-canvas-{}", props.chart_id)}
                width={draw_width.to_string()}
                height={draw_height.to_string()}
                class="block w-full h-auto"
                onmousemove={on_mouse_move}
                onmouseleave={on_mouse_leave}
            />
            {
                if let (Some(idx), Some((x, y))) = (*hovered_index, *hover_pos) {
                    let slice = &data[idx];
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
                            { format!("{}: {:.0}", slice.label, slice.value) }
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
            items={props.data.iter().map(|d| LegendItem {
                label: d.label.clone(),
                value: Some(format!("{:.0}", d.value)),
                color: d.color.clone(),
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
