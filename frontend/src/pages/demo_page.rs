// frontend/src/pages/demo_page.rs

use crate::templates::all_demo_links;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DemoPageProps {
    pub component: String,
}

#[component(DemoPage)]
pub fn demo_page(props: &DemoPageProps) -> Html {
    let DemoPageProps { component } = props;

    let content = all_demo_links()
        .iter()
        .find(|link| link.route.eq_ignore_ascii_case(component))
        .map(|link| (link.render)())
        .unwrap_or_else(|| {
            html! {
                <div class="text-center mt-16 text-red-500">
                    { format!("No demo found for component: {}", component) }
                </div>
            }
        });

    html! {
        <div class="flex">
            <div class="flex-1 p-6 overflow-auto">
                { content }
            </div>
        </div>
    }
}
