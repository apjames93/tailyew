use crate::templates::demos::DemoComponent;
use tailyew::organisms::table::Column;
use yew::prelude::*;

#[function_component(SidebarDemoSection)]
pub fn sidebar_demo_section() -> Html {
    let usage_code = r#"
<Sidebar
    icon_list={vec![
        SidebarButton {
            icon: html! { <YourIconSvg /> },
            list: vec![NestedItem::new("Example")],
        }
    ]}
    on_select={Callback::from(|val| log::info!("{val}"))}
/>
"#;

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "icon_list".into(),
                "on_select".into(),
                "auto_close".into(),
                "top_offset_class".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "Vec<SidebarButton>".into(),
                "Callback<AttrValue>".into(),
                "bool".into(),
                "Classes".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "List of toggle buttons and their nested items.".into(),
                "Called when a leaf item is selected.".into(),
                "If true, drawer closes after item click.".into(),
                "Optional class to offset buttons below AppBar.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            title="Sidebar Component"
            description={Some(html! {
                <p>
                    {"The `Sidebar` component is used for navigation across the TailYew documentation site. It features toggleable drawers, nested lists, and is fully mobile responsive."}
                </p>
            })}
            example={html! {
                <div class="text-sm text-gray-500 dark:text-gray-400">
                    {"You’re currently using the `Sidebar`! Open the menu on the left to explore its functionality."}
                </div>
            }}
            usage_code={usage_code}
            props_table={Some(props_table)}
        />
    }
}
