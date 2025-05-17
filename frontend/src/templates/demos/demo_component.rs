// tailyew/src/templates/demo_component.rs

use tailyew::{
    atoms::{CodeBlock, Section, TagType, Typo},
    molecules::CopyToClipboard,
    organisms::{Column, Table},
    ButtonType,
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

            <Typo tag={TagType::P} class="mb-4">{ props.description.clone().unwrap_or_else(|| html! {}) }</Typo>

            // Example Render
            <div class="my-6 p-4 rounded bg-gray-50 dark:bg-gray-800 text-center">
                { props.example.clone() }
            </div>

            // Usage Code Block
            <div class="mt-8 flex items-center justify-between">
                <Typo tag={TagType::H3}>{"Usage Example"}</Typo>
                <CopyToClipboard
                    value={props.usage_code.clone()}
                    copy_text={"Copy Example"}
                    copied_text={"Copied!"}
                    button_type={ButtonType::Ghost}
                    copied_button_type={ButtonType::Primary}
                    class="text-sm"
                />
            </div>
            <CodeBlock>
                { props.usage_code.clone() }
            </CodeBlock>


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
