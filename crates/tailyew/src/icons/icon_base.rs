use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct IconBaseProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or(24)]
    pub size: u32,
    #[prop_or(1.5)]
    pub stroke_width: f32,
    #[prop_or_default]
    pub label: Option<AttrValue>,
    #[prop_or(false)]
    pub decorative: bool,
    #[prop_or_default]
    pub children: Children,
}

#[component(IconBase)]
pub fn icon_base(props: &IconBaseProps) -> Html {
    let aria_label = props.label.clone().unwrap_or_default();

    html! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            width={props.size.to_string()}
            height={props.size.to_string()}
            stroke-width={props.stroke_width.to_string()}
            class={classes!(
                "inline-block",
                "text-gray-700",
                "dark:text-gray-200",
                props.class.clone()
            )}
            role="img"
            aria-label={(!props.decorative).then_some(aria_label.clone())}
            aria-hidden={props.decorative.then_some("true")}
        >
            { for props.children.iter() }
        </svg>
    }
}
