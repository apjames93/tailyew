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
        name="message"
        label="Your Message"
        placeholder="Type your message here..."
        default_value=""
        helper_text="Use a few sentences if helpful."
        rows={6}
        on_change={Some(on_change)}
    />
}
"#;

#[component(TextareaDemoSection)]
pub fn textarea_demo_section() -> Html {
    let state = use_state(String::new);
    let on_change = {
        let state = state.clone();
        Callback::from(move |val: String| state.set(val))
    };

    let example = html! {
        <Textarea
            id="message"
            name="message"
            label="Your Message"
            placeholder="Type your message here..."
            default_value=""
            helper_text="Use a few sentences if helpful."
            rows={6}
            on_change={Some(on_change)}
        />
    };

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "id",
                "name",
                "label",
                "default_value",
                "value",
                "placeholder",
                "class",
                "container_class",
                "label_class",
                "visually_hidden_label",
                "helper_text",
                "error",
                "aria_invalid",
                "aria_describedby",
                "aria_label",
                "required",
                "disabled",
                "on_change",
                "on_blur",
                "rows",
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
                "AttrValue",
                "AttrValue",
                "Option<AttrValue>",
                "AttrValue",
                "Classes",
                "Classes",
                "Classes",
                "bool",
                "Option<AttrValue>",
                "Option<AttrValue>",
                "Option<bool>",
                "Option<AttrValue>",
                "Option<AttrValue>",
                "bool",
                "bool",
                "Option<Callback<String>>",
                "Option<Callback<FocusEvent>>",
                "usize",
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
                "Visible label text.",
                "Initial uncontrolled value.",
                "Controlled value for parent-owned textarea state.",
                "Placeholder text when empty.",
                "Additional Tailwind classes for the textarea.",
                "Additional Tailwind classes for the wrapper.",
                "Additional Tailwind classes for the label.",
                "Hides the label visually while preserving it for screen readers.",
                "Optional helper copy below the textarea.",
                "External error message shown below the textarea.",
                "Overrides computed aria-invalid state.",
                "Additional aria-describedby IDs.",
                "Accessible label for hidden-label contexts.",
                "Whether the textarea is required.",
                "Disables the textarea.",
                "Callback called on input value change.",
                "Called when the textarea loses focus.",
                "Number of visible text rows.",
            ]
            .into_iter()
            .map(Html::from)
            .collect(),
        },
    ];

    html! {
        <DemoComponent
            github_demo_path="form/textarea_demo_section.rs"
            github_source_path="form/textarea.rs"
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
