use crate::atoms::{TagType, Typo};
use yew::prelude::*;
// use yew::virtual_dom::{VNode, VTag, VText};

/// A flexible column definition supporting arbitrary Yew HTML for headers and cells
#[derive(Clone, PartialEq)]
pub struct Column {
    pub header: Html,
    pub values: Vec<Html>,
}

#[derive(Clone, Default, PartialEq)]
pub enum TableMobileLayout {
    #[default]
    Scroll,
    Stacked,
}

/// Properties for the Table component
#[derive(Properties, PartialEq, Clone)]
pub struct TableProps {
    pub columns: Vec<Column>,

    #[prop_or_default]
    pub row_click_callback: Option<Callback<usize>>,

    #[prop_or_default]
    pub mobile_layout: TableMobileLayout,

    #[prop_or_default]
    pub caption: Option<Html>,

    #[prop_or_default]
    pub aria_label: Option<AttrValue>,
}

#[component(Table)]
pub fn table(props: &TableProps) -> Html {
    let TableProps {
        columns,
        row_click_callback,
        mobile_layout,
        caption,
        aria_label,
    } = props.clone();

    let headers: Vec<Html> = columns.iter().map(|col| col.header.clone()).collect();
    let num_rows = columns.first().map(|col| col.values.len()).unwrap_or(0);

    let rows: Vec<Vec<Html>> = (0..num_rows)
        .map(|row_index| {
            columns
                .iter()
                .map(|col| col.values.get(row_index).cloned().unwrap_or_default())
                .collect()
        })
        .collect();

    let on_row_click = {
        let row_click_callback = row_click_callback.clone();
        Callback::from(move |index: usize| {
            if let Some(cb) = &row_click_callback {
                cb.emit(index);
            }
        })
    };

    /// Helper: Wrap raw text in Typo for dark mode support
    fn normalize_cell(cell: &Html) -> Html {
        match cell {
            Html::VText(text) => {
                html! { <Typo tag={TagType::Span}>{ text.text.clone() }</Typo> }
            }
            _ => cell.clone(),
        }
    }

    let desktop_table = html! {
        <table
            class="min-w-full bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-700 rounded-lg"
            aria-label={aria_label.clone()}
        >
            { caption.as_ref().map(|caption| html! {
                <caption class="px-4 py-3 text-left text-sm font-semibold text-gray-700 dark:text-gray-200">
                    { caption.clone() }
                </caption>
            }).unwrap_or_default() }

            <thead class="bg-gray-200 dark:bg-gray-700 text-gray-600 dark:text-gray-300">
                <tr>
                    { for headers.iter().enumerate().map(|(i, header)| html! {
                        <th key={i} class="py-2 px-4 text-left align-top border-b border-gray-300 dark:border-gray-600 break-words">
                            { header.clone() }
                        </th>
                    }) }
                </tr>
            </thead>

            <tbody>
                { for rows.iter().enumerate().map(|(index, row)| {
                    let clickable = row_click_callback.is_some();
                    let onclick = {
                        let on_row_click = on_row_click.clone();
                        Callback::from(move |_| on_row_click.emit(index))
                    };

                    let mut row_classes = vec![
                        if index % 2 == 0 {
                            "bg-gray-50 dark:bg-gray-800"
                        } else {
                            "bg-white dark:bg-gray-900"
                        },
                        "transition", "duration-150",
                    ];

                    if clickable {
                        row_classes.push("hover:bg-blue-50 dark:hover:bg-blue-700");
                        row_classes.push("cursor-pointer");
                    }

                    html! {
                        <tr key={index} class={classes!(row_classes)} onclick={if clickable { Some(onclick) } else { None }}>
                            { for row.iter().enumerate().map(|(col_idx, cell)| html! {
                                <td key={col_idx} class="py-2 px-4 align-top border-b border-gray-300 dark:border-gray-600 break-words">
                                    { normalize_cell(cell) }
                                </td>
                            }) }
                        </tr>
                    }
                }) }
            </tbody>
        </table>
    };

    let mobile_cards = html! {
        <div class="md:hidden space-y-3" role="list" aria-label={aria_label.clone()}>
            { caption.as_ref().map(|caption| html! {
                <div class="px-1 text-left text-sm font-semibold text-gray-700 dark:text-gray-200">
                    { caption.clone() }
                </div>
            }).unwrap_or_default() }

            { for rows.iter().enumerate().map(|(index, row)| {
                let clickable = row_click_callback.is_some();
                let onclick = {
                    let on_row_click = on_row_click.clone();
                    Callback::from(move |_| on_row_click.emit(index))
                };

                let mut card_classes = vec![
                    "rounded-lg",
                    "border",
                    "border-gray-200",
                    "dark:border-gray-700",
                    "bg-white",
                    "dark:bg-gray-900",
                    "shadow-sm",
                    "transition",
                    "duration-150",
                ];

                if clickable {
                    card_classes.push("hover:bg-blue-50");
                    card_classes.push("dark:hover:bg-blue-700");
                    card_classes.push("cursor-pointer");
                }

                html! {
                    <div key={index} class={classes!(card_classes)} onclick={if clickable { Some(onclick) } else { None }} role="listitem">
                        <div class="divide-y divide-gray-200 dark:divide-gray-700">
                            { for row.iter().enumerate().map(|(col_idx, cell)| {
                                let header = headers.get(col_idx).cloned().unwrap_or_default();

                                html! {
                                    <div key={col_idx} class="grid grid-cols-3 gap-x-3 gap-y-2 px-4 py-3 text-left">
                                        <div class="text-xs font-semibold text-gray-500 dark:text-gray-400 break-words">
                                            { normalize_cell(&header) }
                                        </div>
                                        <div class="col-span-2 text-sm text-gray-800 dark:text-gray-100 break-words">
                                            { normalize_cell(cell) }
                                        </div>
                                    </div>
                                }
                            }) }
                        </div>
                    </div>
                }
            }) }
        </div>
    };

    match mobile_layout {
        TableMobileLayout::Scroll => html! {
            <div class="overflow-x-auto p-4 rounded-lg bg-gray-50 dark:bg-gray-800 shadow-lg">
                { desktop_table }
            </div>
        },
        TableMobileLayout::Stacked => html! {
            <div class="p-4 rounded-lg bg-gray-50 dark:bg-gray-800 shadow-lg">
                { mobile_cards }
                <div class="hidden md:block overflow-x-auto">
                    { desktop_table }
                </div>
            </div>
        },
    }
}
