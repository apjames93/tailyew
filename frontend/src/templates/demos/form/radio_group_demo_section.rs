use crate::templates::demos::DemoComponent;
use tailyew::form::RadioGroup;
use tailyew::organisms::table::Column;
use yew::prelude::*;

const USAGE_CODE: &str = r#"
html! {
    <RadioGroup
        id="favorite-color"
        label="Favorite Color"
        default_value="green".to_string()
        options={vec![
            ("red".to_string(), "Red".to_string()),
            ("green".to_string(), "Green".to_string()),
            ("blue".to_string(), "Blue".to_string()),
        ]}
        on_change={Some(Callback::from(|val| web_sys::console::log_1(&format!("Selected: {}", val).into())))}
    />
}
"#;

#[function_component(RadioGroupDemoSection)]
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
            label="Favorite Color"
            options={options}
            default_value={"green".to_string()}
            on_change={Some(on_change)}
        />
    };

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "id".into(),
                "label".into(),
                "options".into(),
                "default_value".into(),
                "class".into(),
                "on_change".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "String".into(),
                "String".into(),
                "Vec<(String, String)>".into(),
                "String".into(),
                "Classes".into(),
                "Option<Callback<String>>".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "The unique ID and name used for all radio inputs.".into(),
                "Label displayed above the radio group.".into(),
                "A list of (value, label) pairs for each option.".into(),
                "The initially selected value.".into(),
                "Optional Tailwind classes for custom styling.".into(),
                "Callback fired when the selected value changes.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
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
