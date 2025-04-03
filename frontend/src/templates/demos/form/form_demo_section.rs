use crate::templates::demos::DemoComponent;
use tailyew::atoms::{TagType, Typo};
use tailyew::form::*;
use tailyew::organisms::table::Column;
use web_sys::SubmitEvent;
use yew::prelude::*;

const USAGE_CODE: &str = r#"
let form_values = use_state(|| "".to_string());

let onsubmit_callback = {
    let form_values = form_values.clone();
    Callback::from(move |e: SubmitEvent| {
        e.prevent_default();

        let mut values = String::new();
        let fields = vec![
            "email", "password", "search", "color", "range", "date", "age",
            "time", "textarea", "select", "gender", "file_upload", "phone",
        ];

        for field in fields {
            let value = e_input_value(field, &e);
            values.push_str(&format!("{}: {}\n", field, value));
        }

        let checked = e_checkbox_checked("checkbox", &e);
        values.push_str(&format!("checkbox: {}\n", checked));

        form_values.set(values);
    })
};

html! {
    <Form onsubmit_callback={onsubmit_callback}>
        <Input id="email" label="Email" input_type={InputType::Email} placeholder="Enter email" />
        // ... other inputs ...
        <Checkbox id="checkbox" label="Accept Terms" />
    </Form>
};
"#;

#[function_component(FormDemoSection)]
pub fn form_demo_section() -> Html {
    let form_values = use_state(|| "".to_string());

    let onsubmit_callback = {
        let form_values = form_values.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let mut values = String::new();
            let fields = vec![
                "email",
                "password",
                "search",
                "color",
                "range",
                "date",
                "age",
                "time",
                "textarea",
                "select",
                "gender",
                "file_upload",
                "phone",
            ];

            for field in fields {
                let value = e_input_value(field, &e);
                values.push_str(&format!("{}: {}\n", field, value));
            }

            let checked = e_checkbox_checked("checkbox", &e);
            values.push_str(&format!("checkbox: {}\n", checked));

            form_values.set(values);
        })
    };

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

    let radio_options = vec![
        ("male".to_string(), "Male".to_string()),
        ("female".to_string(), "Female".to_string()),
        ("other".to_string(), "Other".to_string()),
    ];

    let example = html! {
        <section class="max-w-4xl mx-auto p-6 bg-white dark:bg-gray-800 rounded-lg shadow-lg space-y-6">
            <Typo tag={TagType::H1}>{ "Form Component Demo" }</Typo>

            <Form onsubmit_callback={onsubmit_callback}>
                <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                    <Input id="email" label="Email" input_type={InputType::Email} placeholder="Enter email" />
                    <Input id="password" label="Password" input_type={InputType::Password} placeholder="Enter password" />
                    <Input id="search" label="Search" input_type={InputType::Search} placeholder="Search..." />
                    <ColorInput id="color" label="Pick a Color" value={"#00ffcc".to_string()} />
                    <RangeInput id="range" label="Volume" min="0" max="100" step="5" default_value="50" />
                    <Input id="date" label="Date of Birth" input_type={InputType::Date} placeholder="Select a date" />
                    <Input id="age" label="Age" input_type={InputType::Number} placeholder="Enter age" />
                    <Input id="time" label="Time" input_type={InputType::Time} placeholder="Select time" />
                </div>

                <Textarea id="textarea" label="Description" placeholder="Write something..." />
                <Select id="select" options={options} default_value={"2".to_string()} />
                <RadioGroup id="gender" label="Select Gender" options={radio_options} default_value={"female".to_string()} />
                <FileInput id="file_upload" label="Upload File" />
                <Checkbox id="checkbox" label="Accept Terms" />
                <PhoneInput id="phone" label="Phone Number" placeholder="123-456-7890" />

                <div class="mt-6 text-sm text-gray-600 whitespace-pre-wrap dark:text-gray-300">
                    { (*form_values).clone() }
                </div>
            </Form>
        </section>
    };

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "children",
                "onsubmit_callback",
                "form_class",
                "button_label",
                "show_submit_button",
                "loading",
                "id",
                "error_message",
                "success_message",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
        },
        Column {
            header: "Type".into(),
            values: vec![
                "Children",
                "Callback<SubmitEvent>",
                "Classes",
                "String",
                "bool",
                "bool",
                "Option<String>",
                "Option<String>",
                "Option<String>",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
        },
        Column {
            header: "Description".into(),
            values: vec![
                "Form content and layout.",
                "Fires on form submit.",
                "Custom class for the <form> element.",
                "Submit button text.",
                "Controls visibility of the submit button.",
                "Disables button when loading.",
                "Optional form ID.",
                "Error banner message.",
                "Success banner message.",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
        },
    ];

    html! {
        <DemoComponent
            title="Form Component"
            description={Some(html! {
                <p>{"The `Form` component wraps children and handles validation, layout, and submit logic. Demonstrated here with every form input component."}</p>
            })}
            example={example}
            usage_code={USAGE_CODE}
            props_table={Some(props_table)}
        />
    }
}
