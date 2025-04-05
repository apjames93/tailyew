use tailyew::atoms::{Button, ButtonType, TagType, Typo};
use tailyew::molecules::navbar::AppBarPosition;
use tailyew::molecules::NavBar;
use tailyew::organisms::table::Column;
use yew::prelude::*;

use crate::templates::demos::DemoComponent;

#[function_component(NavBarDemoSection)]
pub fn navbar_demo_section() -> Html {
    let example = html! {
        <NavBar position={AppBarPosition::Static}>
            <Typo tag={TagType::H3}>{ "My App" }</Typo>
            <div class="space-x-4">
                <Button button_type={ButtonType::Secondary}>{ "Login" }</Button>
                <Button button_type={ButtonType::Primary}>{ "Sign Up" }</Button>
            </div>
        </NavBar>
    };

    let usage_code = r#"
use tailyew::molecules::NavBar;
use tailyew::molecules::navbar::AppBarPosition;
use tailyew::atoms::{Typo, TagType, Button, ButtonType};

<NavBar position={AppBarPosition::Static}>
    <Typo tag={TagType::H3}>{ "My App" }</Typo>
    <div class="space-x-4">
        <Button button_type={ButtonType::Secondary}>{ "Login" }</Button>
        <Button button_type={ButtonType::Primary}>{ "Sign Up" }</Button>
    </div>
</NavBar>
"#;

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "children".into(),
                "class".into(),
                "id".into(),
                "position".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "Children".into(),
                "Classes".into(),
                "Option<String>".into(),
                "AppBarPosition".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "Elements rendered inside the NavBar.".into(),
                "Tailwind utility classes to customize layout and style.".into(),
                "Optional HTML `id` for the `<nav>` element.".into(),
                r#"Positioning behavior: "Static" (default), "Top", or "Bottom"."#.into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            title="NavBar Component"
            description={Some(html! {
                <p>{ "The `NavBar` component provides a styled, responsive container for navigation content like logos, links, and action buttons. Position it at the top, bottom, or inline in your layout." }</p>
            })}
            example={example}
            usage_code={usage_code}
            props_table={Some(props_table)}
        />
    }
}
