use yew::prelude::*;

/// Properties for the Spacer component
#[derive(Properties, PartialEq, Clone)]
pub struct SpacerProps {
    #[prop_or(16)] // Default size for the spacing (16px)
    pub size: u32, // Size of the space in pixels
    #[prop_or_default]
    pub horizontal: bool, // Whether the spacer is horizontal or vertical (default is vertical)
}

#[function_component(Spacer)]
pub fn spacer(props: &SpacerProps) -> Html {
    let SpacerProps { size, horizontal } = props.clone();

    // Determine the style based on whether the spacer is horizontal or vertical
    let style = if horizontal {
        format!("width: {}px; height: 1px;", size) // Horizontal spacer with specified width
    } else {
        format!("height: {}px; width: 100%;", size) // Vertical spacer with specified height
    };

    html! {
        <div style={style} />
    }
}
