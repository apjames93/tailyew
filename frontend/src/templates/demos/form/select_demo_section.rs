use crate::templates::demos::DemoComponent;
use tailyew::form::{Select, SelectOption, SelectSize, StateDropdown};
use tailyew::organisms::table::Column;
use yew::prelude::*;

const USAGE_CODE: &str = r#"
let options = vec![
    SelectOption { label: "Option 1".into(), value: "1".into() },
    SelectOption { label: "Option 2".into(), value: "2".into() },
    SelectOption { label: "Option 3".into(), value: "3".into() },
];

html! {
    <>
        <Select
            id="demo-select"
            name="demo_select"
            label="Choose an option"
            options={options.clone()}
            default_value="2"
            helper_text="Submitted under demo_select."
            on_change={Some(Callback::from(|val| web_sys::console::log_1(&format!("Selected: {}", val).into())))}
        />

        <Select
            id="compact-select"
            label="Compact select"
            options={options}
            size={SelectSize::Small}
            helper_text="Compact controls still support helper and error copy."
            error={Some("Choose a valid option.")}
            aria_describedby={Some("compact-select-help")}
        />
    </>
}

html! {
    <StateDropdown
        id="state-select"
        name="shipping_state"
        default_value="CO"
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
    let team_options = vec![
        SelectOption {
            label: "Design".into(),
            value: "design".into(),
        },
        SelectOption {
            label: "Engineering".into(),
            value: "engineering".into(),
        },
        SelectOption {
            label: "Support".into(),
            value: "support".into(),
        },
    ];
    let selected_team = use_state(|| AttrValue::from("engineering"));

    let on_change = Callback::from(|val: String| {
        web_sys::console::log_1(&format!("Selected value: {}", val).into());
    });
    let on_team_change = {
        let selected_team = selected_team.clone();
        Callback::from(move |val: String| {
            selected_team.set(AttrValue::from(val));
        })
    };

    let base_select = html! {
        <Select
            id="demo-select"
            name="demo_select"
            label="Choose an option"
            options={options}
            default_value="2"
            helper_text="Submitted under demo_select."
            on_change={Some(on_change.clone())}
        />
    };
    let controlled_select = html! {
        <div class="space-y-2">
            <Select
                id="team-select"
                name="team"
                label="Controlled select"
                options={team_options.clone()}
                value={Some((*selected_team).clone())}
                on_change={Some(on_team_change)}
                aria_describedby={Some("team-select-help")}
            />
            <p id="team-select-help" class="text-xs text-gray-500 dark:text-gray-400">
                { "Controlled by parent state. Submitted under the team field name." }
            </p>
            <p class="text-xs font-medium text-gray-700 dark:text-gray-300">
                { format!("Selected team: {}", (*selected_team).to_string()) }
            </p>
        </div>
    };
    let compact_error_select = html! {
        <Select
            id="compact-select"
            label="Compact select with error"
            options={team_options.clone()}
            size={SelectSize::Small}
            helper_text="Compact controls still support helper and error copy."
            error={Some("Choose a valid option.")}
            aria_invalid={Some(true)}
            aria_describedby={Some("compact-select-help")}
        />
    };
    let hidden_label_select = html! {
        <div>
            <p id="hidden-label-select-help" class="mb-1 text-xs text-gray-500 dark:text-gray-400">
                { "The visible label is hidden, so aria_label names the control." }
            </p>
            <Select
                id="hidden-label-select"
                label="Hidden label select"
                options={team_options.clone()}
                default_value="support"
                visually_hidden_label={true}
                aria_label={Some("Choose a support queue")}
                aria_describedby={Some("hidden-label-select-help")}
            />
        </div>
    };
    let disabled_select = html! {
        <Select
            id="disabled-select"
            label="Disabled select"
            options={team_options}
            default_value="design"
            disabled={true}
        />
    };

    let state_dropdown = html! {
        <StateDropdown
            id="state-select"
            name="shipping_state"
            default_value="CO"
        />
    };

    let combined_example = html! {
        <div class="grid gap-6 text-left lg:grid-cols-2">
            <div class="space-y-3">
                <h3 class="text-sm font-semibold text-gray-900 dark:text-gray-100">{ "Basic select" }</h3>
                { base_select }
            </div>
            <div class="space-y-3">
                <h3 class="text-sm font-semibold text-gray-900 dark:text-gray-100">{ "Controlled value" }</h3>
                { controlled_select }
            </div>
            <div class="space-y-3">
                <h3 class="text-sm font-semibold text-gray-900 dark:text-gray-100">{ "Validation state" }</h3>
                { compact_error_select }
                <p id="compact-select-help" class="text-xs text-gray-500 dark:text-gray-400">
                    { "External error text sets aria-invalid unless explicitly overridden." }
                </p>
            </div>
            <div class="space-y-3">
                <h3 class="text-sm font-semibold text-gray-900 dark:text-gray-100">{ "Accessible hidden label" }</h3>
                { hidden_label_select }
            </div>
            <div class="space-y-3">
                <h3 class="text-sm font-semibold text-gray-900 dark:text-gray-100">{ "Disabled state" }</h3>
                { disabled_select }
            </div>
            <div class="space-y-3">
                <h3 class="text-sm font-semibold text-gray-900 dark:text-gray-100">{ "StateDropdown wrapper" }</h3>
                { state_dropdown }
            </div>
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
                "helper_text",
                "error",
                "aria_invalid",
                "aria_describedby",
                "required",
                "on_change",
                "on_blur",
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
                "Option<AttrValue>",
                "Option<bool>",
                "Option<AttrValue>",
                "bool",
                "Option<Callback<String>>",
                "Option<Callback<FocusEvent>>",
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
                "Optional helper copy below the select.",
                "External error message shown below the select.",
                "Overrides computed aria-invalid state.",
                "ARIA describedby reference. Helper and error IDs are added automatically.",
                "Marks the field as required.",
                "Callback fired when the selected value changes.",
                "Called when the select loses focus.",
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
                    <p>{"The `Select` component renders a styled dropdown with submitted-name support, uncontrolled or controlled values, native required validation, external error state, and accessible hidden-label usage."}</p>
                    <p class="mt-2">{"Use `StateDropdown` when you need a U.S. state selector with the options already provided."}</p>
                </>
            })}
            example={combined_example}
            usage_code={USAGE_CODE}
            props_table={Some(props_table)}
        />
    }
}
