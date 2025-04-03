use crate::templates::demos::DemoComponent;
use tailyew::atoms::{TagType, Typo};
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
            header: "Name".into(),
            values: vec!["Alice".into(), "Bob".into(), "Charlie".into()],
        },
        Column {
            header: "Role".into(),
            values: vec!["Admin".into(), "Editor".into(), "Viewer".into()],
        },
        Column {
            header: "Status".into(),
            values: vec!["Active".into(), "Inactive".into(), "Active".into()],
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
        header: "Name".into(),
        values: vec!["Alice".into(), "Bob".into(), "Charlie".into()],
    },
    Column {
        header: "Role".into(),
        values: vec!["Admin".into(), "Editor".into(), "Viewer".into()],
    },
    Column {
        header: "Status".into(),
        values: vec!["Active".into(), "Inactive".into(), "Active".into()],
    },
];

<Table columns={columns} row_click_callback={Some(on_row_click)} />
"#;

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec!["columns".into(), "row_click_callback".into()],
        },
        Column {
            header: "Type".into(),
            values: vec!["Vec<Column>".into(), "Option<Callback<usize>>".into()],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "Each column with header and its row values.".into(),
                "Optional click handler when a row is clicked.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            title="Table Component"
            description={Some(html! {
                <p>{"The `Table` component displays structured data in rows and columns. Supports row clicks and is styled for both light and dark modes."}</p>
            })}
            example={example}
            usage_code={usage_code}
            props_table={Some(props_table)}
        />
    }
}
