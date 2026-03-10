use super::render_label_with_required_indicator;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone, Default)]
pub struct LabelProps {
    #[prop_or_default]
    pub id: Option<AttrValue>,

    #[prop_or_default]
    pub for_id: AttrValue,

    #[prop_or_default]
    pub text: AttrValue,

    #[prop_or(false)]
    pub required: bool,

    #[prop_or_default]
    pub class: Classes,
}

#[component(Label)]
pub fn label(props: &LabelProps) -> Html {
    let LabelProps {
        id,
        for_id,
        text,
        required,
        class,
    } = props;

    let label_classes = classes!(
        "block",
        "text-sm",
        "font-medium",
        "text-left",
        "text-gray-700",
        "dark:text-gray-300",
        class.clone()
    );

    let for_attr = (!for_id.is_empty()).then_some(for_id.clone());

    html! {
        <label id={id.clone()} for={for_attr} class={label_classes}>
            { render_label_with_required_indicator(text, *required) }
        </label>
    }
}
