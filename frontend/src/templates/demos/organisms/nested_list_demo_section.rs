use crate::templates::demos::DemoComponent;
use tailyew::organisms::table::Column;
use tailyew::organisms::{NestedItem, NestedList};
use yew::prelude::*;

#[function_component(NestedListDemoSection)]
pub fn nested_list_demo_section() -> Html {
    let on_select = Callback::from(|value: AttrValue| {
        web_sys::console::log_1(&format!("Selected: {}", value).into());
    });

    let nested_items = vec![
        NestedItem {
            value: "wow".into(),
            text: "Billing".into(),
            children: Some(vec![
                NestedItem {
                    value: "Invoices".into(),
                    text: "Invoices".into(),
                    children: None,
                },
                NestedItem {
                    value: "subscriptions_route".into(),
                    text: "Subscriptions".into(),
                    children: Some(vec![NestedItem {
                        value: "order".into(),
                        text: "Order".into(),
                        children: None,
                    }]),
                },
                NestedItem {
                    value: "value".into(),
                    text: "Past Due".into(),
                    children: None,
                },
            ]),
        },
        NestedItem {
            value: "wow".into(),
            text: "Reports".into(),
            children: None,
        },
        NestedItem {
            value: "wow".into(),
            text: "Dashboards".into(),
            children: Some(vec![
                NestedItem {
                    value: "wow".into(),
                    text: "Admin".into(),
                    children: None,
                },
                NestedItem {
                    value: "wow".into(),
                    text: "User".into(),
                    children: None,
                },
            ]),
        },
        NestedItem {
            value: "settings".into(),
            text: "Settings".into(),
            children: None,
        },
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
    NestedItem {
        text: "Reports".into(),
        children: None,
    },
    NestedItem {
        text: "Dashboards".into(),
        children: Some(vec![
            NestedItem { text: "Admin".into(), children: None },
            NestedItem { text: "User".into(), children: None },
        ]),
    },
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
                "Callback triggered when a leaf item is clicked.".into(),
                "Starting index used for alternating row striping.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            title="NestedList Component"
            description={Some(html! {
                <p>{"The `NestedList` component renders a recursive list with support for collapsible sublists using Accordions. Useful for nested navigation, categories, or drill-down structures."}</p>
            })}
            example={example}
            usage_code={usage_code}
            props_table={Some(props_table)}
        />
    }
}
