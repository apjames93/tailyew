// frontend/src/templates/demos/icon_demo_section.rs

use crate::templates::demos::DemoComponent;
use tailyew::atoms::{TagType, Typo};
use tailyew::icons::{
    AddIcon, AppsIcon, ArrowDownIcon, AtomIcon, BarChartIcon, CopyIcon, DeleteIcon, FormIcon,
    IconBase, PolylineIcon, SystemIcon, XIcon,
};
use tailyew::molecules::CodeBlock;
use tailyew::organisms::table::Column;
use yew::prelude::*;

const USAGE_CODE: &str = r#"use yew::prelude::*;
use tailyew::icons::{IconBase};

<IconBase class="text-sky-500" size={28}>
    <circle cx="12" cy="12" r="9" />
    <path d="M9 12l2 2 4-4" stroke-linecap="round" stroke-linejoin="round" />
</IconBase>
"#;

#[component(IconDemoSection)]
pub fn icon_demo_section() -> Html {
    let example = html! { <IconDemoExample /> };

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "class".into(),
                "size".into(),
                "stroke_width".into(),
                "label".into(),
                "decorative".into(),
                "children".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "Classes".into(),
                "u32".into(),
                "f32".into(),
                "Option<String>".into(),
                "bool".into(),
                "Children".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "Extra Tailwind or custom class names applied to the <svg>.".into(),
                "Width/height in px for the icon (defaults to 24).".into(),
                "SVG stroke width passed to children (defaults to 1.5).".into(),
                "Accessible label for screen readers.".into(),
                "If true, hides the icon from screen readers.".into(),
                "The inner <path>/<circle>/<g> describing the icon.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            github_demo_path="demos/icon_demo_section.rs"
            github_source_path="icons/icon_base.rs"
            title="IconBase & Icons"
            description={Some(html! {
                <p>
                    {"`IconBase` is the shared wrapper for all TailYew icons. "}
                    {"It normalizes props like size, class, stroke_width, and a11y, "}
                    {"and lets you pass your own SVG content."}
                </p>
            })}
            example={example}
            usage_code={USAGE_CODE}
            props_table={Some(props_table)}
        />
    }
}

// actual DOM we render in the demo pane
#[component(IconDemoExample)]
fn icon_demo_example() -> Html {
    html! {
        <div class="space-y-6">
            <div>
                <Typo tag={TagType::H3}>{"Custom icon via IconBase"}</Typo>
                <Typo tag={TagType::P}>
                    {"Pass any <path>/<circle>/<g> and keep TailYew's icon props."}
                </Typo>
                <div class="flex items-center gap-3">
                    <IconBase class="text-sky-500" size={28}>
                        <circle cx="12" cy="12" r="9" stroke-linecap="round" stroke-linejoin="round" />
                        <path d="M9 12l2 2 4-4" stroke-linecap="round" stroke-linejoin="round" />
                    </IconBase>
                    <CodeBlock show_copy={false}> {"<IconBase> ... </IconBase>"}</CodeBlock>
                </div>
            </div>

            <div class="space-y-2">
                <Typo tag={TagType::H3}>{"TailYew icons"}</Typo>
                <div class="grid grid-cols-2 sm:grid-cols-4 md:grid-cols-6 gap-4">
                    <IconTile label="Add"><AddIcon /></IconTile>
                    <IconTile label="Delete"><DeleteIcon /></IconTile>
                    <IconTile label="Close / X"><XIcon /></IconTile>
                    <IconTile label="Apps"><AppsIcon /></IconTile>
                    <IconTile label="Copy"><CopyIcon /></IconTile>
                    <IconTile label="Arrow down"><ArrowDownIcon /></IconTile>
                    <IconTile label="Bar chart"><BarChartIcon /></IconTile>
                    <IconTile label="Atom"><AtomIcon /></IconTile>
                    <IconTile label="Form"><FormIcon /></IconTile>
                    <IconTile label="Polyline"><PolylineIcon /></IconTile>
                    <IconTile label="System"><SystemIcon /></IconTile>
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct IconTileProps {
    pub label: String,
    #[prop_or_default]
    pub children: Children,
}

#[component(IconTile)]
fn icon_tile(props: &IconTileProps) -> Html {
    html! {
        <div class="flex flex-col items-center gap-2 rounded-lg border border-border/60 bg-background/40 p-3">
            <div class="text-foreground">
                { for props.children.iter() }
            </div>
            <Typo tag={TagType::Span}>
                { html! { props.label.clone() } }
            </Typo>
        </div>
    }
}
