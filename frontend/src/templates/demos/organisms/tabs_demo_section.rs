use crate::templates::demos::DemoComponent;
use tailyew::atoms::{TagType, Typo};
use tailyew::organisms::table::Column;
use tailyew::organisms::tabs::{TabItem, Tabs};
use yew::prelude::*;

#[component(TabsDemoSection)]
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
    let active_tab = use_state(|| 2usize);
    let on_tab_change = {
        let active_tab = active_tab.clone();
        Callback::from(move |index: usize| active_tab.set(index))
    };

    let example_default = html! {
        <div class="space-y-4">
            <Typo tag={TagType::H2}>{"Default Tabs"}</Typo>
            <Tabs items={items.clone()} />
        </div>
    };

    let example_no_scroll = html! {
        <div class="space-y-4">
            <Typo tag={TagType::H2}>{"Tabs with Explicit scroll_into_view = false"}</Typo>
            <Tabs items={items.clone()} scroll_into_view={false} />
        </div>
    };

    let example_initial_active_tab = html! {
        <div class="space-y-4">
            <Typo tag={TagType::H2}>{"Uncontrolled Tabs with initial_active_tab = 1"}</Typo>
            <Tabs items={items.clone()} initial_active_tab={1} />
        </div>
    };

    let example_controlled = html! {
        <div class="space-y-4">
            <Typo tag={TagType::H2}>{"Controlled Tabs"}</Typo>
            <Typo>{ html! { format!("Parent-controlled active tab index: {}", *active_tab) } }</Typo>
            <Tabs
                items={items.clone()}
                active_tab={Some(*active_tab)}
                on_tab_change={Some(on_tab_change.clone())}
            />
        </div>
    };

    let example = html! {
        <div class="space-y-8">
            <Typo tag={TagType::H1}>{"Tabs Component Demo"}</Typo>
            <Typo>{
                "Our `Tabs` component supports horizontal scrolling, keyboard navigation, and optional parent-controlled selection. Use `initial_active_tab` for uncontrolled defaults, `active_tab` + `on_tab_change` for controlled usage, and `scroll_into_view` to toggle smooth auto-scrolling on tab select."
            }</Typo>
            { example_default }
            { example_no_scroll }
            { example_initial_active_tab }
            { example_controlled }
        </div>
    };

    let usage_code = r#"
let items = vec![
    TabItem { title: "Overview".into(), content: html!{<Typo>{"Overview content"}</Typo>} },
    TabItem { title: "Details".into(), content: html!{<Typo>{"Details content"}</Typo>} },
    // ...more tabs
];

// Default uncontrolled usage
<Tabs items={items.clone()} />

// Explicitly keep smooth scroll disabled
<Tabs items={items.clone()} scroll_into_view={false} />

// Uncontrolled with a non-zero initial tab
<Tabs items={items.clone()} initial_active_tab={1} />

// Controlled usage
let active_tab = use_state(|| 2usize);
let on_tab_change = {
    let active_tab = active_tab.clone();
    Callback::from(move |index: usize| active_tab.set(index))
};

<Tabs
    items={items}
    active_tab={Some(*active_tab)}
    on_tab_change={Some(on_tab_change)}
/>
"#;

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "items".into(),
                "scroll_into_view".into(),
                "id_prefix".into(),
                "initial_active_tab".into(),
                "active_tab".into(),
                "on_tab_change".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "Vec<TabItem>".into(),
                "bool".into(),
                "Option<AttrValue>".into(),
                "usize".into(),
                "Option<usize>".into(),
                "Option<Callback<usize>>".into(),
            ],
        },
        Column { header: "Description".into(), values: vec![
            "List of `TabItem` (title + Html content). Tabs are horizontally scrollable, snap into view, and include focus styling.".into(),
            "When `true`, the active tab scrolls into view smoothly; when `false`, no auto-scrolling occurs.".into(),
            "Optional id prefix used to generate the tab and panel ids for aria wiring.".into(),
            "Initial active tab index for uncontrolled usage. Out-of-bounds values are clamped safely.".into(),
            "Controlled active tab index. When provided, the parent owns the rendered selection.".into(),
            "Optional callback fired with the selected tab index in both controlled and uncontrolled usage.".into(),
        ] },
    ];

    html! {
        <DemoComponent
            github_demo_path="organisms/tabs_demo_section.rs"
            github_source_path="organisms/tabs.rs"
            title="Tabs Component"
            description={Some(html! {
                <Typo>{"The `Tabs` component provides a responsive, scrollable tab interface with keyboard navigation and optional controlled selection. Use it in fully uncontrolled mode, seed it with `initial_active_tab`, or let a parent drive the active tab with `active_tab` and `on_tab_change`."}</Typo>
            })}
            example={example}
            usage_code={usage_code}
            props_table={Some(props_table)}
        />
    }
}
