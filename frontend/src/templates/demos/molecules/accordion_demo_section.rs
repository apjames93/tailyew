use crate::templates::demos::DemoComponent;
use tailyew::atoms::{TagType, Typo};
use tailyew::molecules::Accordion;
use tailyew::organisms::table::Column;
use yew::prelude::*;

#[component(AccordionDemoSection)]
pub fn accordion_demo_section() -> Html {
    let controlled_is_open = use_state(|| false);
    let controlled_state_label = if *controlled_is_open {
        "open"
    } else {
        "closed"
    };

    let on_controlled_toggle = {
        let controlled_is_open = controlled_is_open.clone();
        Callback::from(move |next_is_open: bool| controlled_is_open.set(next_is_open))
    };

    let toggle_outside = {
        let controlled_is_open = controlled_is_open.clone();
        Callback::from(move |_| controlled_is_open.set(!*controlled_is_open))
    };

    let example = html! {
        <div class="space-y-6">
            <div class="space-y-4">
                <Typo tag={TagType::H3}>{"Uncontrolled usage"}</Typo>

                <Accordion
                    title="What is TailYew?"
                    default_open={true}
                    heading_tag={TagType::H3}
                >
                    <Typo>{"TailYew is a UI component library for Rust + Yew, styled with Tailwind CSS."}</Typo>
                </Accordion>

                <Accordion
                    title="Can I customize components?"
                    heading_tag={TagType::H3}
                    content_class={classes!("bg-yellow-50", "dark:bg-yellow-900")}
                >
                    <Typo>{"Yes! You can extend or override any component using standard Rust and Yew patterns."}</Typo>
                </Accordion>

                <Accordion
                    title="Compact mode example"
                    compact={true}
                    default_open={true}
                >
                    <Typo>{"Compact accordions remove padding and styling for use in tighter layouts (like sidebars)."}</Typo>
                </Accordion>

                <Accordion
                    title="Customized onboarding panel"
                    default_open={true}
                    class={classes!(
                        "rounded-2xl",
                        "border",
                        "border-emerald-200",
                        "bg-emerald-50",
                        "shadow-sm",
                        "dark:border-emerald-800",
                        "dark:bg-emerald-950/30"
                    )}
                    trigger_classes={classes!(
                        "bg-emerald-100",
                        "hover:bg-emerald-200",
                        "px-4",
                        "py-3",
                        "gap-4",
                        "text-emerald-950",
                        "dark:bg-emerald-900/50",
                        "dark:hover:bg-emerald-900/70",
                        "dark:text-emerald-100"
                    )}
                    content_class={classes!(
                        "border-t",
                        "border-emerald-200",
                        "bg-emerald-50",
                        "px-4",
                        "py-4",
                        "text-emerald-900",
                        "dark:border-emerald-800",
                        "dark:bg-emerald-950/40",
                        "dark:text-emerald-100"
                    )}
                >
                    <Typo>{"Consumer classes can replace the default shell, trigger, and content styles without losing Accordion behavior or accessibility."}</Typo>
                </Accordion>
            </div>

            <div class="space-y-3">
                <Typo tag={TagType::H3}>{"Controlled usage"}</Typo>
                <Typo class="text-sm text-gray-600 dark:text-gray-300">
                    { "This accordion renders from external state and reports toggle intent through `on_toggle`." }
                </Typo>

                <div class="flex flex-wrap items-center gap-3">
                    <button
                        type="button"
                        class="rounded-md border border-gray-300 bg-white px-3 py-2 text-sm font-medium text-gray-800 transition hover:bg-gray-50 dark:border-gray-700 dark:bg-gray-900 dark:text-gray-100 dark:hover:bg-gray-800"
                        onclick={toggle_outside}
                    >
                        {"Toggle from outside"}
                    </button>

                    <Typo class="text-sm text-gray-600 dark:text-gray-300">
                        {"External state: "}{controlled_state_label}
                    </Typo>
                </div>

                <Accordion
                    title="Persisted onboarding details"
                    heading_tag={TagType::H3}
                    is_open={Some(*controlled_is_open)}
                    on_toggle={Some(on_controlled_toggle)}
                >
                    <Typo>{"Use controlled mode when accordion state needs to stay synchronized with external app state, routing, or persistence."}</Typo>
                </Accordion>
            </div>
        </div>
    };

    let usage_code = r#"
<Accordion title="What is TailYew?" default_open={true} heading_tag={TagType::H3}>
    <Typo>{"TailYew is a UI component library for Rust + Yew, styled with Tailwind CSS."}</Typo>
</Accordion>

let controlled_is_open = use_state(|| false);
let on_toggle = {
    let controlled_is_open = controlled_is_open.clone();
    Callback::from(move |next_is_open: bool| controlled_is_open.set(next_is_open))
};

<Accordion
    title="Persisted onboarding details"
    is_open={Some(*controlled_is_open)}
    on_toggle={Some(on_toggle)}
>
    <Typo>{"This instance renders from external state."}</Typo>
</Accordion>

<Accordion title="Compact mode" compact={true}>
    <Typo>{"Useful in nested UIs like sidebars."}</Typo>
</Accordion>

<Accordion
    title="Customized onboarding panel"
    class={classes!("rounded-2xl", "border-emerald-200", "bg-emerald-50", "shadow-sm")}
    trigger_classes={classes!("bg-emerald-100", "hover:bg-emerald-200", "px-4", "py-3", "gap-4")}
    content_class={classes!("border-emerald-200", "bg-emerald-50", "px-4", "py-4")}
>
    <Typo>{"Use the wrapper, trigger, and content props to restyle the panel shell."}</Typo>
</Accordion>
"#;

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "children".into(),
                "class".into(),
                "trigger_classes".into(),
                "content_class".into(),
                "title".into(),
                "heading_tag".into(),
                "default_open".into(),
                "is_open".into(),
                "on_toggle".into(),
                "compact".into(),
                "arrow".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "Children".into(),
                "Classes".into(),
                "Classes".into(),
                "Classes".into(),
                "Html".into(),
                "TagType".into(),
                "bool".into(),
                "Option<bool>".into(),
                "Option<Callback<bool>>".into(),
                "bool".into(),
                "Option<Html>".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "Content shown when expanded.".into(),
                "Classes for the outer wrapper.".into(),
                "Additional Tailwind classes for the trigger element.".into(),
                "Additional Tailwind classes for content wrapper.".into(),
                "The title content rendered inside the accordion header.".into(),
                "Tag used for the heading (e.g. H2, Span).".into(),
                "Initial open state for uncontrolled usage.".into(),
                "Controlled open state. When set, the accordion renders from this value.".into(),
                "Called with the next intended open state when the accordion is toggled.".into(),
                "Enable compact styling for dense layouts.".into(),
                "Optional icon to replace the default arrow.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            github_demo_path="molecules/accordion_demo_section.rs"
            github_source_path="molecules/accordion.rs"
            title="Accordion Component"
            description={Some(html! {
                <Typo>{"The `Accordion` component toggles visibility of its children. Use `default_open` for uncontrolled usage, or pass `is_open` with `on_toggle` to keep the expanded state in sync with external app state. Wrapper, trigger, and content classes can also replace the built-in Tailwind defaults for downstream app theming."}</Typo>
            })}
            example={example}
            usage_code={usage_code}
            props_table={Some(props_table)}
        />
    }
}
