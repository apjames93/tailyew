use crate::system::use_themed_classes;
use yew::prelude::*;

/// Where to render the legend relative to the chart.
#[derive(PartialEq, Clone, Default)]
pub enum LegendPosition {
    /// Responsive:
    /// - small screens: legend above chart
    /// - ≥ sm: chart left, legend right
    #[default]
    Auto,
    Right,
    Left,
    Top,
    Bottom,
    Hidden,
}

#[derive(Properties, PartialEq, Clone)]
pub struct ChartLayoutProps {
    /// The chart canvas (or chart container) HTML.
    pub chart: Html,
    /// The legend component HTML.
    pub legend: Html,
    /// Position of the legend relative to the chart.
    #[prop_or_default]
    pub legend_position: LegendPosition,
    /// Optional extra classes for the outer container.
    #[prop_or_default]
    pub class: Classes,
}

#[component(ChartLayout)]
pub fn chart_layout(props: &ChartLayoutProps) -> Html {
    let ChartLayoutProps {
        chart,
        legend,
        legend_position,
        class,
    } = props.clone();
    let root_defaults = match legend_position {
        LegendPosition::Auto => classes!(
            "flex",
            "flex-col",
            "sm:flex-row",
            "gap-4",
            "items-start",
            "w-full"
        ),
        LegendPosition::Top | LegendPosition::Bottom => {
            classes!("flex", "flex-col", "gap-3", "w-full")
        }
        LegendPosition::Left | LegendPosition::Right => {
            classes!("flex", "flex-row", "gap-4", "items-start", "w-full")
        }
        LegendPosition::Hidden => classes!("w-full"),
    };
    let layout_class = use_themed_classes("ChartLayout", "root", root_defaults, class.clone());

    match legend_position {
        // Auto: legend on top for very small screens, to the right for sm+
        LegendPosition::Auto => html! {
            <div class={layout_class.clone()}>
                // Legend: first on mobile (top), second on sm+ (right)
                <div class="order-1 sm:order-2 w-full sm:w-auto sm:shrink-0">
                    { legend }
                </div>
                // Chart: second on mobile, first on sm+ (left)
                <div class="order-2 sm:order-1 w-full sm:flex-1">
                    { chart }
                </div>
            </div>
        },

        LegendPosition::Top => html! {
            <div class={layout_class.clone()}>
                { legend }
                { chart }
            </div>
        },

        LegendPosition::Bottom => html! {
            <div class={layout_class.clone()}>
                { chart }
                { legend }
            </div>
        },

        LegendPosition::Left => html! {
            <div class={layout_class.clone()}>
                <div class="shrink-0">
                    { legend }
                </div>
                <div class="flex-1">
                    { chart }
                </div>
            </div>
        },

        LegendPosition::Right => html! {
            <div class={layout_class.clone()}>
                <div class="flex-1">
                    { chart }
                </div>
                <div class="shrink-0">
                    { legend }
                </div>
            </div>
        },
        LegendPosition::Hidden => html! {
            <div class={layout_class}>
                { chart }
            </div>
        },
    }
}
