use crate::XIcon;
use yew::prelude::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub enum ChipVariant {
    #[default]
    Neutral,
    Primary,
    Success,
    Warning,
    Danger,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum ChipSize {
    Small,
    #[default]
    Medium,
}

#[derive(Properties, PartialEq)]
pub struct ChipProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub variant: ChipVariant,

    #[prop_or_default]
    pub size: ChipSize,

    #[prop_or(false)]
    pub removable: bool,

    #[prop_or(false)]
    pub disabled: bool,

    #[prop_or_default]
    pub on_remove: Option<Callback<MouseEvent>>,

    #[prop_or_default]
    pub remove_aria_label: Option<AttrValue>,

    #[prop_or_else(|| AttrValue::from("Remove"))]
    pub remove_title: AttrValue,

    #[prop_or_default]
    pub class: Classes,
}

#[component(Chip)]
pub fn chip(props: &ChipProps) -> Html {
    let ChipProps {
        children,
        variant,
        size,
        removable,
        disabled,
        on_remove,
        remove_aria_label,
        remove_title,
        class,
    } = props;

    let variant_classes = match variant {
        ChipVariant::Neutral => "bg-gray-100 text-gray-700 dark:bg-gray-700 dark:text-gray-100",
        ChipVariant::Primary => "bg-blue-50 text-blue-700 dark:bg-blue-950 dark:text-blue-200",
        ChipVariant::Success => "bg-green-50 text-green-700 dark:bg-green-950 dark:text-green-200",
        ChipVariant::Warning => {
            "bg-yellow-50 text-yellow-800 dark:bg-yellow-950 dark:text-yellow-200"
        }
        ChipVariant::Danger => "bg-red-50 text-red-700 dark:bg-red-950 dark:text-red-200",
    };

    let size_classes = match size {
        ChipSize::Small => "h-7 gap-1 px-2.5 text-sm",
        ChipSize::Medium => "h-8 gap-1.5 px-3 text-sm",
    };

    let remove_size_classes = match size {
        ChipSize::Small => "h-5 w-5",
        ChipSize::Medium => "h-6 w-6",
    };

    let remove_disabled = *disabled || on_remove.is_none();

    html! {
        <span
            class={classes!(
                "inline-flex",
                "items-center",
                "rounded-full",
                "font-medium",
                variant_classes,
                size_classes,
                if *disabled { "opacity-50" } else { "" },
                class.clone(),
            )}
        >
            <span>{ for children.iter() }</span>
            if *removable {
                <button
                    type="button"
                    class={classes!(
                        "inline-flex",
                        "items-center",
                        "justify-center",
                        "rounded-full",
                        "text-current",
                        "opacity-70",
                        "transition",
                        "hover:bg-black/10",
                        "hover:opacity-100",
                        "focus:outline-none",
                        "focus:ring-2",
                        "focus:ring-primary",
                        "disabled:cursor-not-allowed",
                        "disabled:opacity-40",
                        "dark:hover:bg-white/10",
                        remove_size_classes,
                    )}
                    aria-label={remove_aria_label.clone().unwrap_or_else(|| remove_title.clone())}
                    title={remove_title.clone()}
                    disabled={remove_disabled}
                    onclick={on_remove.clone()}
                >
                    <XIcon size={12} decorative=true />
                </button>
            }
        </span>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use yew::html;

    #[test]
    fn chip_public_props_accept_string_literals_for_accessible_text() {
        let _ = html! {
            <Chip
                removable={true}
                remove_aria_label="Remove beta"
                remove_title="Remove tag"
            >
                { "beta" }
            </Chip>
        };
    }
}
