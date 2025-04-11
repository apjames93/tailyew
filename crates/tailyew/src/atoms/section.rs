use crate::atoms::Container;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct SectionProps {
    #[prop_or_default]
    pub background_color: Option<String>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub container_class: Classes,

    #[prop_or_default]
    pub id: Option<String>,

    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,

    #[prop_or_default]
    pub style: Option<String>,

    pub children: Children,

    #[prop_or(true)]
    pub with_container: bool,

    #[prop_or_default]
    pub aria_label: Option<String>,

    #[prop_or_default]
    pub aria_labelledby: Option<String>,
}

#[function_component(Section)]
pub fn section(props: &SectionProps) -> Html {
    let SectionProps {
        aria_label,
        aria_labelledby,
        background_color,
        class,
        container_class,
        id,
        onclick,
        style,
        children,
        with_container,
    } = props;

    let base_classes = classes!(background_color.clone().unwrap_or_default(), class.clone(),);

    if *with_container {
        html! {
            <section
                id={id.clone()}
                class={base_classes}
                onclick={onclick.clone()}
                style={style.clone()}
                aria-label={aria_label.clone()}
                aria-labelledby={aria_labelledby.clone()}
            >
                <Container class={container_class.clone()}>
                    { for children.iter() }
                </Container>
            </section>
        }
    } else {
        html! {
            <section
                id={id.clone()}
                class={base_classes}
                onclick={onclick.clone()}
                style={style.clone()}
                aria-label={aria_label.clone()}
                aria-labelledby={aria_labelledby.clone()}
            >
                { for children.iter() }
            </section>
        }
    }
}
