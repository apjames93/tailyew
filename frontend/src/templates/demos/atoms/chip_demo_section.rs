use crate::templates::demos::DemoComponent;
use tailyew::atoms::{Chip, ChipSize, ChipVariant};
use tailyew::organisms::table::Column;
use yew::prelude::*;

const USAGE_CODE: &str = include_str!("./chip_usage.rs");

#[component(ChipDemoSection)]
pub fn chip_demo_section() -> Html {
    let example = html! {
        <div class="space-y-6">
            <div class="flex flex-wrap items-center gap-3">
                <Chip>{ "Neutral" }</Chip>
                <Chip variant={ChipVariant::Primary}>{ "Primary" }</Chip>
                <Chip variant={ChipVariant::Success}>{ "Success" }</Chip>
                <Chip variant={ChipVariant::Warning}>{ "Warning" }</Chip>
                <Chip variant={ChipVariant::Danger}>{ "Danger" }</Chip>
            </div>

            <div class="flex flex-wrap items-center gap-3">
                <Chip size={ChipSize::Small}>{ "Small" }</Chip>
                <Chip>{ "Medium" }</Chip>
                <Chip
                    removable={true}
                    remove_aria_label="Remove beta"
                    remove_title="Remove tag"
                >
                    { "beta" }
                </Chip>
                <Chip
                    variant={ChipVariant::Primary}
                    removable={true}
                    disabled={true}
                    remove_aria_label="Remove locked"
                >
                    { "locked" }
                </Chip>
            </div>
        </div>
    };

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "children".into(),
                "variant".into(),
                "size".into(),
                "removable".into(),
                "disabled".into(),
                "on_remove".into(),
                "remove_aria_label".into(),
                "remove_title".into(),
                "class".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "Children".into(),
                "ChipVariant".into(),
                "ChipSize".into(),
                "bool".into(),
                "bool".into(),
                "Option<Callback<MouseEvent>>".into(),
                "Option<AttrValue>".into(),
                "AttrValue".into(),
                "Classes".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "Content inside the chip.".into(),
                "Visual style: Neutral, Primary, Success, Warning, or Danger.".into(),
                "Chip density: Small or Medium.".into(),
                "Shows a compact remove button when true.".into(),
                "Dims the chip and disables removal.".into(),
                "Remove button click handler.".into(),
                "Accessible label for the remove button.".into(),
                "Visible browser title for the remove button.".into(),
                "Additional Tailwind classes.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            github_demo_path="atoms/chip_demo_section.rs"
            github_source_path="atoms/chip.rs"
            title="Chip Component"
            description={Some(html! {
                <p>{ "The `Chip` component renders compact labels, filters, selected values, and removable tokens." }</p>
            })}
            example={example}
            usage_code={USAGE_CODE}
            props_table={Some(props_table)}
        />
    }
}
