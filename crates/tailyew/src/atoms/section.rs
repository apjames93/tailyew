use crate::atoms::Container;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct SectionProps {
    /// Padding utility classes (e.g., "py-12")
    #[prop_or_else(|| classes!("py-12"))]
    pub padding: Classes,

    /// Background color (e.g., "bg-gray-100")
    #[prop_or_default]
    pub background_color: Option<String>,

    /// Additional classes applied to the `<section>`
    #[prop_or_default]
    pub class: Classes,

    /// Additional classes applied to the `<Container>`
    #[prop_or_default]
    pub container_class: Classes,

    #[prop_or_default]
    pub id: Option<String>,

    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,

    #[prop_or_default]
    pub style: Option<String>,

    pub children: Children,
}

#[function_component(Section)]
pub fn section(props: &SectionProps) -> Html {
    let SectionProps {
        padding,
        background_color,
        class,
        container_class,
        id,
        onclick,
        style,
        children,
    } = props;

    let base_classes = classes!(
        padding.clone(),
        background_color.clone().unwrap_or_default(),
        class.clone()
    );

    html! {
        <section
            id={id.clone()}
            class={base_classes}
            onclick={onclick.clone()}
            style={style.clone()}
        >
            <Container class={container_class.clone()}>
                { for children.iter() }
            </Container>
        </section>
    }
}
