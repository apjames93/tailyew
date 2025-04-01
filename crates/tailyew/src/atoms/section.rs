use crate::atoms::Container;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct SectionProps {
    #[prop_or("py-12".to_string())]
    pub padding: String,
    #[prop_or_default]
    pub background_color: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub container_class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    pub children: Children,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub style: Option<String>,
}

#[function_component(Section)]
pub fn section(props: &SectionProps) -> Html {
    let SectionProps {
        padding,
        background_color,
        class,
        container_class,
        id,
        children,
        onclick,
        style,
    } = props.clone();

    let base_classes = classes!(
        padding,
        background_color.unwrap_or_default(),
        class.unwrap_or_default(),
    );

    html! {
        <section
            id={id}
            class={base_classes}
            onclick={onclick}
            style={style}
        >
            <Container class={container_class}>
                { for children.iter() }
            </Container>
        </section>
    }
}
