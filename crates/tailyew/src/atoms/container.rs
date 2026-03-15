use crate::system::use_themed_classes;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ContainerProps {
    /// Optional Tailwind utility classes
    #[prop_or_default]
    pub class: Classes,

    /// Optional DOM id
    #[prop_or_default]
    pub id: Option<AttrValue>,

    /// Child elements to render inside the container
    pub children: Children,
}

#[component(Container)]
pub fn container(props: &ContainerProps) -> Html {
    let ContainerProps {
        class,
        id,
        children,
    } = props;
    let merged_classes = use_themed_classes("Container", "root", Classes::default(), class.clone());

    html! {
        <div id={id.clone()} class={merged_classes}>
            { for children.iter() }
        </div>
    }
}
