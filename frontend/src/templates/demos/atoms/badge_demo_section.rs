use crate::templates::demos::DemoComponent;
use tailyew::atoms::{Avatar, Badge, Button, Typo};
use tailyew::organisms::table::Column;
use yew::prelude::*;

#[function_component(BadgeDemoSection)]
pub fn badge_demo_section() -> Html {
    let example = html! {
        <div class="flex flex-wrap gap-8 text-gray-800 dark:text-white">
            // Count badge
            <Badge badge_content={Some(String::from("4"))} color={classes!("bg-blue-600")}>
                <Avatar src={Some(String::from("/images/TailYew.png"))} alt={Some(String::from("TailYew Logo"))} />
            </Badge>

            // Dot badge
            <Badge is_dot=true color={classes!("bg-emerald-500")}>
                <Typo class="px-4 py-2 border rounded">{"Notifications"}</Typo>
            </Badge>

            // Max limit badge (e.g. 1000 → 99+)
            <Badge badge_content={Some(String::from("1000"))} max={99} color={classes!("bg-green-600")}>
                <Button>{"Inbox"}</Button>
            </Badge>

            // Text badge
            <Badge badge_content={Some(String::from("New"))} color={classes!("bg-purple-500")}>
                <Button>{"Inbox"}</Button>
            </Badge>

            // Show zero explicitly
            <Badge badge_content={Some(String::from("0"))} show_zero=true color={classes!("bg-yellow-500")}>
                <Button>{"Inbox"}</Button>
            </Badge>
        </div>
    };

    let usage_code = r#"
<div class="flex items-center gap-8">
  <Badge badge_content={Some(String::from("4"))} color={classes!("bg-blue-600")}>
    <Avatar src={Some(String::from("/images/TailYew.png"))} alt={Some(String::from("TailYew Logo"))} />
  </Badge>

  <Badge badge_content={Some(String::from("1000"))} max={99} color={classes!("bg-green-600")}>
    <Typo>{"TailYew"}</Typo>
  </Badge>

  <Badge is_dot=true color={classes!("bg-red-500")}>
    <span class="inline-block w-8 h-8 bg-gray-200 dark:bg-gray-700 rounded-full" />
  </Badge>

  <Badge badge_content={Some(String::from("0"))} show_zero=true color={classes!("bg-yellow-500")}>
    <span class="inline-block w-8 h-8 bg-gray-200 dark:bg-gray-700 rounded-full" />
  </Badge>
</div>
"#;

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "badge_content".into(),
                "show_zero".into(),
                "is_dot".into(),
                "max".into(),
                "color".into(),
                "position".into(),
                "class".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "Option<String>".into(),
                "bool".into(),
                "bool".into(),
                "usize".into(),
                "Classes".into(),
                "Classes".into(),
                "Classes".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "The text or number inside the badge.".into(),
                "If true, show the badge when the content is zero.".into(),
                "If true, show a dot instead of content.".into(),
                "Maximum value before showing as +max.".into(),
                "Tailwind classes for badge color.".into(),
                "Tailwind classes to position the badge.".into(),
                "Extra Tailwind classes.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            github_demo_path="atoms/badge_demo_section.rs"
            github_source_path="atoms/badge.rs"
            title="Badge Component"
            description={Some(html! {
                <p>{ "The `Badge` component displays content or a dot over its child, supporting max counts, position, dot variant, and color control." }</p>
            })}
            example={example}
            usage_code={usage_code}
            props_table={Some(props_table)}
        />
    }
}
