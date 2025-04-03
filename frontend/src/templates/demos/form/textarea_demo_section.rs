use crate::templates::demos::DemoComponent;
use tailyew::form::Textarea;
use tailyew::organisms::table::Column;
use yew::prelude::*;

const USAGE_CODE: &str = r#"
let value = use_state(|| String::new());
let on_change = {
    let value = value.clone();
    Callback::from(move |val: String| value.set(val))
};

html! {
    <Textarea
        id="message"
        label="Your Message"
        placeholder="Type your message here..."
        default_value=""
        rows={6}
        on_change={Some(on_change)}
    />
}
"#;

#[function_component(TextareaDemoSection)]
pub fn textarea_demo_section() -> Html {
    let state = use_state(String::new);
    let on_change = {
        let state = state.clone();
        Callback::from(move |val: String| state.set(val))
    };

    let example = html! {
        <Textarea
            id="message"
            label="Your Message"
            placeholder="Type your message here..."
            default_value=""
            rows={6}
            on_change={Some(on_change)}
        />
    };

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "id".into(),
                "label".into(),
                "default_value".into(),
                "placeholder".into(),
                "class".into(),
                "container_class".into(),
                "required".into(),
                "on_change".into(),
                "rows".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "String".into(),
                "String".into(),
                "String".into(),
                "String".into(),
                "Option<String>".into(),
                "Option<String>".into(),
                "bool".into(),
                "Option<Callback<String>>".into(),
                "usize".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "ID and name for the textarea.".into(),
                "Visible label text.".into(),
                "Initial value of the textarea.".into(),
                "Placeholder text when empty.".into(),
                "Optional Tailwind classes for the input.".into(),
                "Optional Tailwind classes for the container.".into(),
                "Whether the input is required.".into(),
                "Callback called on input value change.".into(),
                "Number of visible text rows.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            title="Textarea Component"
            description={Some(html! {
                <p>{"The `Textarea` component is a styled multiline input field with full Tailwind support. Ideal for comments, messages, and larger text blocks."}</p>
            })}
            example={example}
            usage_code={USAGE_CODE}
            props_table={Some(props_table)}
        />
    }
}
