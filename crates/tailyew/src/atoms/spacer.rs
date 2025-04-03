use yew::prelude::*;

/// A simple vertical or horizontal spacer used for layout control.
#[derive(Properties, PartialEq, Clone)]
pub struct SpacerProps {
    /// Size in pixels (default: 16)
    #[prop_or(16)]
    pub size: u32,

    /// If true, renders a horizontal spacer (width). Default: false (vertical).
    #[prop_or_default]
    pub horizontal: bool,

    /// Optional Tailwind class overrides or additions
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Spacer)]
pub fn spacer(props: &SpacerProps) -> Html {
    let SpacerProps {
        size,
        horizontal,
        class,
    } = props;

    let style = if *horizontal {
        format!("display: inline-block; width: {}px; height: 1px;", size)
    } else {
        format!("display: block; height: {}px; width: 100%;", size)
    };

    html! {
        <div style={style} class={class.clone()} />
    }
}
