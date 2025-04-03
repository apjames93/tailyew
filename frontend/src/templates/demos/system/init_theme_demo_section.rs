use crate::templates::demos::DemoComponent;
use tailyew::atoms::{Button, ButtonType, Section, TagType, Typo};
use tailyew::organisms::table::Column;
use tailyew::system::{InitTheme, Theme};
use yew::prelude::*;

#[function_component(InitThemeDemoSection)]
pub fn init_theme_demo_section() -> Html {
    let example = html! {
        <InitTheme
            theme={Some(Theme {
                name: "dark".into(),
                class: Some("p-4 rounded-lg border border-gray-700 bg-gray-900 text-white".into()),
            })}
        >
            <Section>
                <Typo tag={TagType::H3}>{ "This content is wrapped in a dark theme." }</Typo>
                <p class="mt-2 text-sm text-gray-400">
                    { "You can customize the background, typography, or layout by passing a custom class and theme name." }
                </p>
                <div class="mt-4">
                    <Button button_type={ButtonType::Primary}>{ "Primary Button" }</Button>
                </div>
            </Section>
        </InitTheme>
    };

    let usage_code = r#"
<InitTheme
    theme={Some(Theme {
        name: "dark".into(),
        class: Some("p-4 rounded-lg border border-gray-700 bg-gray-900 text-white".into()),
    })}
>
    <Section>
        <Typo tag={TagType::H3}>{ "This content is wrapped in a dark theme." }</Typo>
        <p>{ "Custom styles and Tailwind utilities work here." }</p>
    </Section>
</InitTheme>
"#;

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec!["theme".into(), "class".into()],
        },
        Column {
            header: "Type".into(),
            values: vec!["Option<Theme>".into(), "Classes".into()],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "Optional theme config. Accepts a `name` (light, dark, system) and optional `class`.".into(),
                "Extra utility classes for layout/styling the root container.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            title="InitTheme Component"
            description={Some(html! {
                <p>{ "Use the `InitTheme` component to wrap your Yew application and apply consistent theming and layout. Supports Tailwind utility class overrides and system-based light/dark themes." }</p>
            })}
            example={example}
            usage_code={usage_code}
            props_table={Some(props_table)}
        />
    }
}
