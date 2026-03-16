use crate::templates::demos::DemoComponent;
use tailyew::atoms::{A, Button, Typo};
use tailyew::molecules::Breadcrumbs;
use tailyew::organisms::table::Column;
use yew::prelude::*;

#[component(BreadcrumbsDemoSection)]
pub fn breadcrumbs_demo_section() -> Html {
    let example = html! {
        <div class="space-y-6 text-sm text-gray-800 dark:text-white">
            <Breadcrumbs separator={html! { "›" }}>
                <A href="/">{ "Home" }</A>
                <A href="/dashboard">{ "Dashboard" }</A>
                <Typo>{"Demo"}</Typo>
            </Breadcrumbs>

            <Breadcrumbs aria_label="breadcrumb-2" separator={html! { "/" }}>
                <A href="/">{ "Home" }</A>
                <A href="/dashboard">{ "Dashboard" }</A>
                <A href="/settings">{ "Settings" }</A>
            </Breadcrumbs>

            <Breadcrumbs aria_label="breadcrumb-3" separator={html! { "-" }}>
                <Button>{ "Home" }</Button>
                <Button>{ "Dashboard" }</Button>
                <Button>{ "Settings" }</Button>
            </Breadcrumbs>
        </div>
    };

    let usage_code = r#"
<Breadcrumbs separator={html! { "›" }}>
    <A href="/">{ "Home" }</A>
    <A href="/dashboard">{ "Dashboard" }</A>
    <A href="/settings">{ "Settings" }</A>
</Breadcrumbs>
"#;

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "children".into(),
                "separator".into(),
                "class".into(),
                "aria_label".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "Children".into(),
                "Html".into(),
                "Classes".into(),
                "String".into(),
                "String".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "Each breadcrumb segment as a node (e.g., A, Button, Typo).".into(),
                "Custom separator between items. Defaults to '/'.".into(),
                "Optional wrapper class for styling the <nav> container.".into(),
                "Optional aria-label for accessibility.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            github_demo_path="molecules/breadcrumbs_demo_section.rs"
            github_source_path="molecules/breadcrumbs.rs"
            title="Breadcrumbs Component"
            description={Some(html! {
                <p>{ "The `Breadcrumbs` component displays a navigation trail using `A`, `Typo`, or `Button`. It supports custom separators and styled segments." }</p>
            })}
            example={example}
            usage_code={usage_code}
            props_table={Some(props_table)}
        />
    }
}
