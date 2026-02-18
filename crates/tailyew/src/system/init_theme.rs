use super::{ThemeContext, ThemeOverrides};

use yew::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct Theme {
    /// Tailwind-compatible theme name (for `data-theme`)
    pub name: String,

    /// Override classes to be merged into the root
    pub class: Classes,

    /// Component/slot-level class overrides
    pub overrides: ThemeOverrides,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            name: "system".to_string(),
            class: Classes::default(),
            overrides: ThemeOverrides::default(),
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

#[component(InitTheme)]
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

    let extra_theme_class = theme.class.clone();
    let theme_context = ThemeContext {
        name: theme.name.clone().into(),
        root_class: theme.class.clone(),
        overrides: theme.overrides.clone(),
    };

    html! {
        <div
            class={classes!(
                theme_class,
                extra_theme_class,
                class.clone()
            )}
            data-theme={theme.name.clone()}
        >
            <ContextProvider<ThemeContext> context={theme_context}>
                { for children.iter() }
            </ContextProvider<ThemeContext>>
        </div>
    }
}
