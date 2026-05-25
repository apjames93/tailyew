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
        name="demo_select"
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

#[component(SelectDemoSection)]
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
            name="demo_select"
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
                "id",
                "name",
                "options",
                "default_value",
                "value",
                "size",
                "label",
                "class",
                "container_class",
                "label_class",
                "visually_hidden_label",
                "aria_label",
                "error",
                "aria_invalid",
                "required",
                "on_change",
                "disabled",
            ]
            .into_iter()
            .map(Html::from)
            .collect(),
        },
        Column {
            header: "Type".into(),
            values: vec![
                "AttrValue",
                "Option<AttrValue>",
                "Vec<SelectOption>",
                "AttrValue",
                "Option<AttrValue>",
                "SelectSize",
                "AttrValue",
                "Classes",
                "Classes",
                "Classes",
                "bool",
                "Option<AttrValue>",
                "Option<AttrValue>",
                "Option<bool>",
                "bool",
                "Option<Callback<String>>",
                "bool",
            ]
            .into_iter()
            .map(Html::from)
            .collect(),
        },
        Column {
            header: "Description".into(),
            values: vec![
                "DOM/accessibility ID.",
                "Submitted form field name. Defaults to id.",
                "List of options with label and value.",
                "Initial uncontrolled selected value.",
                "Controlled selected value.",
                "Visual density for the select control.",
                "Label rendered above the select.",
                "Additional Tailwind classes for the select.",
                "Additional Tailwind classes for the wrapper.",
                "Additional Tailwind classes for the label.",
                "Hides the label visually while preserving it for screen readers.",
                "Accessible label for hidden-label contexts.",
                "External error message shown below the select.",
                "Overrides computed aria-invalid state.",
                "Marks the field as required.",
                "Callback fired when the selected value changes.",
                "Disables the select if true.",
            ]
            .into_iter()
            .map(Html::from)
            .collect(),
        },
    ];

    html! {
        <DemoComponent
            github_demo_path="form/select_demo_section.rs"
            github_source_path="form/select.rs"
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
