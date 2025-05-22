// frontend/src/templates/demos/avatar_group_demo_section.rs

use crate::templates::demos::DemoComponent;
use tailyew::molecules::{AvatarData, AvatarGroup};
use tailyew::organisms::table::Column;
use yew::prelude::*;

#[function_component(AvatarGroupDemoSection)]
pub fn avatar_group_demo_section() -> Html {
    let avatars = vec![
        AvatarData {
            src: Some(String::from("/images/TailYew.png")),
            fallback: Some(String::from("AJ")),
            alt: Some(String::from("Alex")),
            class: None,
        },
        AvatarData {
            src: None,
            fallback: Some(String::from("B")),
            alt: Some(String::from("Bri")),
            class: None,
        },
        AvatarData {
            src: Some(String::from("/images/TailYew.png")),
            fallback: None,
            alt: Some(String::from("C")),
            class: None,
        },
        AvatarData {
            src: None,
            fallback: Some(String::from("D")),
            alt: Some(String::from("Dana")),
            class: None,
        },
        AvatarData {
            src: None,
            fallback: Some(String::from("E")),
            alt: Some(String::from("Eli")),
            class: None,
        },
        AvatarData {
            src: None,
            fallback: Some(String::from("F")),
            alt: Some(String::from("Fay")),
            class: None,
        },
    ];

    let example = html! {
        <AvatarGroup avatars={avatars.clone()} max_visible={4} size={classes!("w-10", "h-10")} />
    };

    let usage_code = r#"
let avatars = vec![
    AvatarData {
        src: Some(String::from("/images/user1.png")),
        fallback: Some(String::from("AJ")),
        alt: Some(String::from("Alex")),
        class: None,
    },
    AvatarData {
        src: None,
        fallback: Some(String::from("B")),
        alt: Some(String::from("Bri")),
        class: None,
    },
    AvatarData {
        src: Some(String::from("/images/user3.png")),
        fallback: None,
        alt: Some(String::from("C")),
        class: None,
    },
    // ...
];

html! {
    <AvatarGroup avatars={avatars} max_visible={4} size={classes!("w-10", "h-10")} />
};
"#;

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "avatars".into(),
                "max_visible".into(),
                "size".into(),
                "reverse".into(),
                "class".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "Vec<AvatarData>".into(),
                "usize".into(),
                "Classes".into(),
                "bool".into(),
                "Classes".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "List of avatar props.".into(),
                "Number of avatars to show before +N overflow.".into(),
                "Shared Tailwind width/height for each avatar.".into(),
                "Reverse the visual order (RTL-style).".into(),
                "Additional Tailwind classes for the wrapper.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            github_demo_path="molecules/avatar_group_demo_section.rs"
            github_source_path="molecules/avatar_group.rs"
            title="AvatarGroup Component"
            description={Some(html! {
                <p>{ "The `AvatarGroup` stacks multiple avatars with overlapping styles. Supports fallback text, image avatars, and overflow counts like +2." }</p>
            })}
            example={example}
            usage_code={usage_code}
            props_table={Some(props_table)}
        />
    }
}
