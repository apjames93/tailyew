use crate::templates::demos::DemoComponent;
use tailyew::form::RadioGroup;
use tailyew::organisms::table::Column;
use yew::prelude::*;

const USAGE_CODE: &str = r#"
html! {
    <RadioGroup
        id="favorite-color"
        name="favorite_color"
        label="Favorite Color"
        default_value="green".to_string()
        helper_text="Choose one color."
        options={vec![
            ("red".to_string(), "Red".to_string()),
            ("green".to_string(), "Green".to_string()),
            ("blue".to_string(), "Blue".to_string()),
        ]}
        on_change={Some(Callback::from(|val| web_sys::console::log_1(&format!("Selected: {}", val).into())))}
    />
}
"#;

#[component(RadioGroupDemoSection)]
pub fn radio_group_demo_section() -> Html {
    let options = vec![
        ("red".into(), "Red".into()),
        ("green".into(), "Green".into()),
        ("blue".into(), "Blue".into()),
    ];

    let on_change = Callback::from(|val: String| {
        web_sys::console::log_1(&format!("Selected value: {}", val).into());
    });

    let example = html! {
        <RadioGroup
            id="favorite-color"
            name="favorite_color"
            label="Favorite Color"
            options={options}
            default_value={"green".to_string()}
            helper_text="Choose one color."
            on_change={Some(on_change)}
        />
    };

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "id".into(),
                "name".into(),
                "label".into(),
                "options".into(),
                "default_value".into(),
                "value".into(),
                "helper_text".into(),
                "error".into(),
                "aria_invalid".into(),
                "aria_describedby".into(),
                "required".into(),
                "disabled".into(),
                "class".into(),
                "on_change".into(),
                "on_blur".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "AttrValue".into(),
                "Option<AttrValue>".into(),
                "AttrValue".into(),
                "Vec<(String, String)>".into(),
                "AttrValue".into(),
                "Option<AttrValue>".into(),
                "Option<AttrValue>".into(),
                "Option<AttrValue>".into(),
                "Option<bool>".into(),
                "Option<AttrValue>".into(),
                "bool".into(),
                "bool".into(),
                "Classes".into(),
                "Option<Callback<String>>".into(),
                "Option<Callback<FocusEvent>>".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "DOM/accessibility ID for the radio group.".into(),
                "Submitted form field name shared by all options. Defaults to id.".into(),
                "Label displayed above the radio group.".into(),
                "A list of (value, label) pairs for each option.".into(),
                "The initially selected value.".into(),
                "Controlled selected value for parent-owned state.".into(),
                "Optional helper copy below the group.".into(),
                "External error message shown below the group.".into(),
                "Overrides computed aria-invalid state.".into(),
                "Additional aria-describedby IDs.".into(),
                "Requires one selected option.".into(),
                "Disables all radio options.".into(),
                "Optional Tailwind classes for custom styling.".into(),
                "Callback fired when the selected value changes.".into(),
                "Called when an option loses focus.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            github_demo_path="form/radio_group_demo_section.rs"
            github_source_path="form/radio_group.rs"
            title="RadioGroup Component"
            description={Some(html! {
                <p>{"The `RadioGroup` component renders a set of radio buttons from a list of options. It supports controlled state and an optional `on_change` callback."}</p>
            })}
            example={example}
            usage_code={USAGE_CODE}
            props_table={Some(props_table)}
        />
    }
}
