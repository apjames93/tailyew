use crate::templates::demos::DemoComponent;
use tailyew::organisms::table::Column;
use tailyew::organisms::{NestedItem, NestedList};
use yew::prelude::*;

#[component(NestedListDemoSection)]
pub fn nested_list_demo_section() -> Html {
    let on_select = Callback::from(|value: AttrValue| {
        web_sys::console::log_1(&format!("Selected: {}", value).into());
    });

    let nested_items = vec![
        NestedItem::with_children(
            "Billing",
            vec![
                NestedItem::with_select("Invoices", "invoices"),
                NestedItem::with_children(
                    "Subscriptions",
                    vec![NestedItem::with_select("Order", "order")],
                ),
                NestedItem::with_external_link(
                    "Past Due Docs",
                    "past_due_docs",
                    "https://tailyew.com",
                ),
            ],
        ),
        NestedItem::with_select("Reports", "reports"),
        NestedItem::with_children(
            "Dashboards",
            vec![
                NestedItem::with_select("Admin", "admin"),
                NestedItem::with_select("User", "user"),
            ],
        ),
        NestedItem::with_content(
            html! { <span>{ "Settings widget slot" }</span> },
            "settings",
        ),
    ];

    let example = html! {
        <div class="max-w-md space-y-4">
            <NestedList list={nested_items} on_select={on_select.clone()} start_index={0} />
        </div>
    };

    let usage_code = r#"
let on_select = Callback::from(|value: AttrValue| {
    // handle click event
});

let nested_items = vec![
    NestedItem::with_select("Reports", "reports"),
    NestedItem::with_external_link("TailYew Docs", "docs", "https://tailyew.com"),
    NestedItem::with_children("Dashboards", vec![
        NestedItem::with_select("Admin", "admin"),
        NestedItem::with_select("User", "user"),
    ]),
];

<NestedList list={nested_items} on_select={on_select} start_index={0} />
"#;

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec!["list".into(), "on_select".into(), "start_index".into()],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "Vec<NestedItem>".into(),
                "Callback<AttrValue>".into(),
                "usize".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "Recursive list of items to render.".into(),
                "Callback triggered when a selectable or link leaf item is activated.".into(),
                "Starting index used for alternating row striping.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            github_demo_path="organisms/nested_list_demo_section.rs"
            github_source_path="organisms/nested_list.rs"
            title="NestedList Component"
            description={Some(html! {
                <p>{"The `NestedList` component renders recursive menus with semantic button rows, full-width anchor rows, and collapsible sublists. Use `with_select` for actions, `with_external_link` for URL navigation, and `with_content` for embedded widgets."}</p>
            })}
            example={example}
            usage_code={usage_code}
            props_table={Some(props_table)}
        />
    }
}
