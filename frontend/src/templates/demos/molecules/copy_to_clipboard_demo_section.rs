use crate::templates::demos::DemoComponent;
use tailyew::molecules::CopyToClipboard;
use tailyew::organisms::table::Column;
use yew::prelude::*;

#[function_component(CopyToClipboardDemoSection)]
pub fn copy_to_clipboard_demo_section() -> Html {
    let example = html! {
        <div class="max-w-xl">
            <CopyToClipboard
                label="API Token"
                value={"abc123xyz456token789".to_string()}
            />
        </div>
    };

    let usage_code = r#"
<CopyToClipboard
    label="API Token"
    value="abc123xyz456token789".to_string()
/>
"#;

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec!["label".into(), "value".into()],
        },
        Column {
            header: "Type".into(),
            values: vec!["String".into(), "String".into()],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "The label shown above the input field.".into(),
                "The value to be copied to clipboard.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            title="CopyToClipboard Component"
            description={Some(html! {
                <p>{"The `CopyToClipboard` component renders a read-only text input with a copy button. Ideal for exposing API tokens or keys to users."}</p>
            })}
            example={example}
            usage_code={usage_code}
            props_table={Some(props_table)}
        />
    }
}
