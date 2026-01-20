// frontend/src/templates/demos/avatar_group_demo_section.rs

use crate::templates::demos::DemoComponent;
use tailyew::molecules::{AvatarData, AvatarGroup};
use tailyew::organisms::table::Column;
use yew::prelude::*;

#[component(AvatarGroupDemoSection)]
pub fn avatar_group_demo_section() -> Html {
    let avatars = vec![
        AvatarData {
            src: Some("/static/images/TailYew.png".into()),
            fallback: Some("AJ".into()),
            alt: Some("Alex".into()),
            class: classes!(),
        },
        AvatarData {
            src: None,
            fallback: Some("B".into()),
            alt: Some("Bri".into()),
            class: classes!(),
        },
        AvatarData {
            src: Some("/static/images/TailYew.png".into()),
            fallback: None,
            alt: Some("C".into()),
            class: classes!(),
        },
        AvatarData {
            src: None,
            fallback: Some("D".into()),
            alt: Some("Dana".into()),
            class: classes!(),
        },
        AvatarData {
            src: None,
            fallback: Some("E".into()),
            alt: Some("Eli".into()),
            class: classes!(),
        },
        AvatarData {
            src: None,
            fallback: Some("F".into()),
            alt: Some("Fay".into()),
            class: classes!(),
        },
    ];

    let example = html! {
        <AvatarGroup avatars={avatars.clone()} max_visible={4} size={classes!("w-10", "h-10")} />
    };

    let usage_code = r#"
let avatars = vec![
    AvatarData {
        src: Some("/static/images/user1.png".into()),
        fallback: Some("AJ".into()),
        alt: Some("Alex".into()),
        class: classes!(),
    },
    AvatarData {
        src: None,
        fallback: Some("B".into()),
        alt: Some("Bri".into()),
        class: classes!(),
    },
    AvatarData {
        src: Some("/static/images/user3.png".into()),
        fallback: None,
        alt: Some("C".into()),
        class: classes!(),
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
