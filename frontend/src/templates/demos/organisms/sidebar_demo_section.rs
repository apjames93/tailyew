use crate::templates::demos::DemoComponent;
use tailyew::atoms::Typo;
use tailyew::organisms::{table::Column, NestedItem, Sidebar, SidebarButton, SidebarPosition};
use yew::prelude::*;

#[component(SidebarDemoSection)]
pub fn sidebar_demo_section() -> Html {
    let icons = [
        html! {
            <svg class="h-6 w-6" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" d="M4 6h16M4 12h16M4 18h16"/>
            </svg>
        },
        html! {
            <svg class="h-6 w-6" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" d="M3 12l2-2m0 0l7-7 7 7m-9 2v8"/>
            </svg>
        },
        html! {
            <svg class="h-6 w-6" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" d="M11 11V9a4 4 0 1 1 8 0v2M6 19h12a2 2 0 0 0 2-2v-6a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2z"/>
            </svg>
        },
        html! {
            <svg class="h-6 w-6" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" d="M12 4v1m0 14v1m8-9h1M4 12H3m15.364 5.364l.707.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707"/>
            </svg>
        },
        html! {
            <svg class="h-6 w-6" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" d="M12 14l9-5-9-5-9 5 9 5z"/>
                <path stroke-linecap="round" stroke-linejoin="round" d="M12 14l6.16-3.422A12.042 12.042 0 0 1 21 15.338C21 18.278 16.97 21 12 21s-9-2.722-9-5.662c0-1.095.423-2.157 1.184-3.038L12 14z"/>
            </svg>
        },
    ];

    let usage_code = r#"
<Sidebar
    icon_list={vec![
        SidebarButton {
            icon: html! {
                <svg class="h-6 w-6" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M4 6h16M4 12h16M4 18h16" />
                </svg>
            },
            open_text: html! {"Menu"},
            list: vec![NestedItem::new("Example")],
        }
    ]}
    position={SidebarPosition::Left}
    top_offset_class={classes!("top-20")}
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
                "position".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "Vec<SidebarButton>".into(),
                "Callback<AttrValue>".into(),
                "bool".into(),
                "Classes".into(),
                "SidebarPosition".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "List of toggle buttons and their nested items.".into(),
                "Called when a leaf item is selected.".into(),
                "If true, drawer closes after item click.".into(),
                "Optional class to offset below AppBar.".into(),
                "Where to render the Sidebar (Left, Right, or Static).".into(),
            ],
        },
    ];

    let nested_list = vec![
        NestedItem::with_children(
            "Docs",
            vec![
                NestedItem::with_children(
                    "Nested",
                    vec![
                        NestedItem::with_html(html! { "Nested 1" }, "nested-1"),
                        NestedItem::with_html(html! { "Nested 2" }, "nested-2"),
                    ],
                ),
                NestedItem::with_html(html! { "Getting Started" }, "getting-started"),
            ],
        ),
        NestedItem::with_children(
            "Components",
            vec![
                NestedItem::with_html(html! { "Button" }, "button"),
                NestedItem::with_html(html! { "Card" }, "card"),
            ],
        ),
    ];

    let on_select = Callback::from(|val: AttrValue| {
        web_sys::console::log_1(&format!("Selected item: {val}").into());
    });

    html! {
        <DemoComponent
            github_demo_path="organisms/sidebar_demo_section.rs"
            github_source_path="organisms/sidebar.rs"
            title="Sidebar Component"
            description={Some(html! {
                <p>
                    {"The `Sidebar` component supports `left`, `right`, and `static` positioning. Try interacting with the menus below to see how it behaves in each mode."}
                </p>
            })}
            example={html! {
                <>
                    <Typo class="mt-4 text-sm text-gray-500 dark:text-gray-400">
                        {"The Demo site uses the Left-positioned sidebar."}
                    </Typo>

                    // Right-positioned sidebar
                    <Sidebar
                    icon_list={vec![
                        SidebarButton {
                            icon: icons[0].clone(),
                            open_text: html! { "Static Menu 1" },
                            list: nested_list.clone(),
                        },
                        SidebarButton {
                            icon: icons[1].clone(),
                            open_text: html! { "Static Menu 2" },
                            list: nested_list.clone(),
                        },
                        SidebarButton {
                            icon: icons[2].clone(),
                            open_text: html! { "Static Menu 3" },
                            list: nested_list.clone(),
                        },
                        SidebarButton {
                            icon: icons[3].clone(),
                            open_text: html! { "Static Menu 4" },
                            list: nested_list.clone(),
                        },
                        SidebarButton {
                            icon: icons[4].clone(),
                            open_text: html! { "Static Menu 5" },
                            list: nested_list.clone(),
                        },
                    ]}
                        position={SidebarPosition::Right}
                        top_offset_class={classes!("top-16")}
                        on_select={on_select.clone()}
                    />

                    // Static sidebar (always expanded)
                    <div class="mt-20 ml-72 mr-72 p-4 border rounded h-[50vh]">
                        <Sidebar
                            icon_list={vec![
                                SidebarButton {
                                    icon: icons[0].clone(),
                                    open_text: html! { "Static Menu 1" },
                                    list: nested_list.clone(),
                                },
                                SidebarButton {
                                    icon: icons[1].clone(),
                                    open_text: html! { "Static Menu 2" },
                                    list: nested_list.clone(),
                                },
                                SidebarButton {
                                    icon: icons[2].clone(),
                                    open_text: html! { "Static Menu 3" },
                                    list: nested_list.clone(),
                                },
                                SidebarButton {
                                    icon: icons[3].clone(),
                                    open_text: html! { "Static Menu 4" },
                                    list: nested_list.clone(),
                                },
                                SidebarButton {
                                    icon: icons[4].clone(),
                                    open_text: html! { "Static Menu 5" },
                                    list: nested_list.clone(),
                                },
                            ]}
                            position={SidebarPosition::Static}
                            on_select={on_select}
                        />

                    </div>
                </>
            }}
            usage_code={usage_code}
            props_table={Some(props_table)}
        />
    }
}
