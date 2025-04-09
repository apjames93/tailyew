use crate::templates::demos::DemoComponent;
use tailyew::atoms::ButtonType;
use tailyew::molecules::CopyToClipboard;
use tailyew::organisms::table::Column;
use yew::prelude::*;

#[function_component(CopyToClipboardDemoSection)]
pub fn copy_to_clipboard_demo_section() -> Html {
    let example = html! {
        <div class="max-w-xl space-y-4">
            <CopyToClipboard
                value="abc123xyz456token789"
            />
            <CopyToClipboard
                value="this was hidden"
                button_type={ButtonType::Secondary}
                copied_button_type={ButtonType::Danger}
                copy_text="Tap to copy"
                copied_text="Nice!"
            />
        </div>
    };

    let usage_code = r#"
<CopyToClipboard
    value="abc123xyz456token789"
    button_type={ButtonType::Ghost}
    copied_button_type={ButtonType::Success}
    copy_text="Copy token"
    copied_text="Copied!"
/>
"#;

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "value".into(),
                "copy_text".into(),
                "copied_text".into(),
                "button_type".into(),
                "copied_button_type".into(),
                "class".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "AttrValue".into(),
                "String".into(),
                "String".into(),
                "ButtonType".into(),
                "ButtonType".into(),
                "Classes".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "The value to be copied to clipboard.".into(),
                "Label for the copy button before copying.".into(),
                "Label for the copy button after copying.".into(),
                "Button style before copying.".into(),
                "Button style after copying.".into(),
                "Extra Tailwind classes for button layout or spacing.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            title="CopyToClipboard Component"
            description={Some(html! {
                <p>{"The `CopyToClipboard` component renders a customizable copy button that copies a value to the clipboard. Useful for sharing tokens, codes, or references."}</p>
            })}
            example={example}
            usage_code={usage_code}
            props_table={Some(props_table)}
        />
    }
}
