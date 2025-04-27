use crate::templates::demos::DemoComponent;
use tailyew::molecules::{AppBar, AppBarPosition};
use tailyew::organisms::{table::Column, NestedItem};
use yew::prelude::*;

const USAGE_CODE: &str = include_str!("app_bar_usage.rs");
const LOGO_IMAGE_URL: &str = "/images/logo.png";

#[function_component(AppBarDemoSection)]
pub fn app_bar_demo_section() -> Html {
    let nested_list = vec![
        NestedItem::with_html(html! { "Home" }, "home"),
        NestedItem::with_html(html! { "Docs" }, "docs"),
        NestedItem::with_html(html! { "Components" }, "components"),
        NestedItem::with_html(html! { "GitHub" }, "github"),
        NestedItem::with_children(
            "Account",
            vec![
                NestedItem::with_html(html! { "Login" }, "login"),
                NestedItem::with_html(html! { "Sign up" }, "signup"),
            ],
        ),
    ];

    let example = html! {
        <div class="h-[300px] bg-gray-50 dark:bg-gray-800 relative">
            <AppBar
                title={Some(AttrValue::from("TailYew"))}
                logo_url={Some(AttrValue::from(LOGO_IMAGE_URL))}
                nested_list={nested_list}
                position={AppBarPosition::Static}
            />
        </div>
    };

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "title".into(),
                "logo_url".into(),
                "nested_list".into(),
                "position".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "Option<AttrValue>".into(),
                "Option<AttrValue>".into(),
                "Vec<NestedItem>".into(),
                "AppBarPosition".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "Optional brand or site name displayed next to the logo.".into(),
                "Optional logo image URL.".into(),
                "Vector of `NestedItem` used for drawer navigation.".into(),
                "Placement of the AppBar: Top, Bottom, or Static.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            title="AppBar Component"
            description={Some(html! {
                <p>{"The `AppBar` provides a responsive navigation bar with logo, centered title, and a mobile-friendly drawer for navigation."}</p>
            })}
            example={example}
            usage_code={USAGE_CODE}
            props_table={Some(props_table)}
        />
    }
}
