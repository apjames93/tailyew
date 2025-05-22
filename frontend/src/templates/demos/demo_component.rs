use tailyew::{
    atoms::{Section, TagType, Typo, A},
    molecules::CodeBlock,
    organisms::{Column, Table},
};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct DemoComponentProps {
    pub title: AttrValue,
    #[prop_or_default]
    pub description: Option<Html>,
    pub example: Html,
    pub usage_code: AttrValue,
    #[prop_or_default]
    pub props_table: Option<Vec<Column>>,
    #[prop_or_default]
    pub children: Children,
    pub github_demo_path: AttrValue,
    pub github_source_path: AttrValue,
}

#[function_component(DemoComponent)]
pub fn demo_component(props: &DemoComponentProps) -> Html {
    let DemoComponentProps {
        title,
        description,
        example,
        usage_code,
        props_table,
        children,
        github_demo_path,
        github_source_path,
    } = props;

    let github_links = html! {
        <div class="flex gap-4 text-sm items-center">
            <A
                href={format!(
                    "https://github.com/apjames93/tailyew/tree/main/frontend/src/templates/demos/{}",
                    github_demo_path.trim_start_matches('/')
                )}
                target={Some("_blank".to_string())}
            >
                { "Usage ↗" }
            </A>

            <A
                href={format!(
                    "https://github.com/apjames93/tailyew/blob/main/crates/tailyew/src/{}",
                    github_source_path.trim_start_matches('/')
                )}
                target={Some("_blank".to_string())}
            >
                { "Source ↗" }
            </A>
        </div>
    };

    html! {
        <Section class="pt-[74px] mb-16">
            <div class="flex items-center justify-between mb-4">
                <Typo tag={TagType::H2}>{ title.clone() }</Typo>
                { github_links }
            </div>

            { description.as_ref().map(|d| html! {
                <Typo tag={TagType::P} class="mb-4">{ d.clone() }</Typo>
            }) }

            <div class="my-6 p-4 rounded bg-gray-50 dark:bg-gray-800 text-center">
                { example.clone() }
            </div>

            <CodeBlock>
                { usage_code.clone() }
            </CodeBlock>

            { props_table.as_ref().map(|columns| html! {
                <>
                    <Typo tag={TagType::H4} class="mt-8">{ "Props" }</Typo>
                    <Table columns={columns.clone()} />
                </>
            }) }

            { for children.iter() }
        </Section>
    }
}
