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

/// Geometry we use for hover hit-testing.
#[derive(Clone)]
struct BarGeometry {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

#[derive(Properties, PartialEq, Clone)]
pub struct BarChartData {
    pub label: String,
    pub value: f64,
    pub color: String,
}

#[derive(Properties, PartialEq, Clone)]
pub struct BarChartProps {
    pub data: Vec<BarChartData>,

    /// Where to place the legend relative to the chart. Defaults to `Auto`.
    #[prop_or_default]
    pub legend_position: LegendPosition,
}

const MARGIN_LEFT: f64 = 50.0;
const MARGIN_RIGHT: f64 = 20.0;
const MARGIN_TOP: f64 = 20.0;
const MARGIN_BOTTOM: f64 = 40.0;
const GAP_FRACTION: f64 = 0.10;

fn draw_chart(
    canvas: &HtmlCanvasElement,
    data: &[BarChartData],
    theme: &str,
    bar_geometries: &Rc<RefCell<Vec<BarGeometry>>>,
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

    // Axes
    apply_theme_styles(&ctx, &styles);
    ctx.begin_path();
    ctx.move_to(y_axis_x, x_axis_y);
    ctx.line_to(y_axis_x + plot_width, x_axis_y);
    ctx.stroke();

    ctx.begin_path();
    ctx.move_to(y_axis_x, x_axis_y);
    ctx.line_to(y_axis_x, MARGIN_TOP);
    ctx.stroke();

    // Prepare bar geometries
    let n = data.len().max(1) as f64;
    let max_value = data
        .iter()
        .fold(0.0_f64, |acc, d| acc.max(d.value))
        .max(1.0);

    // Simple gap model: 10% of width is gaps
    let total_gap = (plot_width * GAP_FRACTION).max(0.0);
    let gap = total_gap / (n + 1.0);
    let bar_area = plot_width - total_gap;
    let bar_width = bar_area / n;

    // Clear and repopulate geometries for hover
    {
        let mut geoms = bar_geometries.borrow_mut();
        geoms.clear();

        for (i, item) in data.iter().enumerate() {
            let i = i as f64;
            let bar_height = (item.value / max_value) * plot_height;
            let x = y_axis_x + gap + i * (bar_width + gap);
            let y = x_axis_y - bar_height;

            ctx.set_fill_style_str(&item.color);
            ctx.fill_rect(x, y, bar_width, bar_height);

            geoms.push(BarGeometry {
                x,
                y,
                width: bar_width,
                height: bar_height,
            });

            apply_theme_styles(&ctx, &styles);
            let label_y = x_axis_y + 12.0;
            let label_x = x + bar_width / 2.0;
            ctx.set_text_align("center");
            let _ = ctx.fill_text(&item.label, label_x, label_y);
        }
    }

    // Y-axis ticks (0, 25%, 50%, 75%, 100%)
    apply_theme_styles(&ctx, &styles);
    let steps = 4;
    for i in 0..=steps {
        let frac = i as f64 / steps as f64;
        let value = max_value * frac;
        let y = x_axis_y - frac * plot_height;

        ctx.begin_path();
        ctx.move_to(y_axis_x - 4.0, y);
        ctx.line_to(y_axis_x, y);
        ctx.stroke();

        ctx.set_text_align("right");
        let _ = ctx.fill_text(&format!("{:.0}", value), y_axis_x - 6.0, y + 4.0);
    }
}

#[component(BarChartComponent)]
pub fn bar_chart_component(props: &BarChartProps) -> Html {
    let canvas_ref = use_node_ref();
    let chart_container_ref = use_node_ref();

    let data = props.data.clone();
    let theme = use_get_chart_theme();

    // Track bar geometries for hover hit-testing
    let bar_geometries = use_mut_ref(Vec::<BarGeometry>::new);
    // Currently hovered bar index
    let hovered_index = use_state(|| None::<usize>);
    // Tooltip position in canvas/container CSS pixels
    let hover_pos = use_state(|| None::<(f64, f64)>);

    // Measure available width for the chart container
    let container_width = use_container_width(&chart_container_ref);

    // Fallback so first render works before ResizeObserver fires
    let draw_width = if container_width > 50.0 {
        container_width
    } else {
        300.0
    };
    let draw_height = draw_width * 0.75; // 4:3-ish aspect ratio

    // Redraw chart when theme, data, or draw size changes
    {
        let canvas_ref = canvas_ref.clone();
        let theme = theme.clone();
        let data = data.clone();
        let bar_geometries = bar_geometries.clone();

        use_effect_with(
            (theme.clone(), data.clone(), draw_width, draw_height),
            move |_| {
                if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                    draw_chart(&canvas, &data, &theme, &bar_geometries);
                }
            },
        );
    }

    // Hover handling: map mouse position to canvas coordinates and hit-test bars.
    let on_mouse_move = {
        let canvas_ref = canvas_ref.clone();
        let bar_geometries = bar_geometries.clone();
        let hovered_index = hovered_index.clone();
        let hover_pos = hover_pos.clone();

        Callback::from(move |event: MouseEvent| {
            if canvas_ref.cast::<HtmlCanvasElement>().is_some() {
                // Mouse position relative to the canvas (CSS pixels)
                let x = event.offset_x() as f64;
                let y = event.offset_y() as f64;

                let geoms = bar_geometries.borrow();
                let found = geoms
                    .iter()
                    .enumerate()
                    .find(|(_, g)| {
                        x >= g.x && x <= g.x + g.width && y >= g.y && y <= g.y + g.height
                    })
                    .map(|(i, _)| i);

                if let Some(idx) = found {
                    if *hovered_index != Some(idx) {
                        hovered_index.set(Some(idx));
                    }
                    // Position tooltip a bit above the bar / cursor
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
        // Relative so our tooltip can be absolutely positioned inside.
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
                    let bar = &data[idx];
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
                            { format!("{}: {}", bar.label, bar.value) }
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
                value: Some(format!("{}", d.value)),
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
