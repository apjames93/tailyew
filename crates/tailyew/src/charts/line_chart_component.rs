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
    /// Formats X-axis tick labels. Defaults to decimal-friendly automatic formatting.
    #[prop_or_default]
    pub x_axis_formatter: LineChartValueFormatter,
    /// Formats Y-axis tick labels. Defaults to decimal-friendly automatic formatting.
    #[prop_or_default]
    pub y_axis_formatter: LineChartValueFormatter,
    /// Formats tooltip content. By default it reuses the axis formatters.
    #[prop_or_default]
    pub tooltip_formatter: LineChartTooltipFormatter,
}

const MARGIN_LEFT: f64 = 50.0;
const MARGIN_RIGHT: f64 = 20.0;
const MARGIN_TOP: f64 = 20.0;
const MARGIN_BOTTOM: f64 = 40.0;
const AXIS_TICK_STEPS: usize = 4;
const AUTO_FORMAT_MAX_DECIMALS: usize = 6;
const WHOLE_NUMBER_EPSILON: f64 = 1e-9;

/// A custom value formatter for axis labels and tooltip values.
pub type LineChartValueFormatFn = fn(f64) -> String;

/// A custom formatter for the full tooltip label.
pub type LineChartTooltipFormatFn = for<'a> fn(LineChartTooltipContext<'a>) -> String;

/// Context provided to custom tooltip formatters.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LineChartTooltipContext<'a> {
    pub line_label: &'a str,
    pub line_index: usize,
    pub point_index: usize,
    pub x: f64,
    pub y: f64,
}

/// Formatting options for axis labels and tooltip values.
#[derive(Clone, Debug, Default)]
pub enum LineChartValueFormatter {
    /// Formats whole values without decimals and fractional values with trimmed decimals.
    #[default]
    Auto,
    /// Formats values as whole numbers.
    Integer,
    /// Formats values with a fixed decimal precision.
    Fixed(usize),
    /// Formats values as currency with a symbol prefix and fixed precision.
    Currency { symbol: String, precision: usize },
    /// Uses a custom formatter.
    Custom(LineChartValueFormatFn),
}

impl PartialEq for LineChartValueFormatter {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Auto, Self::Auto) | (Self::Integer, Self::Integer) => true,
            (Self::Fixed(left), Self::Fixed(right)) => left == right,
            (
                Self::Currency {
                    symbol: left_symbol,
                    precision: left_precision,
                },
                Self::Currency {
                    symbol: right_symbol,
                    precision: right_precision,
                },
            ) => left_symbol == right_symbol && left_precision == right_precision,
            (Self::Custom(left), Self::Custom(right)) => std::ptr::fn_addr_eq(*left, *right),
            _ => false,
        }
    }
}

impl LineChartValueFormatter {
    pub fn fixed(precision: usize) -> Self {
        Self::Fixed(precision)
    }

    pub fn currency(symbol: impl Into<String>, precision: usize) -> Self {
        Self::Currency {
            symbol: symbol.into(),
            precision,
        }
    }

    fn format_value(&self, value: f64) -> String {
        match self {
            Self::Auto => format_auto_value(value),
            Self::Integer => format!("{:.0}", value),
            Self::Fixed(precision) => format!("{value:.precision$}"),
            Self::Currency { symbol, precision } => {
                let absolute = value.abs();
                if value.is_sign_negative() {
                    format!("-{symbol}{absolute:.precision$}")
                } else {
                    format!("{symbol}{value:.precision$}")
                }
            }
            Self::Custom(formatter) => formatter(value),
        }
    }
}

/// Formatting options for line-chart tooltips.
#[derive(Clone, Debug, Default)]
pub enum LineChartTooltipFormatter {
    /// Reuses the configured axis formatters.
    #[default]
    Default,
    /// Uses explicit formatters for the tooltip X/Y values.
    ValuePair {
        x: LineChartValueFormatter,
        y: LineChartValueFormatter,
    },
    /// Uses a custom formatter for the full tooltip label.
    Custom(LineChartTooltipFormatFn),
}

impl PartialEq for LineChartTooltipFormatter {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Default, Self::Default) => true,
            (
                Self::ValuePair {
                    x: left_x,
                    y: left_y,
                },
                Self::ValuePair {
                    x: right_x,
                    y: right_y,
                },
            ) => left_x == right_x && left_y == right_y,
            (Self::Custom(left), Self::Custom(right)) => std::ptr::fn_addr_eq(*left, *right),
            _ => false,
        }
    }
}

impl LineChartTooltipFormatter {
    pub fn value_pair(x: LineChartValueFormatter, y: LineChartValueFormatter) -> Self {
        Self::ValuePair { x, y }
    }

    fn format(
        &self,
        context: LineChartTooltipContext<'_>,
        x_axis_formatter: &LineChartValueFormatter,
        y_axis_formatter: &LineChartValueFormatter,
    ) -> String {
        match self {
            Self::Default => default_tooltip_text(context, x_axis_formatter, y_axis_formatter),
            Self::ValuePair { x, y } => default_tooltip_text(context, x, y),
            Self::Custom(formatter) => formatter(context),
        }
    }
}

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
    x_axis_formatter: &LineChartValueFormatter,
    y_axis_formatter: &LineChartValueFormatter,
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
    for i in 0..=AXIS_TICK_STEPS {
        let frac = i as f64 / AXIS_TICK_STEPS as f64;

        let x_val = max_x * frac;
        let x = y_axis_x + frac * plot_width;
        ctx.begin_path();
        ctx.move_to(x, x_axis_y);
        ctx.line_to(x, x_axis_y + 5.0);
        ctx.stroke();
        ctx.set_text_align("center");
        let _ = ctx.fill_text(&x_axis_formatter.format_value(x_val), x, x_axis_y + 18.0);

        let y_val = max_y * frac;
        let y = x_axis_y - frac * plot_height;
        ctx.begin_path();
        ctx.move_to(y_axis_x - 5.0, y);
        ctx.line_to(y_axis_x, y);
        ctx.stroke();
        ctx.set_text_align("right");
        let _ = ctx.fill_text(
            &y_axis_formatter.format_value(y_val),
            y_axis_x - 8.0,
            y + 4.0,
        );
    }

    {
        let mut geoms = geoms_ref.borrow_mut();
        geoms.clear();

        for (line_idx, line) in lines.iter().enumerate() {
            if line.points.is_empty() {
                continue;
            }

            ctx.begin_path();
            ctx.set_line_width(2.0);
            ctx.set_stroke_style_str(&line.color);
            let geom_start = geoms.len();

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
            for geom in geoms.iter().skip(geom_start) {
                ctx.begin_path();
                let _ = ctx.arc(geom.x, geom.y, 3.0, 0.0, std::f64::consts::PI * 2.0);
                ctx.fill();
            }
        }
    }

    // Reapply text style for any subsequent text
    apply_theme_styles(&ctx, &styles);
}

#[component(LineChartComponent)]
pub fn line_chart_component(props: &LineChartProps) -> Html {
    let canvas_ref = use_node_ref();
    let theme = use_get_chart_theme();
    let chart_container_ref = use_node_ref();

    let lines = props.lines.clone();
    let x_axis_formatter = props.x_axis_formatter.clone();
    let y_axis_formatter = props.y_axis_formatter.clone();
    let tooltip_formatter = props.tooltip_formatter.clone();

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
        let x_axis_formatter = x_axis_formatter.clone();
        let y_axis_formatter = y_axis_formatter.clone();
        let geoms_ref = geoms_ref.clone();

        use_effect_with(
            (
                theme.clone(),
                lines.clone(),
                draw_width,
                draw_height,
                x_axis_formatter.clone(),
                y_axis_formatter.clone(),
            ),
            move |_| {
                if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                    draw_chart(
                        &canvas,
                        &lines,
                        &theme,
                        &x_axis_formatter,
                        &y_axis_formatter,
                        &geoms_ref,
                    );
                }
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
                let next_hovered = Some((g.line_index, g.point_index));
                if *hovered != next_hovered {
                    hovered.set(next_hovered);
                }
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
                    let tooltip_text = tooltip_formatter.format(
                        LineChartTooltipContext {
                            line_label: &line.label,
                            line_index: line_idx,
                            point_index: point_idx,
                            x: point.x,
                            y: point.y,
                        },
                        &x_axis_formatter,
                        &y_axis_formatter,
                    );
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
                            { tooltip_text }
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
                value: None,
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

fn default_tooltip_text(
    context: LineChartTooltipContext<'_>,
    x_formatter: &LineChartValueFormatter,
    y_formatter: &LineChartValueFormatter,
) -> String {
    format!(
        "{}: ({}, {})",
        context.line_label,
        x_formatter.format_value(context.x),
        y_formatter.format_value(context.y)
    )
}

fn format_auto_value(value: f64) -> String {
    if value.abs() < WHOLE_NUMBER_EPSILON {
        return "0".to_string();
    }

    if (value.round() - value).abs() < WHOLE_NUMBER_EPSILON {
        return format!("{:.0}", value.round());
    }

    if value.abs() < 10_f64.powi(-(AUTO_FORMAT_MAX_DECIMALS as i32)) {
        return format!("{value:.2e}");
    }

    trim_trailing_zeros(format!(
        "{value:.precision$}",
        precision = AUTO_FORMAT_MAX_DECIMALS
    ))
}

fn trim_trailing_zeros(mut value: String) -> String {
    if value.contains('.') {
        while value.ends_with('0') {
            value.pop();
        }

        if value.ends_with('.') {
            value.pop();
        }
    }

    if value == "-0" {
        "0".to_string()
    } else {
        value
    }
}
