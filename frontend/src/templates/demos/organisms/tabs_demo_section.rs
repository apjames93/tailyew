use crate::templates::demos::DemoComponent;
use tailyew::atoms::{TagType, Typo};
use tailyew::organisms::table::Column;
use tailyew::organisms::tabs::TabItem;
use tailyew::organisms::Tabs;
use yew::prelude::*;

#[function_component(TabsDemoSection)]
pub fn tabs_demo_section() -> Html {
    let items = vec![
        TabItem {
            title: "Overview".into(),
            content: html! {
                <p class="text-gray-800 dark:text-gray-300">
                    {"This is the overview tab. You can provide a summary, dashboard, or intro content here."}
                </p>
            },
        },
        TabItem {
            title: "Details".into(),
            content: html! {
                <p class="text-gray-800 dark:text-gray-300">
                    {"Here are some detailed insights, metrics, or extended documentation."}
                </p>
            },
        },
        TabItem {
            title: "Settings".into(),
            content: html! {
                <p class="text-gray-800 dark:text-gray-300">
                    {"Adjust your preferences and configuration from this tab."}
                </p>
            },
        },
    ];

    let example = html! {
        <div class="space-y-6">
            <Typo tag={TagType::H1}>{ "Tabs Component Demo" }</Typo>
            <Tabs items={items.clone()} />
        </div>
    };

    let usage_code = r#"
let items = vec![
    TabItem {
        title: "Overview".into(),
        content: html! { <p>{"This is the overview tab."}</p> },
    },
    TabItem {
        title: "Details".into(),
        content: html! { <p>{"Detailed metrics and info."}</p> },
    },
];

<Tabs items={items} />
"#;

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec!["items".into()],
        },
        Column {
            header: "Type".into(),
            values: vec!["Vec<TabItem>".into()],
        },
        Column {
            header: "Description".into(),
            values: vec!["A list of tabs with a `title` and `content` for each tab.".into()],
        },
    ];

    html! {
        <DemoComponent
            title="Tabs Component"
            description={Some(html! {
                <p>{"The `Tabs` component enables navigation between multiple content sections using a horizontal tab interface. Supports accessibility and Tailwind styling."}</p>
            })}
            example={example}
            usage_code={usage_code}
            props_table={Some(props_table)}
        />
    }
}
