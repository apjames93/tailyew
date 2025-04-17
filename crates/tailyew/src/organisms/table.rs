use crate::atoms::{TagType, Typo};
use yew::prelude::*;

/// A flexible column definition supporting arbitrary Yew HTML for headers and cells
#[derive(Clone, PartialEq)]
pub struct Column {
    pub header: Html,
    pub values: Vec<Html>,
}

/// Properties for the Table component
#[derive(Properties, PartialEq, Clone)]
pub struct TableProps {
    pub columns: Vec<Column>,
    #[prop_or_default]
    pub row_click_callback: Option<Callback<usize>>,
}

#[function_component(Table)]
pub fn table(props: &TableProps) -> Html {
    let TableProps {
        columns,
        row_click_callback,
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

    html! {
        <div class="overflow-x-auto p-4 rounded-lg bg-gray-50 dark:bg-gray-800 shadow-lg">
            <table class="min-w-full bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-700 rounded-lg">
                <thead class="bg-gray-200 dark:bg-gray-700 text-gray-600 dark:text-gray-300">
                    <tr>
                        { for headers.iter().enumerate().map(|(i, header)| html! {
                            <th key={i} class="py-2 px-4 text-left border-b border-gray-300 dark:border-gray-600">
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
                                    <td key={col_idx} class="py-2 px-4 border-b border-gray-300 dark:border-gray-600">
                                        { cell.clone() }
                                    </td>
                                }) }
                            </tr>
                        }
                    }) }
                </tbody>
            </table>
        </div>
    }
}
