use crate::templates::demos::DemoComponent;
use tailyew::atoms::{TagType, Typo};
use tailyew::organisms::table::Column;
use tailyew::organisms::tabs::{TabItem, Tabs};
use yew::prelude::*;

#[function_component(TabsDemoSection)]
pub fn tabs_demo_section() -> Html {
    // Sample tab data
    let items = vec![
        TabItem {
            title: "Overview".into(),
            content: html! { <Typo>{"This is the overview tab. Great for summaries, dashboards, or intro content."}</Typo> },
        },
        TabItem {
            title: "Details".into(),
            content: html! { <Typo>{"Here are detailed insights, metrics, or extended documentation."}</Typo> },
        },
        TabItem {
            title: "Settings".into(),
            content: html! { <Typo>{"Adjust your preferences and configuration here."}</Typo> },
        },
        TabItem {
            title: "Long Tab Label Example".into(),
            content: html! { <Typo>{"Demonstrates scrollable behavior with long tab names on narrow screens."}</Typo> },
        },
    ];

    let example_default = html! {
        <div class="space-y-4">
            <Typo tag={TagType::H2}>{"Default Tabs (scroll_into_view = true)"}</Typo>
            <Tabs items={items.clone()} scroll_into_view={true}/>
        </div>
    };

    let example_no_scroll = html! {
        <div class="space-y-4">
            <Typo tag={TagType::H2}>{"Tabs with Smooth Scroll Disabled (scroll_into_view = false)"}</Typo>
            <Tabs items={items.clone()} scroll_into_view={false} />
        </div>
    };

    let example = html! {
        <div class="space-y-8">
            <Typo tag={TagType::H1}>{"Tabs Component Demo"}</Typo>
            <Typo>{
                "Our `Tabs` component supports horizontal scrolling, snap-to-tab, and keyboard focus rings—all via built-in Tailwind classes. Use the `scroll_into_view` prop to toggle smooth auto-scrolling on tab select."
            }</Typo>
            { example_default }
            { example_no_scroll }
        </div>
    };

    let usage_code = r#"
let items = vec![
    TabItem { title: "Overview".into(), content: html!{<Typo>{"Overview content"}</Typo>} },
    TabItem { title: "Details".into(), content: html!{<Typo>{"Details content"}</Typo>} },
    // ...more tabs
];

// Default (smooth scroll enabled)
<Tabs items={items.clone()} />

// Disable smooth scroll
<Tabs items={items} scroll_into_view={false} />
"#;

    let props_table = vec![
        Column { header: "Prop".into(), values: vec!["items".into(), "scroll_into_view".into()] },
        Column { header: "Type".into(), values: vec!["Vec<TabItem>".into(), "bool".into()] },
        Column { header: "Description".into(), values: vec![
            "List of `TabItem` (title + Html content). Tabs are horizontally scrollable, snap into view, and include focus styling.".into(),
            "When `true`, the active tab scrolls into view smoothly; when `false`, no auto-scrolling occurs.".into(),
        ] },
    ];

    html! {
        <DemoComponent
            github_demo_path="organisms/tabs_demo_section.rs"
            github_source_path="organisms/tabs.rs"
            title="Tabs Component"
            description={Some(html! {
                <Typo>{"The `Tabs` component provides a responsive, scrollable tab interface out of the box—no extra CSS required. It automatically snaps tabs into view on selection and includes keyboard focus rings for accessibility. Toggle `scroll_into_view` to control smooth auto-scrolling."}</Typo>
            })}
            example={example}
            usage_code={usage_code}
            props_table={Some(props_table)}
        />
    }
}
