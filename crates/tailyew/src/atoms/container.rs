use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ContainerProps {
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    pub children: Children,
}

#[function_component(Container)]
pub fn container(props: &ContainerProps) -> Html {
    let ContainerProps {
        class,
        id,
        children,
    } = props.clone();

    let base_classes = classes!(
        "max-w-screen-xl",
        "mx-auto",
        "px-4",
        "sm:px-6",
        "lg:px-8",
        class.unwrap_or_default(),
    );

    html! {
        <div id={id} class={base_classes}>
            { for children.iter() }
        </div>
    }
}
