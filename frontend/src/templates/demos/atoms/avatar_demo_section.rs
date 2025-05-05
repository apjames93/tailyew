use crate::templates::demos::DemoComponent;
use tailyew::atoms::Avatar;
use tailyew::organisms::table::Column;
use yew::prelude::*;

#[function_component(AvatarDemoSection)]
pub fn avatar_demo_section() -> Html {
    let log_click = Callback::from(|_| web_sys::console::log_1(&"Avatar clicked!".into()));

    let example = html! {
        <div class="flex items-center gap-6">
            <Avatar
                src={Some(String::from("/images/TailYew.png"))}
                alt={Some(String::from("TailYew Logo"))}
            />
            <Avatar
                fallback={Some(String::from("AJ"))}
                on_click={log_click.clone()}
            />
            <Avatar
                fallback={Some(String::from("🚀"))}
                class="bg-purple-600"
                on_click={log_click}
            />
        </div>
    };

    let usage_code = r#"
let log_click = Callback::from(|_| web_sys::console::log_1(&"Avatar clicked!".into()));

<div class="flex items-center gap-6">
  <Avatar
    src={Some(String::from("/images/TailYew.png"))}
    alt={Some(String::from("TailYew Logo"))}
  />
  <Avatar
    fallback={Some(String::from("AJ"))}
    on_click={log_click.clone()}
  />
  <Avatar
    fallback={Some(String::from("🚀"))}
    class="bg-purple-600"
    on_click={log_click}
  />
</div>
"#;

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "src".into(),
                "fallback".into(),
                "alt".into(),
                "size".into(),
                "class".into(),
                "on_click".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "Option<String>".into(),
                "Option<String>".into(),
                "Option<String>".into(),
                "Classes".into(),
                "Classes".into(),
                "Option<Callback<MouseEvent>>".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "Image source URL.".into(),
                "Text fallback when no image is provided.".into(),
                "Alt text for accessibility.".into(),
                "Tailwind width/height (default: w-12 h-12).".into(),
                "Additional Tailwind classes.".into(),
                "Fires when the avatar is clicked.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            title="Avatar Component"
            description={Some(html! {
                <p>{ "The `Avatar` component renders a circular user image or fallback content with full Tailwind and dark mode support. It can also accept an optional `on_click` callback." }</p>
            })}
            example={example}
            usage_code={usage_code}
            props_table={Some(props_table)}
        />
    }
}
