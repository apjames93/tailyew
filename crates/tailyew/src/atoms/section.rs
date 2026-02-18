use crate::atoms::Container;
use crate::system::use_themed_classes;
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
    pub id: Option<AttrValue>,

    #[prop_or_default]
    pub on_click: Option<Callback<MouseEvent>>,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    pub children: Children,

    #[prop_or(true)]
    pub with_container: bool,

    #[prop_or_default]
    pub aria_label: Option<AttrValue>,

    #[prop_or_default]
    pub aria_labelledby: Option<AttrValue>,
}

#[component(Section)]
pub fn section(props: &SectionProps) -> Html {
    let SectionProps {
        aria_label,
        aria_labelledby,
        background_color,
        class,
        container_class,
        id,
        on_click,
        style,
        children,
        with_container,
    } = props;

    let defaults = classes!(background_color.clone().unwrap_or_default());
    let base_classes = use_themed_classes("Section", "root", defaults, class.clone());
    let themed_container_class = use_themed_classes(
        "Section",
        "container",
        Classes::default(),
        container_class.clone(),
    );

    if *with_container {
        html! {
            <section
                id={id.clone()}
                class={base_classes}
                onclick={on_click.clone()}
                style={style.clone()}
                aria-label={aria_label.clone()}
                aria-labelledby={aria_labelledby.clone()}
            >
                <Container class={themed_container_class}>
                    { for children.iter() }
                </Container>
            </section>
        }
    } else {
        html! {
            <section
                id={id.clone()}
                class={base_classes}
                onclick={on_click.clone()}
                style={style.clone()}
                aria-label={aria_label.clone()}
                aria-labelledby={aria_labelledby.clone()}
            >
                { for children.iter() }
            </section>
        }
    }
}
