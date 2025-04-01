// src/organisms/table.rs

use crate::atoms::{TagType, Typo};
use yew::prelude::*;

/// Define a new structure for representing a column
#[derive(Clone, PartialEq)]
pub struct Column {
    pub header: String,      // Column header
    pub values: Vec<String>, // Values associated with the column
}

/// Define the properties for the Table component
#[derive(Properties, PartialEq, Clone)]
pub struct TableProps {
    pub columns: Vec<Column>, // List of columns
    #[prop_or_default]
    pub row_click_callback: Option<Callback<usize>>, // Optional row click callback
}

#[function_component(Table)]
pub fn table(props: &TableProps) -> Html {
    let TableProps {
        columns,
        row_click_callback,
    } = props.clone();

    // Derive headers and rows from the columns structure
    let headers: Vec<String> = columns.iter().map(|col| col.header.clone()).collect();

    // Transpose columns into rows based on values, assuming all columns have the same length
    let num_rows = if !columns.is_empty() {
        columns[0].values.len()
    } else {
        0
    };
    let rows: Vec<Vec<String>> = (0..num_rows)
        .map(|row_index| {
            columns
                .iter()
                .map(|col| col.values.get(row_index).unwrap_or(&"".to_string()).clone())
                .collect()
        })
        .collect();

    // Row click callback handling
    let on_row_click = {
        let row_click_callback = row_click_callback.clone();
        Callback::from(move |index: usize| {
            if let Some(callback) = row_click_callback.as_ref() {
                callback.emit(index);
            }
        })
    };

    html! {
        <div class="overflow-x-auto p-4 rounded-lg bg-gray-50 dark:bg-gray-800 shadow-lg">
            <table class="min-w-full bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-700 rounded-lg">
                // Table Header
                <thead class="bg-gray-200 dark:bg-gray-700 text-gray-600 dark:text-gray-300">
                    <tr>
                        { for headers.iter().map(|header| html! {
                            <th class="py-2 px-4 text-left border-b border-gray-300 dark:border-gray-600">
                                <Typo tag={TagType::P} class="font-semibold">{ header.to_string() }</Typo>
                            </th>
                        }) }
                    </tr>
                </thead>

                // Table Body
                <tbody>
                    { for rows.iter().enumerate().map(|(index, row)| {
                        let onclick = {
                            let on_row_click = on_row_click.clone();
                            Callback::from(move |_| on_row_click.emit(index))
                        };

                        // Conditional styling for even/odd rows and hover effects
                        let row_class = if index % 2 == 0 {
                            "bg-gray-50 dark:bg-gray-800 hover:bg-blue-50 dark:hover:bg-blue-700 cursor-pointer transition duration-150"
                        } else {
                            "bg-white dark:bg-gray-900 hover:bg-blue-50 dark:hover:bg-blue-700 cursor-pointer transition duration-150"
                        };

                        html! {
                            <tr class={row_class} onclick={onclick}>
                                { for row.iter().map(|cell| html! {
                                    <td class="py-2 px-4 border-b border-gray-300 dark:border-gray-600">
                                        <Typo tag={TagType::P}>{ cell.to_string() }</Typo>
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
