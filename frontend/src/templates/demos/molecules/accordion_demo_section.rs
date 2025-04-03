use crate::templates::demos::DemoComponent;
use tailyew::molecules::Accordion;
use tailyew::organisms::table::Column;
use yew::prelude::*;

#[function_component(AccordionDemoSection)]
pub fn accordion_demo_section() -> Html {
    let example = html! {
        <div class="space-y-4">
            <Accordion title="What is TailYew?" default_open={true}>
                <p>{"TailYew is a UI component library for Rust + Yew, styled with Tailwind CSS."}</p>
            </Accordion>
            <Accordion title="Can I customize components?" default_open={false}>
                <p>{"Yes! You can extend or override any component using standard Rust and Yew patterns."}</p>
            </Accordion>
            <Accordion title="Is it production-ready?">
                <p>{"Absolutely — we're using it in real-world apps with SSR and WASM support."}</p>
            </Accordion>
        </div>
    };

    let usage_code = r#"
<Accordion title="What is TailYew?" default_open={true}>
    <p>{"TailYew is a UI component library for Rust + Yew, styled with Tailwind CSS."}</p>
</Accordion>
"#;

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "title".into(),
                "children".into(),
                "class".into(),
                "heading_tag".into(),
                "default_open".into(),
                "arrow".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "String".into(),
                "Children".into(),
                "Classes".into(),
                "TagType".into(),
                "bool".into(),
                "Option<Html>".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "The title of the accordion header.".into(),
                "Content shown when expanded.".into(),
                "Optional custom classes for the outer wrapper.".into(),
                "The HTML tag used for the title (e.g., H2, H3, Span).".into(),
                "Sets the initial open state.".into(),
                "Optional custom arrow icon.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            title="Accordion Component"
            description={Some(html! {
                <p>{"The `Accordion` toggles visibility of content. Fully accessible with keyboard support, and customizable with a `heading_tag` or arrow icon."}</p>
            })}
            example={example}
            usage_code={usage_code}
            props_table={Some(props_table)}
        />
    }
}
