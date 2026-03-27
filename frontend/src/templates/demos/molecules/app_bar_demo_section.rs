use crate::templates::demos::DemoComponent;
use tailyew::molecules::ThemeToggle;
use tailyew::molecules::{AppBar, AppBarPosition};
use tailyew::organisms::{NestedItem, table::Column};
use web_sys::console;
use yew::prelude::*;

const USAGE_CODE: &str = include_str!("app_bar_usage.rs");
const LOGO_IMAGE_URL: &str = "/static/images/logo.png";

#[component(AppBarDemoSection)]
pub fn app_bar_demo_section() -> Html {
    let nested_list = vec![
        NestedItem::with_select("Home", "home"),
        NestedItem::with_select("Docs", "docs"),
        NestedItem::with_select("Components", "components"),
        NestedItem::with_external_link("GitHub", "github", "https://github.com/apjames93/tailyew"),
        NestedItem::with_children(
            "Account",
            vec![
                NestedItem::with_select("Login", "login"),
                NestedItem::with_select("Sign up", "signup"),
            ],
        ),
        NestedItem::with_children(
            "Settings",
            vec![NestedItem::with_content(
                html! { <ThemeToggle /> },
                "theme-toggle",
            )],
        ),
    ];

    let on_logo_click = Callback::from(|_| console::log_1(&"Logo clicked!".into()));
    let on_title_click = Callback::from(|_| console::log_1(&"Title clicked!".into()));
    let on_select = Callback::from(|value: AttrValue| {
        console::log_1(&format!("Selected menu item: {value}").into());
    });

    let example = html! {
        <div class="h-[300px] bg-gray-50 dark:bg-gray-800 relative">
            <AppBar
                title={Some(AttrValue::from("TailYew"))}
                logo_url={Some(AttrValue::from(LOGO_IMAGE_URL))}
                nested_list={nested_list}
                position={AppBarPosition::Static}
                on_select={Some(on_select)}
                logo_on_click={Some(on_logo_click)}
                title_on_click={Some(on_title_click)}
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
                "on_select".into(),
                "logo_on_click".into(),
                "title_on_click".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "Option<AttrValue>".into(),
                "Option<AttrValue>".into(),
                "Vec<NestedItem>".into(),
                "AppBarPosition".into(),
                "Option<Callback<AttrValue>>".into(),
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
                "Optional callback for selectable drawer rows.".into(),
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
                <p>{"The `AppBar` provides a responsive navigation bar with logo, centered title, and a mobile-friendly drawer. Use `on_select` for selectable rows, link row kinds for external navigation, and `with_content` for embedded widgets like the theme toggle."}</p>
            })}
            example={example}
            usage_code={USAGE_CODE}
            props_table={Some(props_table)}
        />
    }
}
