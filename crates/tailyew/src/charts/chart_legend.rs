use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct LegendItem {
    pub label: String,
    pub value: Option<String>,
    pub color: String,
}

#[derive(Properties, PartialEq)]
pub struct ChartLegendProps {
    pub items: Vec<LegendItem>,
}

#[component(ChartLegend)]
pub fn chart_legend(props: &ChartLegendProps) -> Html {
    html! {
        <div
            class="
            space-y-1
                max-h-[200px]
                overflow-y-auto
                pr-2
            "
        >
            { for props.items.iter().map(|item| html! {
                <div class="flex items-center space-x-2">
                    <div
                        class="w-3 h-3 rounded-full"
                        style={format!("background-color: {};", item.color)}
                    />
                    <span class="text-xs md:text-sm text-gray-700 dark:text-gray-300 leading-tight">
                        {
                            if let Some(val) = &item.value {
                                format!("{}: {}", item.label, val)
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
