use yew::prelude::*;

/// Define the possible HTML tag types for the Typography component
#[derive(PartialEq, Clone, Default)]
pub enum TagType {
    H1,
    H2,
    H3,
    H4,
    H5,
    P,
    #[default]
    Span,
}

#[derive(Properties, PartialEq, Clone)]
pub struct TypoProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub tag: TagType,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub margin_class: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,
}

/// Typo component that renders different text elements based on the `tag` prop
#[function_component(Typo)]
pub fn typo(props: &TypoProps) -> Html {
    let TypoProps {
        children,
        tag,
        class,
        margin_class,
        style,
    } = props.clone();

    // Determine base styles for different tag types
    let base_classes = match tag {
        TagType::H1 => "text-4xl font-bold text-gray-900 dark:text-gray-100 mb-8", // Larger margin for H1
        TagType::H2 => "text-3xl font-semibold text-gray-800 dark:text-gray-200 mb-6",
        TagType::H3 => "text-2xl font-medium text-gray-700 dark:text-gray-300 mb-5",
        TagType::H4 => "text-xl font-medium text-gray-700 dark:text-gray-300 mb-4",
        TagType::H5 => "text-lg font-medium text-gray-700 dark:text-gray-300 mb-3",
        TagType::P => "text-base text-gray-800 dark:text-gray-300 mb-4", // Consistent spacing for paragraphs
        TagType::Span => "text-sm text-gray-600 dark:text-gray-400", // No margin for inline text
    };

    // Combine the base classes, optional margin class, and any additional classes provided as props
    let classes = classes!(
        base_classes,
        margin_class.unwrap_or_default(), // Apply custom margin if provided
        class         // Include custom class if provided
    );

    // Render the appropriate HTML tag based on the `tag` prop
    match tag {
        TagType::H1 => html! { <h1 class={classes} style={style}>{ for children.iter() }</h1> },
        TagType::H2 => html! { <h2 class={classes} style={style}>{ for children.iter() }</h2> },
        TagType::H3 => html! { <h3 class={classes} style={style}>{ for children.iter() }</h3> },
        TagType::H4 => html! { <h4 class={classes} style={style}>{ for children.iter() }</h4> },
        TagType::H5 => html! { <h5 class={classes} style={style}>{ for children.iter() }</h5> },
        TagType::P => html! { <p class={classes} style={style}>{ for children.iter() }</p> },
        TagType::Span => {
            html! { <span class={classes} style={style}>{ for children.iter() }</span> }
        }
    }
}
