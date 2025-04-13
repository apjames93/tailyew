use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct LegendItem {
    pub label: String,
    pub value: Option<f64>,
    pub color: String,
}

#[derive(Properties, PartialEq)]
pub struct ChartLegendProps {
    pub items: Vec<LegendItem>,
}

#[function_component(ChartLegend)]
pub fn chart_legend(props: &ChartLegendProps) -> Html {
    html! {
        <div class="legend space-y-1 max-h-[400px] overflow-y-auto pr-2 min-w-[120px]">
            { for props.items.iter().map(|item| html! {
                <div class="flex items-center space-x-2">
                    <div
                        class="w-4 h-4 rounded-full"
                        style={format!("background-color: {};", item.color)}
                    />
                    <span class="text-sm text-gray-700 dark:text-gray-300">
                        {
                            if let Some(val) = item.value {
                                format!("{}: {:.0}", item.label, val)
                            } else {
                                item.label.clone()
                            }
                        }
                    </span>
                </div>
            }) }
        </div>
    }
}
