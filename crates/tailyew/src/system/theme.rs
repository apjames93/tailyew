//! Theme context and component-slot class overrides.
//!
//! TailYew theming has two layers:
//! - root theme metadata/classes from `InitTheme`
//! - component slot overrides from `ThemeOverrides`
//!
//! Components should merge classes in this order:
//! `defaults -> theme override -> props class`.
//!
//! `InitTheme` keeps existing root behavior (`data-theme` + root class merge), and also
//! provides `ThemeContext` so component hooks can read overrides.
//!
//! Slot naming convention:
//! - every component supports `"root"`
//! - extra slots use stable names (`"image"`, `"body"`, `"trigger"`, `"content"`, etc.)
//!
//! ```rust,ignore
//! use yew::prelude::*;
//! use tailyew::{A, Button, InitTheme, Theme, ThemeOverrides};
//!
//! html! {
//!   <InitTheme
//!     theme={Some(Theme {
//!       name: "dark".into(),
//!       class: classes!("bg-gray-900", "text-white"),
//!       overrides: ThemeOverrides::new()
//!         .set("A", "root", classes!("text-white", "hover:text-gray-200"))
//!         .set("Button", "root", classes!("rounded-xl"))
//!         .set("Card", "image", classes!("rounded-none")),
//!     })}
//!   >
//!     <A href="/">{ "wow" }</A>
//!     <Button>{ "Press me" }</Button>
//!   </InitTheme>
//! }
//! ```

use std::collections::HashMap;
use yew::prelude::*;

/// Shared slot name for the top-level class merge in most components.
pub const ROOT_SLOT: &str = "root";

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct ThemeOverrideKey {
    component: String,
    slot: String,
}

impl ThemeOverrideKey {
    fn new(component: impl Into<String>, slot: impl Into<String>) -> Self {
        Self {
            component: component.into(),
            slot: slot.into(),
        }
    }
}

/// MUI-like class overrides keyed by `(component, slot)`.
///
/// This uses string keys by design:
/// - no per-component schema maintenance burden as the crate grows
/// - end users can override current and future components without waiting on enum updates
/// - static literals (`"Button"`, `"root"`, etc.) still give compile-time spell-checking in code
#[derive(Clone, Debug, PartialEq, Default)]
pub struct ThemeOverrides {
    classes_by_slot: HashMap<ThemeOverrideKey, Classes>,
}

impl ThemeOverrides {
    pub fn new() -> Self {
        Self::default()
    }

    /// Builder-style setter, useful for fluent theme construction.
    pub fn set(
        mut self,
        component: impl Into<String>,
        slot: impl Into<String>,
        class: Classes,
    ) -> Self {
        self.insert(component, slot, class);
        self
    }

    /// Mutable insert helper for non-builder usage.
    pub fn insert(
        &mut self,
        component: impl Into<String>,
        slot: impl Into<String>,
        class: Classes,
    ) -> &mut Self {
        self.classes_by_slot
            .insert(ThemeOverrideKey::new(component, slot), class);
        self
    }

    /// Convenience for overriding a component's `"root"` slot.
    pub fn set_root(self, component: impl Into<String>, class: Classes) -> Self {
        self.set(component, ROOT_SLOT, class)
    }

    pub fn get(&self, component: &str, slot: &str) -> Classes {
        self.classes_by_slot
            .get(&ThemeOverrideKey::new(component, slot))
            .cloned()
            .unwrap_or_default()
    }
}

/// Context emitted by `InitTheme`.
#[derive(Clone, PartialEq, Debug, Default)]
pub struct ThemeContext {
    pub name: AttrValue,
    pub root_class: Classes,
    pub overrides: ThemeOverrides,
}

#[hook]
pub fn use_themed_classes(
    component: &'static str,
    slot: &'static str,
    defaults: Classes,
    props_class: Classes,
) -> Classes {
    if let Some(theme) = use_context::<ThemeContext>() {
        let override_classes = theme.overrides.get(component, slot);
        classes!(defaults, override_classes, props_class)
    } else {
        classes!(defaults, props_class)
    }
}
