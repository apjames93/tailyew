use yew::prelude::*;

/// Semantic tags for the Typo component
#[derive(PartialEq, Clone, Default)]
pub enum TagType {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    P,
    BlockQuote,
    Emphasis,
    Strong,
    Error,
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
    pub style: Option<String>,

    #[prop_or_default]
    pub id: Option<String>,

    #[prop_or_default]
    pub aria_label: Option<String>,

    #[prop_or_default]
    pub aria_describedby: Option<String>,

    #[prop_or_default]
    pub role: Option<String>,
}

#[component(Typo)]
pub fn typo(props: &TypoProps) -> Html {
    let TypoProps {
        children,
        tag,
        class,
        style,
        id,
        aria_label,
        aria_describedby,
        role,
    } = props;

    let base_classes = match tag {
        TagType::H1 => "text-4xl font-bold text-gray-900 dark:text-gray-100 mb-8",
        TagType::H2 => "text-3xl font-semibold text-gray-800 dark:text-gray-200 mb-6",
        TagType::H3 => "text-2xl font-medium text-gray-700 dark:text-gray-300 mb-5",
        TagType::H4 => "text-xl font-medium text-gray-700 dark:text-gray-300 mb-4",
        TagType::H5 => "text-lg font-medium text-gray-700 dark:text-gray-300 mb-3",
        TagType::H6 => "text-base font-medium text-gray-700 dark:text-gray-300 mb-2",
        TagType::BlockQuote => "border-l-4 pl-4 italic text-gray-600 dark:text-gray-400 my-4",
        TagType::Emphasis => "italic text-gray-700 dark:text-gray-300",
        TagType::Strong => "font-bold text-gray-900 dark:text-gray-100",
        TagType::P => "text-base text-gray-800 dark:text-gray-300 mb-4",
        TagType::Error => "text-sm text-red-600 dark:text-red-400 font-medium mt-2",
        TagType::Span => "text-sm text-gray-600 dark:text-gray-400",
    };

    let all_classes = classes!(base_classes, class.clone());

    // Smart default: auto-role alert for errors
    let resolved_role = match (&tag, &role) {
        (TagType::Error, None) => Some("alert".into()),
        _ => role.clone(),
    };

    macro_rules! render_tag {
        ($tag:ident) => {
            html! {
                <$tag
                    class={all_classes.clone()}
                    style={style.clone()}
                    id={id.clone()}
                    aria-label={aria_label.clone()}
                    aria-describedby={aria_describedby.clone()}
                    role={resolved_role.clone()}
                >
                    { for children.iter() }
                </$tag>
            }
        };
    }

    match tag {
        TagType::H1 => render_tag!(h1),
        TagType::H2 => render_tag!(h2),
        TagType::H3 => render_tag!(h3),
        TagType::H4 => render_tag!(h4),
        TagType::H5 => render_tag!(h5),
        TagType::H6 => render_tag!(h6),
        TagType::BlockQuote => render_tag!(blockquote),
        TagType::Emphasis => render_tag!(em),
        TagType::Strong => render_tag!(strong),
        TagType::P => render_tag!(p),
        TagType::Error => render_tag!(p),
        TagType::Span => render_tag!(span),
    }
}
