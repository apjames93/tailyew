use crate::system::use_themed_classes;
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
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub id: Option<AttrValue>,

    #[prop_or_default]
    pub aria_label: Option<AttrValue>,

    #[prop_or_default]
    pub aria_describedby: Option<AttrValue>,

    #[prop_or_default]
    pub role: Option<AttrValue>,
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
        TagType::H1 => "text-4xl font-bold text-content dark:text-content-invert mb-8",
        TagType::H2 => "text-3xl font-semibold text-content dark:text-content-invert mb-6",
        TagType::H3 => "text-2xl font-medium text-content dark:text-content-invert mb-5",
        TagType::H4 => "text-xl font-medium text-content dark:text-content-invert mb-4",
        TagType::H5 => "text-lg font-medium text-content dark:text-content-invert mb-3",
        TagType::H6 => "text-base font-medium text-content dark:text-content-invert mb-2",
        TagType::BlockQuote => {
            "border-l-4 border-border dark:border-border-dark pl-4 italic text-content-muted dark:text-content-muted-dark my-4"
        }
        TagType::Emphasis => "italic text-content dark:text-content-invert opacity-90",
        TagType::Strong => "font-bold text-content dark:text-content-invert",
        TagType::P => "text-base text-content dark:text-content-invert opacity-90 mb-4",
        TagType::Error => "text-sm text-danger dark:text-danger-dark font-medium mt-2",
        TagType::Span => "text-sm text-content-muted dark:text-content-muted-dark",
    };

    let all_classes = use_themed_classes("Typo", "root", classes!(base_classes), class.clone());

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
