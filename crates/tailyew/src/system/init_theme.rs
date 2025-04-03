// crates/tailyew/src/system/init.rs

use yew::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct Theme {
    /// Tailwind-compatible theme name (for `data-theme`)
    pub name: String,

    /// Optional override classes to be merged into the root
    pub class: Option<String>,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            name: "system".to_string(),
            class: None,
        }
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct InitProps {
    #[prop_or_default]
    pub children: Children,

    /// Optional custom theme configuration
    #[prop_or_default]
    pub theme: Option<Theme>,

    /// Additional classes for root
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(InitTheme)]
pub fn init_theme(props: &InitProps) -> Html {
    let InitProps {
        children,
        theme,
        class,
    } = props;

    let theme = theme.clone().unwrap_or_default();

    let theme_class = match theme.name.as_str() {
        "light" => "light",
        "dark" => "dark",
        _ => "", // system = no explicit root class
    };

    let extra_theme_class = theme.class.unwrap_or_default();

    html! {
        <div
            class={classes!(
                theme_class,
                extra_theme_class,
                class.clone()
            )}
            data-theme={theme.name.clone()}
        >
            { for children.iter() }
        </div>
    }
}
