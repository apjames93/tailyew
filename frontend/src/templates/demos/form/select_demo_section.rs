use crate::templates::demos::DemoComponent;
use tailyew::form::{Select, SelectOption, StateDropdown};
use tailyew::organisms::table::Column;
use yew::prelude::*;

const USAGE_CODE: &str = r#"
let options = vec![
    SelectOption { label: "Option 1".into(), value: "1".into() },
    SelectOption { label: "Option 2".into(), value: "2".into() },
    SelectOption { label: "Option 3".into(), value: "3".into() },
];

html! {
    <Select
        id="demo-select"
        label={"Choose an option"}
        options={options}
        default_value={"2"}
        on_change={Some(Callback::from(|val| web_sys::console::log_1(&format!("Selected: {}", val).into())))}
    />
}

html! {
    <StateDropdown
        id="state-select"
        default_value="CO".to_string()
    />
}
"#;

#[function_component(SelectDemoSection)]
pub fn select_demo_section() -> Html {
    let options = vec![
        SelectOption {
            label: "Option 1".into(),
            value: "1".into(),
        },
        SelectOption {
            label: "Option 2".into(),
            value: "2".into(),
        },
        SelectOption {
            label: "Option 3".into(),
            value: "3".into(),
        },
    ];

    let on_change = Callback::from(|val: String| {
        web_sys::console::log_1(&format!("Selected value: {}", val).into());
    });

    let base_select = html! {
        <Select
            id="demo-select"
            label={"Choose an option"}
            options={options}
            default_value={"2"}
            on_change={Some(on_change.clone())}
        />
    };

    let state_dropdown = html! {
        <StateDropdown
            id={"state-select".to_string()}
            default_value={"CO".to_string()}
        />
    };

    let combined_example = html! {
        <div class="space-y-6">
            { base_select }
            { state_dropdown }
        </div>
    };

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "id".into(),
                "options".into(),
                "default_value".into(),
                "label".into(),
                "class".into(),
                "required".into(),
                "on_change".into(),
                "disabled".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "String".into(),
                "Vec<SelectOption>".into(),
                "String".into(),
                "Option<String>".into(),
                "Option<String>".into(),
                "bool".into(),
                "Option<Callback<String>>".into(),
                "bool".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "Unique ID for the select element.".into(),
                "List of options with `label` and `value`.".into(),
                "Initial selected value.".into(),
                "Optional label rendered above the select.".into(),
                "Optional Tailwind class overrides.".into(),
                "Marks the field as required.".into(),
                "Callback fired when the selected value changes.".into(),
                "Disables the select if true.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            title="Select Component"
            description={Some(html! {
                <>
                    <p>{"The `Select` component renders a styled dropdown with full control over label, default value, and change handling. Integrates well with forms."}</p>
                    <p class="mt-2">{"You can also use our `StateDropdown` for selecting U.S. states with a pre-filled list of 50 state options."}</p>
                </>
            })}
            example={combined_example}
            usage_code={USAGE_CODE}
            props_table={Some(props_table)}
        />
    }
}
