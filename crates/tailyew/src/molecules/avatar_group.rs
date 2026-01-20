use crate::atoms::Avatar;
use yew::prelude::*;

#[derive(Clone, PartialEq, Default)]
pub struct AvatarData {
    pub src: Option<AttrValue>,
    pub fallback: Option<AttrValue>,
    pub alt: Option<AttrValue>,
    pub class: Classes,
}

#[derive(Properties, PartialEq, Clone)]
pub struct AvatarGroupProps {
    pub avatars: Vec<AvatarData>,

    /// Max number to display before showing overflow
    #[prop_or(5)]
    pub max_visible: usize,

    /// Size class passed to each avatar (e.g. "w-10 h-10")
    #[prop_or_else(|| "w-10 h-10".into())]
    pub size: Classes,

    /// Alignment direction (LTR or RTL)
    #[prop_or_default]
    pub reverse: bool,

    /// Additional wrapper classes
    #[prop_or_default]
    pub class: Classes,
}

#[component(AvatarGroup)]
pub fn avatar_group(props: &AvatarGroupProps) -> Html {
    let AvatarGroupProps {
        avatars,
        max_visible,
        size,
        reverse,
        class,
    } = props.clone();

    let display_avatars = avatars.iter().take(max_visible);
    let extra_count = avatars.len().saturating_sub(max_visible);

    let mut items: Vec<Html> = display_avatars
        .enumerate()
        .map(|(i, data)| {
            let overlap_class = if i == 0 { "" } else { "-ml-4" };
            html! {
                <Avatar
                    src={data.src.clone()}
                    fallback={data.fallback.clone()}
                    alt={data.alt.clone()}
                    size={size.clone()}
                    class={classes!(overlap_class, data.class.clone())}
                />
            }
        })
        .collect();

    if extra_count > 0 {
        items.push(html! {
            <div class={classes!(
                "inline-flex", "items-center", "justify-center", "rounded-full", "bg-gray-300", "dark:bg-gray-600", "text-sm", "text-white", "font-medium",
                size.clone(),
                "-ml-4"
            )}>
                { format!("+{}", extra_count) }
            </div>
        });
    }

    if reverse {
        items.reverse();
    }

    html! {
        <div class={classes!("flex", class.clone())}>
            { for items }
        </div>
    }
}
