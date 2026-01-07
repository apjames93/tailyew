use crate::templates::demos::DemoComponent;
use tailyew::atoms::{Button, ButtonType};
use tailyew::molecules::ThemeToggle;
use tailyew::molecules::{AppBar, AppBarPosition};
use tailyew::organisms::{table::Column, NestedItem};
use web_sys::console;
use yew::prelude::*;

const USAGE_CODE: &str = include_str!("app_bar_usage.rs");
const LOGO_IMAGE_URL: &str = "/static/images/logo.png";

#[component(AppBarDemoSection)]
pub fn app_bar_demo_section() -> Html {
    let nested_list = vec![
        NestedItem::with_html(html! { "Home" }, "home"),
        NestedItem::with_html(html! { "Docs" }, "docs"),
        NestedItem::with_html(html! { "Components" }, "components"),
        NestedItem::with_html(html! { "GitHub" }, "github"),
        NestedItem::with_children(
            "Account",
            vec![
                NestedItem::with_html(
                    html! { <Button button_type={ButtonType::Primary}>{ "Login" }</Button> },
                    "login",
                ),
                NestedItem::with_html(
                    html! { <Button button_type={ButtonType::Secondary}>{ "Sign up" }</Button> },
                    "signup",
                ),
            ],
        ),
        NestedItem::with_children(
            "Settings",
            vec![NestedItem::with_html(
                html! { <ThemeToggle /> },
                "theme-toggle",
            )],
        ),
    ];

    let on_logo_click = Callback::from(|_| console::log_1(&"Logo clicked!".into()));
    let on_title_click = Callback::from(|_| console::log_1(&"Title clicked!".into()));

    let example = html! {
        <div class="h-[300px] bg-gray-50 dark:bg-gray-800 relative">
            <AppBar
                title={Some(AttrValue::from("TailYew"))}
                logo_url={Some(AttrValue::from(LOGO_IMAGE_URL))}
                nested_list={nested_list}
                position={AppBarPosition::Static}
                logo_onclick={Some(on_logo_click)}
                title_onclick={Some(on_title_click)}
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
                "logo_onclick".into(),
                "title_onclick".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "Option<AttrValue>".into(),
                "Option<AttrValue>".into(),
                "Vec<NestedItem>".into(),
                "AppBarPosition".into(),
                "Option<Callback<MouseEvent>>".into(),
                "Option<Callback<MouseEvent>>".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "Optional brand or site name displayed centered in the AppBar.".into(),
                "Optional logo image URL displayed left-aligned.".into(),
                "Vector of `NestedItem` used for drawer navigation.".into(),
                "Placement of the AppBar: Top (default), Bottom, or Static.".into(),
                "Optional click handler for the logo image.".into(),
                "Optional click handler for the title text.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            github_demo_path="molecules/app_bar_demo_section.rs"
            github_source_path="molecules/app_bar.rs"
            title="AppBar Component"
            description={Some(html! {
                <p>{"The `AppBar` provides a responsive navigation bar with logo, centered title, and a mobile-friendly drawer. The logo and title can be made clickable with optional callbacks."}</p>
            })}
            example={example}
            usage_code={USAGE_CODE}
            props_table={Some(props_table)}
        />
    }
}
