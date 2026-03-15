// crates/tailyew/src/molecules/navbar.rs

use crate::system::use_themed_classes;
use yew::prelude::*;

#[derive(PartialEq, Clone, Default)]
pub enum AppBarPosition {
    #[default]
    Static,
    Top,
    Bottom,
}

#[derive(Properties, PartialEq, Clone)]
pub struct NavBarProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub position: AppBarPosition,
}

#[component(NavBar)]
pub fn nav_bar(props: &NavBarProps) -> Html {
    let NavBarProps {
        children,
        class,
        id,
        position,
    } = props;
    let defaults = classes!(
        "bg-white",
        "dark:bg-gray-900",
        "shadow-md",
        "transition-colors",
        "duration-300",
        get_position_class(position.clone()),
    );
    let nav_classes = use_themed_classes("NavBar", "root", defaults, class.clone());

    html! {
        <nav
            id={id.clone()}
            class={nav_classes}
            role="navigation"
        >
            <div class="max-w-7xl mx-auto px-4 py-2 flex justify-between items-center">
                { for children.iter() }
            </div>
        </nav>
    }
}

fn get_position_class(position: AppBarPosition) -> &'static str {
    match position {
        AppBarPosition::Top => "fixed top-0 left-0 w-full z-50",
        AppBarPosition::Bottom => "fixed bottom-0 left-0 w-full z-50",
        AppBarPosition::Static => "w-full",
    }
}
