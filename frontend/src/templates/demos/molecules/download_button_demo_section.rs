use crate::templates::demos::DemoComponent;
use serde_json::json;
use tailyew::atoms::ButtonType;
use tailyew::molecules::download_button::{DownloadSource, FileType};
use tailyew::molecules::DownloadButton;
use tailyew::organisms::table::Column;
use yew::prelude::*;

#[function_component(DownloadButtonDemoSection)]
pub fn download_button_demo_section() -> Html {
    let json_data = json!({
        "project": "TailYew",
        "components": ["Button", "Modal", "Table"],
        "open_source": true,
    })
    .to_string();

    let csv_data = "name,role\nAlex,Engineer\nSam,Designer".to_string();
    let txt_data = "Thanks for using TailYew!".to_string();
    let md_data = "# TailYew\n\nReusable components powered by Yew + Tailwind.".to_string();

    let example = html! {
        <div class="max-w-xl space-y-4">
            <DownloadButton
                source={DownloadSource::Url("https://www.w3.org/WAI/ER/tests/xhtml/testfiles/resources/pdf/dummy.pdf".into())}
                filename="sample.pdf"
                filetype={FileType::Pdf}
                label="Download Sample PDF"
                button_type={ButtonType::Primary}
            />

            <DownloadButton
                source={DownloadSource::Json(json_data.clone())}
                filename="config.json"
                filetype={FileType::Json}
                label="Download JSON Config"
                button_type={ButtonType::Secondary}
            />

            <DownloadButton
                source={DownloadSource::Json(csv_data)}
                filename="data.csv"
                filetype={FileType::Csv}
                label="Download CSV Data"
                button_type={ButtonType::Primary}
            />

            <DownloadButton
                source={DownloadSource::Json(txt_data)}
                filename="readme.txt"
                filetype={FileType::Txt}
                label="Download Text File"
                button_type={ButtonType::Secondary}
            />

            <DownloadButton
                source={DownloadSource::Json(md_data)}
                filename="readme.md"
                filetype={FileType::Markdown}
                label="Download Markdown"
                button_type={ButtonType::Ghost}
            />
        </div>
    };

    let usage_code = r#"
<DownloadButton
    source={DownloadSource::Json(json_data.clone())}
    filename="config.json"
    filetype={FileType::Json}
    label="Download JSON Config"
    button_type={ButtonType::Secondary}
/>
"#;

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "source".into(),
                "filename".into(),
                "filetype".into(),
                "label".into(),
                "button_type".into(),
                "class".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "DownloadSource".into(),
                "AttrValue".into(),
                "FileType".into(),
                "String".into(),
                "ButtonType".into(),
                "Classes".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "The source content to download (external URL or JSON blob).".into(),
                "The name to save the file as, including extension.".into(),
                "The file MIME type, inferred from enum variants (e.g., Csv, Json, Pdf).".into(),
                "Text to show inside the button.".into(),
                "TailYew button variant.".into(),
                "Additional Tailwind utility classes.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            title="DownloadButton Component"
            description={Some(html! {
                <p>{"The `DownloadButton` allows users to download content from a given URL or dynamically generated JSON. It supports multiple file formats using the `FileType` enum."}</p>
            })}
            example={example}
            usage_code={usage_code}
            props_table={Some(props_table)}
        />
    }
}
