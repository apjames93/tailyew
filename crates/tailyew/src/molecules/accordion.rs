use crate::atoms::TagType;
use js_sys::Date;
use std::collections::HashSet;
use wasm_bindgen::JsCast;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct AccordionProps {
    pub title: Html,
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub trigger_classes: Classes,

    #[prop_or_default]
    pub content_class: Classes,

    #[prop_or(TagType::Span)]
    pub heading_tag: TagType,

    /// Initial open state when the accordion manages its own state.
    /// Ignored when `is_open` is provided.
    #[prop_or(false)]
    pub default_open: bool,

    /// Controlled open state. When set, the accordion renders from this value
    /// and emits `on_toggle` instead of mutating internal state directly.
    #[prop_or_default]
    pub is_open: Option<bool>,

    /// Called with the next intended open state whenever the accordion is
    /// toggled. In uncontrolled mode this fires after local state updates.
    #[prop_or_default]
    pub on_toggle: Option<Callback<bool>>,

    #[prop_or(false)]
    pub compact: bool,

    #[prop_or_default]
    pub arrow: Option<Html>,
}

#[component(Accordion)]
pub fn accordion(props: &AccordionProps) -> Html {
    let internal_is_open = use_state({
        let default_open = props.default_open;
        move || default_open
    });
    let effective_is_open = props.is_open.unwrap_or(*internal_is_open);

    let request_toggle = {
        let internal_is_open = internal_is_open.clone();
        let is_open = props.is_open;
        let on_toggle = props.on_toggle.clone();

        Callback::from(move |_| {
            let current_is_open = is_open.unwrap_or(*internal_is_open);
            let next_is_open = !current_is_open;

            if is_open.is_none() {
                internal_is_open.set(next_is_open);
            }

            if let Some(on_toggle) = &on_toggle {
                on_toggle.emit(next_is_open);
            }
        })
    };

    let toggle_open = {
        let request_toggle = request_toggle.clone();
        Callback::from(move |e: MouseEvent| {
            let target = e.target();
            if let Some(el) = target {
                let tag = el
                    .dyn_ref::<web_sys::Element>()
                    .map(|el| el.tag_name().to_ascii_lowercase());
                if matches!(
                    tag.as_deref(),
                    Some("input" | "textarea" | "button" | "select")
                ) {
                    // Click was inside an interactive field, don't toggle
                    return;
                }
            }
            request_toggle.emit(());
        })
    };

    let on_keypress = {
        let request_toggle = request_toggle.clone();
        Callback::from(move |e: KeyboardEvent| {
            let tag = e.target().and_then(|t| {
                t.dyn_ref::<web_sys::Element>()
                    .map(|el| el.tag_name().to_ascii_lowercase())
            });
            if matches!(
                tag.as_deref(),
                Some("input" | "textarea" | "button" | "select")
            ) {
                // Ignore keypresses in interactive elements
                return;
            }

            if e.key() == "Enter" || e.key() == " " {
                e.prevent_default();
                request_toggle.emit(());
            }
        })
    };

    let trigger_text_color_classes = extract_text_color_classes(&props.trigger_classes);

    let wrapper_classes = if props.compact {
        merge_tailwind_classes(&["rounded-lg"], &props.class)
    } else {
        merge_tailwind_classes(
            &[
                "border",
                "border-gray-300",
                "dark:border-gray-700",
                "rounded-lg",
                "overflow-hidden",
                "shadow-md",
            ],
            &props.class,
        )
    };

    let base_trigger_classes = if props.compact {
        merge_tailwind_classes(
            &[
                "w-full",
                "bg-gray-200",
                "dark:bg-gray-800",
                "hover:bg-gray-300",
                "dark:hover:bg-gray-700",
                "px-4",
                "py-3",
                "flex",
                "items-center",
                "justify-between",
                "rounded-lg",
                "shadow-md",
                "transition",
                "text-gray-700",
                "dark:text-gray-300",
            ],
            &props.trigger_classes,
        )
    } else {
        merge_tailwind_classes(
            &[
                "cursor-pointer",
                "bg-gray-200",
                "dark:bg-gray-800",
                "hover:bg-gray-300",
                "dark:hover:bg-gray-700",
                "px-3",
                "py-2",
                "flex",
                "justify-between",
                "items-center",
                "transition",
                "duration-200",
                "text-gray-700",
                "dark:text-gray-300",
            ],
            &props.trigger_classes,
        )
    };

    let arrow_classes = if effective_is_open {
        merge_tailwind_classes(
            &[
                "shrink-0",
                "transform",
                "transition-transform",
                "duration-200",
                "rotate-180",
                "text-primary",
            ],
            &trigger_text_color_classes,
        )
    } else {
        merge_tailwind_classes(
            &[
                "shrink-0",
                "transform",
                "transition-transform",
                "duration-200",
                "text-gray-500",
                "dark:text-gray-400",
            ],
            &trigger_text_color_classes,
        )
    };

    let base_content_classes = if props.compact {
        merge_tailwind_classes(&["space-y-1"], &props.content_class)
    } else {
        merge_tailwind_classes(
            &[
                "px-3",
                "py-2",
                "border-t",
                "dark:border-gray-700",
                "bg-white",
                "dark:bg-gray-900",
                "transition-all",
                "duration-300",
                "ease-in-out",
            ],
            &props.content_class,
        )
    };

    let panel_id = use_state(|| AttrValue::from(format!("accordion-panel-{}", Date::now() as u64)));

    // Tailwind transition class handling
    let visibility_classes = if props.compact {
        classes!(
            "overflow-hidden",
            "transition-[max-height]",
            "duration-300",
            "ease-in-out",
            if effective_is_open {
                "max-h-[1000px]"
            } else {
                "max-h-0"
            }
        )
    } else {
        let state_classes = if effective_is_open {
            vec!["opacity-100", "scale-y-100"]
        } else {
            vec!["opacity-0", "scale-y-0", "pointer-events-none", "hidden"]
        };

        classes!(
            "transition-all",
            "duration-300",
            "ease-in-out",
            "transform",
            "origin-top",
            state_classes
        )
    };

    html! {
        <div class={wrapper_classes}>
            <div
                role="button"
                tabindex="0"
                class={base_trigger_classes}
                onclick={toggle_open}
                onkeypress={on_keypress}
                aria-expanded={effective_is_open.to_string()}
                aria-controls={(*panel_id).clone()}
            >
                { render_title(&props.heading_tag, props.title.clone()) }
                {
                    if let Some(icon) = props.arrow.clone() {
                        icon
                    } else {
                        default_arrow_icon(arrow_classes)
                    }
                }
            </div>

            <div
                id={(*panel_id).clone()}
                class={classes!(base_content_classes, visibility_classes)}
            >
                { for props.children.iter() }
            </div>
        </div>
    }
}

// Tailwind resolves conflicting utilities by stylesheet generation order, not
// by the order classes appear in the HTML attribute. This merge keeps Accordion
// defaults intact while allowing consumer classes on the same node to replace
// conflicting default utilities predictably.
fn merge_tailwind_classes(defaults: &[&str], custom: &Classes) -> Classes {
    let custom_groups = custom
        .into_iter()
        .filter_map(|class| tailwind_group_key(class.as_str()))
        .collect::<HashSet<_>>();

    let mut merged = Classes::with_capacity(defaults.len() + custom.into_iter().count());

    for default_class in defaults {
        if tailwind_group_key(default_class)
            .as_ref()
            .is_some_and(|group| custom_groups.contains(group))
        {
            continue;
        }

        merged.push((*default_class).to_owned());
    }

    merged.push(custom.clone());
    merged
}

fn extract_text_color_classes(classes: &Classes) -> Classes {
    let mut extracted = Classes::new();

    for class in classes {
        let (_, base) = split_variants(class.as_str());
        if utility_group(base).as_deref() == Some("text-color") {
            extracted.push(class);
        }
    }

    extracted
}

fn render_title(tag: &TagType, title: Html) -> Html {
    let title_classes = classes!("w-full", "text-left", "m-0", title_typography_classes(tag));

    match tag {
        TagType::H1 => html! { <h1 class={title_classes}>{ title }</h1> },
        TagType::H2 => html! { <h2 class={title_classes}>{ title }</h2> },
        TagType::H3 => html! { <h3 class={title_classes}>{ title }</h3> },
        TagType::H4 => html! { <h4 class={title_classes}>{ title }</h4> },
        TagType::H5 => html! { <h5 class={title_classes}>{ title }</h5> },
        TagType::H6 => html! { <h6 class={title_classes}>{ title }</h6> },
        TagType::P => html! { <p class={title_classes}>{ title }</p> },
        TagType::BlockQuote => html! { <blockquote class={title_classes}>{ title }</blockquote> },
        TagType::Emphasis => html! { <em class={title_classes}>{ title }</em> },
        TagType::Strong => html! { <strong class={title_classes}>{ title }</strong> },
        TagType::Error => html! { <p class={title_classes}>{ title }</p> },
        TagType::Span => html! { <span class={title_classes}>{ title }</span> },
    }
}

fn title_typography_classes(tag: &TagType) -> Classes {
    match tag {
        TagType::H1 => classes!("text-4xl", "font-bold"),
        TagType::H2 => classes!("text-3xl", "font-semibold"),
        TagType::H3 => classes!("text-2xl", "font-medium"),
        TagType::H4 => classes!("text-xl", "font-medium"),
        TagType::H5 => classes!("text-lg", "font-medium"),
        TagType::H6 => classes!("text-base", "font-medium"),
        TagType::P => classes!("text-base"),
        TagType::BlockQuote => classes!("border-l-4", "pl-4", "italic"),
        TagType::Emphasis => classes!("italic"),
        TagType::Strong => classes!("font-bold"),
        TagType::Error => classes!("text-sm", "font-medium"),
        TagType::Span => classes!("text-sm"),
    }
}

fn default_arrow_icon(classes: Classes) -> Html {
    html! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            width="24"
            height="24"
            stroke-width="1.5"
            class={classes!("inline-block", classes)}
            aria-hidden="true"
            focusable="false"
        >
            <path
                d="M12 5v14M19 12l-7 7-7-7"
                stroke-linecap="round"
                stroke-linejoin="round"
            />
        </svg>
    }
}

fn tailwind_group_key(token: &str) -> Option<String> {
    let trimmed = token.trim();
    if trimmed.is_empty() {
        return None;
    }

    let trimmed = trimmed.strip_prefix('!').unwrap_or(trimmed);
    let (variants, base) = split_variants(trimmed);
    let group = utility_group(base)?;

    if variants.is_empty() {
        Some(group)
    } else {
        Some(format!("{variants}:{group}"))
    }
}

fn split_variants(token: &str) -> (&str, &str) {
    let mut bracket_depth = 0_u8;
    let mut last_separator = None;

    for (index, ch) in token.char_indices() {
        match ch {
            '[' => bracket_depth = bracket_depth.saturating_add(1),
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            ':' if bracket_depth == 0 => last_separator = Some(index),
            _ => {}
        }
    }

    if let Some(index) = last_separator {
        (&token[..index], &token[index + 1..])
    } else {
        ("", token)
    }
}

fn utility_group(base: &str) -> Option<String> {
    if base == "border" {
        return Some(String::from("border-all-width"));
    }

    if let Some(group) = prefixed_group(
        base,
        &[
            "rounded-tl",
            "rounded-tr",
            "rounded-br",
            "rounded-bl",
            "rounded-s",
            "rounded-e",
            "rounded-t",
            "rounded-r",
            "rounded-b",
            "rounded-l",
            "rounded",
            "overflow-x",
            "overflow-y",
            "overflow",
            "space-x",
            "space-y",
            "gap-x",
            "gap-y",
            "gap",
            "px",
            "py",
            "ps",
            "pe",
            "pt",
            "pr",
            "pb",
            "pl",
            "p",
            "mx",
            "my",
            "ms",
            "me",
            "mt",
            "mr",
            "mb",
            "ml",
            "m",
            "max-w",
            "min-w",
            "max-h",
            "min-h",
            "w",
            "h",
        ],
    ) {
        return Some(group.to_string());
    }

    if matches!(
        base,
        "block"
            | "inline-block"
            | "inline"
            | "flex"
            | "inline-flex"
            | "grid"
            | "inline-grid"
            | "hidden"
    ) {
        return Some(String::from("display"));
    }

    if base.starts_with("bg-") {
        return Some(String::from("bg"));
    }

    if base.starts_with("shadow") {
        return Some(String::from("shadow"));
    }

    if base.starts_with("cursor-") {
        return Some(String::from("cursor"));
    }

    if base.starts_with("justify-") {
        return Some(String::from("justify"));
    }

    if base.starts_with("items-") {
        return Some(String::from("items"));
    }

    if base == "transition" || base.starts_with("transition-") {
        return Some(String::from("transition"));
    }

    if base.starts_with("duration-") {
        return Some(String::from("duration"));
    }

    if base.starts_with("ease-") {
        return Some(String::from("ease"));
    }

    if base.starts_with("origin-") {
        return Some(String::from("origin"));
    }

    if base.starts_with("opacity-") {
        return Some(String::from("opacity"));
    }

    if base.starts_with("pointer-events-") {
        return Some(String::from("pointer-events"));
    }

    if base == "scale"
        || base.starts_with("scale-")
        || base.starts_with("scale-x-")
        || base.starts_with("scale-y-")
    {
        return Some(String::from("scale"));
    }

    if let Some(group) = border_group(base) {
        return Some(group);
    }

    if let Some(group) = text_group(base) {
        return Some(group);
    }

    if base.starts_with("font-") {
        return Some(String::from("font"));
    }

    None
}

fn prefixed_group<'a>(base: &str, prefixes: &'a [&'a str]) -> Option<&'a str> {
    prefixes.iter().copied().find(|prefix| {
        base == *prefix
            || base
                .strip_prefix(prefix)
                .is_some_and(|suffix| suffix.starts_with('-'))
    })
}

fn border_group(base: &str) -> Option<String> {
    let mut parts = base.split('-');
    if parts.next()? != "border" {
        return None;
    }

    let first = parts.next();
    let (scope, remainder) = match first {
        Some("x" | "y" | "s" | "e" | "t" | "r" | "b" | "l") => {
            let rest = parts.collect::<Vec<_>>();
            (first.unwrap(), rest)
        }
        Some(segment) => (
            "all",
            vec![segment].into_iter().chain(parts).collect::<Vec<_>>(),
        ),
        None => return Some(String::from("border-all-width")),
    };

    if remainder.is_empty() {
        return Some(format!("border-{scope}-width"));
    }

    if remainder.len() == 1 && is_border_width_token(remainder[0]) {
        return Some(format!("border-{scope}-width"));
    }

    Some(format!("border-{scope}-color"))
}

fn is_border_width_token(token: &str) -> bool {
    matches!(token, "0" | "2" | "4" | "8") || token.starts_with('[')
}

fn text_group(base: &str) -> Option<String> {
    let text = base.strip_prefix("text-")?;
    let text = text.split('/').next().unwrap_or(text);

    if matches!(
        text,
        "left" | "center" | "right" | "justify" | "start" | "end" | "ellipsis" | "clip"
    ) {
        return Some(String::from("text-align"));
    }

    if matches!(
        text,
        "xs" | "sm"
            | "base"
            | "lg"
            | "xl"
            | "2xl"
            | "3xl"
            | "4xl"
            | "5xl"
            | "6xl"
            | "7xl"
            | "8xl"
            | "9xl"
    ) {
        return Some(String::from("text-size"));
    }

    Some(String::from("text-color"))
}
