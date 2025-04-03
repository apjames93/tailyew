use crate::templates::demos::DemoComponent;
use tailyew::atoms::{Button, ButtonType};
use tailyew::molecules::{AppBar, AppBarPosition};
use tailyew::organisms::table::Column;
use yew::prelude::*;

const USAGE_CODE: &str = include_str!("app_bar_usage.rs");
const LOGO_IMAGE_URL: &str = "/images/logo.png";

#[function_component(AppBarDemoSection)]
pub fn app_bar_demo_section() -> Html {
    let example = html! {
        <div class="h-[200px] bg-gray-50 dark:bg-gray-800">
            <AppBar
                title={Some(AttrValue::from("TailYew"))}
                logo_url={Some(AttrValue::from(LOGO_IMAGE_URL))}
                position={AppBarPosition::Static}
                links={vec![
                    html! { <a href="#" class="text-sm font-medium hover:underline">{"Docs"}</a> },
                    html! { <a href="#" class="text-sm font-medium hover:underline">{"Components"}</a> },
                    html! { <a href="#" class="text-sm font-medium hover:underline">{"GitHub"}</a> },
                ]}
                actions={vec![
                    html! { <Button button_type={ButtonType::Secondary}>{"Login"}</Button> },
                    html! { <Button button_type={ButtonType::Primary}>{"Sign up"}</Button> },
                ]}
            />
        </div>
    };

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "title".into(),
                "logo_url".into(),
                "links".into(),
                "actions".into(),
                "position".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "Option<AttrValue>".into(),
                "Option<AttrValue>".into(),
                "Vec<Html>".into(),
                "Vec<Html>".into(),
                "AppBarPosition".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "Optional brand or site name displayed next to the logo.".into(),
                "Optional logo image URL.".into(),
                "Vector of navigation link elements.".into(),
                "Vector of action buttons or controls.".into(),
                "Placement of the bar: Top, Bottom, or Static.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            title="AppBar Component"
            description={Some(html! {
                <p>{"The `AppBar` provides a responsive top or bottom navigation bar with logo, links, and actions. Adjust for mobile with automatic menu toggle."}</p>
            })}
            example={example}
            usage_code={USAGE_CODE}
            props_table={Some(props_table)}
        />
    }
}
