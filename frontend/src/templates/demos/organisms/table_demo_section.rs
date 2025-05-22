use crate::templates::demos::DemoComponent;
use tailyew::atoms::{Button, ButtonType, TagType, Typo};
use tailyew::form::Checkbox;
use tailyew::organisms::table::{Column, Table};
use yew::prelude::*;

#[function_component(TableDemoSection)]
pub fn table_demo_section() -> Html {
    let clicked_row = use_state(|| None::<usize>);

    let on_row_click = {
        let clicked_row = clicked_row.clone();
        Callback::from(move |index: usize| {
            web_sys::console::log_1(&format!("Row clicked: {}", index).into());
            clicked_row.set(Some(index));
        })
    };

    let columns = vec![
        Column {
            header: html! { "Name" },
            values: vec![html! { "Alice" }, html! { "Bob" }, html! { "Charlie" }],
        },
        Column {
            header: html! { "Status" },
            values: vec![
                html! { <Checkbox id="chk1" label="Active" checked={true} /> },
                html! { <Checkbox id="chk2" label="Inactive" checked={false} /> },
                html! { <Checkbox id="chk3" label="Pending" checked={true} /> },
            ],
        },
        Column {
            header: html! { "Actions" },
            values: vec![
                html! { <Button button_type={ButtonType::Primary}>{ "Edit" }</Button> },
                html! { <Button button_type={ButtonType::Secondary}>{ "View" }</Button> },
                html! { <Button button_type={ButtonType::Danger}>{ "Delete" }</Button> },
            ],
        },
    ];

    let example = html! {
        <div class="space-y-6">
            <Typo tag={TagType::H1}>{ "Table Component Demo" }</Typo>
            <Table columns={columns.clone()} row_click_callback={Some(on_row_click.clone())} />

            if let Some(index) = *clicked_row {
                <div class="mt-4 text-sm text-gray-800 dark:text-gray-300">
                    { format!("Clicked row index: {}", index) }
                </div>
            }
        </div>
    };

    let usage_code = r#"
let columns = vec![
    Column {
        header: html! { "Name" },
        values: vec![
            html! { "Alice" },
            html! { "Bob" },
            html! { "Charlie" },
        ],
    },
    Column {
        header: html! { "Status" },
        values: vec![
            html! { <Checkbox id="chk1" label="Active" checked={true} /> },
            html! { <Checkbox id="chk2" label="Inactive" checked={false} /> },
            html! { <Checkbox id="chk3" label="Pending" checked={true} /> },
        ],
    },
    Column {
        header: html! { "Actions" },
        values: vec![
            html! { <Button button_type={ButtonType::Primary}>{ "Edit" }</Button> },
            html! { <Button button_type={ButtonType::Secondary}>{ "View" }</Button> },
            html! { <Button button_type={ButtonType::Danger}>{ "Delete" }</Button> },
        ],
    },
];
"#;

    let props_table = vec![
        Column {
            header: html! { "Prop" },
            values: vec![html! { "columns" }, html! { "row_click_callback" }],
        },
        Column {
            header: html! { "Type" },
            values: vec![html! { "Vec<Column>" }, html! { "Option<Callback<usize>>" }],
        },
        Column {
            header: html! { "Description" },
            values: vec![
                html! { "Each column with header and its row values." },
                html! { "Optional click handler when a row is clicked." },
            ],
        },
    ];

    html! {
        <DemoComponent
            github_demo_path="organisms/table_demo_section.rs"
            github_source_path="organisms/table.rs"
            title="Table Component"
            description={Some(html! {
                <Typo tag={TagType::P}>
                    {"The `Table` component displays structured data in rows and columns. Supports plain text, rich HTML (buttons, checkboxes), and row click handlers. Fully styled for light and dark modes."}
                </Typo>
            })}
            example={example}
            usage_code={usage_code}
            props_table={Some(props_table)}
        />
    }
}
