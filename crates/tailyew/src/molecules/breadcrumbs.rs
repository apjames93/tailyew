use crate::system::use_themed_classes;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct BreadcrumbsProps {
    /// Each item is typically a Link, Button, or Typo
    pub children: Children,

    /// Optional separator element between items (default: "/")
    #[prop_or_else(|| html! { "/" })]
    pub separator: Html,

    /// Optional wrapper class for styling (e.g. spacing, font)
    #[prop_or_default]
    pub class: Classes,

    /// Optional aria-label (default: "breadcrumb")
    #[prop_or_else(|| "breadcrumb".to_string())]
    pub aria_label: String,
}

#[component(Breadcrumbs)]
pub fn breadcrumbs(props: &BreadcrumbsProps) -> Html {
    let BreadcrumbsProps {
        children,
        separator,
        class,
        aria_label,
    } = props;

    let items: Vec<Html> = children.iter().collect();
    let len = items.len();
    let root_classes = use_themed_classes(
        "Breadcrumbs",
        "root",
        classes!("w-full", "overflow-x-auto", "py-2"),
        class.clone(),
    );

    html! {
        <nav aria-label={aria_label.clone()} class={root_classes}>
            <ol class="flex items-center gap-x-2 text-sm text-gray-600 dark:text-gray-300">
                {
                    for items.iter().enumerate().flat_map(|(i, item)| {
                        let mut out = vec![
                            html! {
                                <li
                                    class={classes!(
                                        "truncate",
                                        "max-w-[150px]",
                                        if i == len - 1 {
                                            "font-semibold text-gray-900 dark:text-white"
                                        } else {
                                            "hover:underline cursor-pointer"
                                        }
                                    )}
                                    aria-current={if i == len - 1 { Some("page") } else { None }}
                                >
                                    <span class="px-1">{ item.clone() }</span>
                                </li>
                            }
                        ];

                        if i < len - 1 {
                            out.push(html! {
                                <li
                                    class="mx-1 select-none text-gray-400 dark:text-gray-500"
                                    aria-hidden="true"
                                >
                                    { separator.clone() }
                                </li>
                            });
                        }

                        out
                    })
                }
            </ol>
        </nav>
    }
}
