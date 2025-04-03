// tailyew/src/templates/demo_component.rs

use tailyew::{
    atoms::{Section, TagType, Typo},
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
}

#[function_component(DemoComponent)]
pub fn demo_component(props: &DemoComponentProps) -> Html {
    html! {
        <Section class="mb-16">
            <Typo tag={TagType::H2} class="mb-4">{ props.title.clone() }</Typo>

            { props.description.clone().unwrap_or_else(|| html! {}) }

            // Example Render
            <div class="my-6 p-4 rounded bg-gray-50 dark:bg-gray-800 text-center">
                { props.example.clone() }
            </div>

            // Usage Code Block
            <Typo tag={TagType::H3} class="mt-8">{"Usage Example"}</Typo>
            <pre class="bg-gray-200 dark:bg-gray-900 p-4 rounded overflow-auto text-sm">
                <code>{ props.usage_code.clone() }</code>
            </pre>

            // Props Table
            {
                if let Some(columns) = props.props_table.clone() {
                    html! {
                        <>
                            <Typo tag={TagType::H4} class="mt-8">{"Props"}</Typo>
                            <Table columns={columns} />
                        </>
                    }
                } else {
                    html! {}
                }
            }

            { for props.children.iter() }
        </Section>
    }
}
