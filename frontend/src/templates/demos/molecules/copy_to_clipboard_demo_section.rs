use crate::templates::demos::DemoComponent;
use tailyew::atoms::ButtonType;
use tailyew::icons::CopyIcon;
use tailyew::molecules::CopyToClipboard;
use tailyew::organisms::table::Column;
use yew::prelude::*;

#[component(CopyToClipboardDemoSection)]
pub fn copy_to_clipboard_demo_section() -> Html {
    let example = html! {
        <div class="max-w-xl space-y-4">
            <CopyToClipboard value="abc123xyz456token789" />

            <CopyToClipboard
                value="this-was-hidden"
                button_type={ButtonType::Secondary}
                copied_button_type={ButtonType::Danger}
                copy_text="Tap to copy"
                copied_text="Nice!"
            />

            <CopyToClipboard
                value="icon-only-example"
                button_type={ButtonType::Ghost}
                copied_button_type={ButtonType::Ghost}
                class="p-1 hover:bg-gray-200 dark:hover:bg-gray-700 rounded"
            >
                <CopyIcon size={16} />
            </CopyToClipboard>
        </div>
    };

    let usage_code = r#"
<CopyToClipboard value="abc123xyz456token789" />

<CopyToClipboard
    value="this-was-hidden"
    button_type={ButtonType::Secondary}
    copied_button_type={ButtonType::Danger}
    copy_text="Tap to copy"
    copied_text="Nice!"
/>

<CopyToClipboard
    value="icon-only-example"
    button_type={ButtonType::Ghost}
    copied_button_type={ButtonType::Ghost}
    class="p-1 hover:bg-gray-200 dark:hover:bg-gray-700 rounded"
>
    <CopyIcon size={16} />
</CopyToClipboard>
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
                "children".into(),
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
                "Children".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "The value to be copied to clipboard.".into(),
                "Label shown before copy (used if `children` is empty).".into(),
                "Label shown after copy (used if `children` is empty).".into(),
                "Button style before copy.".into(),
                "Button style after copy.".into(),
                "Extra Tailwind classes for layout/styling.".into(),
                "Optional custom content (e.g., icon) to override label.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            github_demo_path="molecules/copy_to_clipboard_demo_section.rs"
            github_source_path="molecules/copy_to_clipboard.rs"
            title="CopyToClipboard Component"
            description={Some(html! {
                <p>{"The `CopyToClipboard` component renders a button that copies text to the clipboard. It supports custom labels, icons, styles, and copy feedback."}</p>
            })}
            example={example}
            usage_code={usage_code}
            props_table={Some(props_table)}
        />
    }
}
