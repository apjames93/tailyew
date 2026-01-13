use tailyew::atoms::{Button, ButtonType};
use tailyew::organisms::table::Column;
use web_sys::console;
use yew::prelude::*;

use crate::templates::demos::DemoComponent;

const USAGE_CODE: &str = include_str!("./button_usage.rs");

#[component(ButtonDemoSection)]
pub fn button_demo_section() -> Html {
    let example: Html = include!("./button_usage.rs");

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "children".into(),
                "button_type".into(),
                "on_click".into(),
                "disabled".into(),
                "aria_label".into(),
                "role".into(),
                "aria_expanded".into(),
                "aria_controls".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "Children".into(),
                "ButtonType".into(),
                "Callback<MouseEvent>".into(),
                "bool".into(),
                "Option<AttrValue>".into(),
                "Option<AttrValue>".into(),
                "Option<AttrValue>".into(),
                "Option<AttrValue>".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "Content inside the button.".into(),
                "Visual style of the button.".into(),
                "Click event handler.".into(),
                "Disables the button.".into(),
                "ARIA label for accessibility.".into(),
                "ARIA role for accessibility.".into(),
                "ARIA expanded state when button controls collapsible content.".into(),
                "ID of the element controlled by this button.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            github_demo_path="atoms/button_demo_section.rs"
            github_source_path="atoms/button.rs"
            title="Button Component"
            description={Some(html! {
                <p>{"The `Button` component supports multiple styles and behaviors for your app."}</p>
            })}
            example={example}
            usage_code={USAGE_CODE}
            props_table={Some(props_table)}
        />
    }
}
