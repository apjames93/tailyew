use gloo::file::{Blob, ObjectUrl};
use wasm_bindgen::JsCast;
use web_sys::HtmlAnchorElement;
use yew::prelude::*;

use crate::atoms::{Button, ButtonType};

#[derive(PartialEq, Clone)]
pub enum FileType {
    Json,
    Csv,
    Txt,
    Html,
    Xml,
    Pdf,
    Zip,
    Binary,
    Markdown,
    Yaml,
    Custom(String),
}

impl FileType {
    pub fn mime_type(&self) -> String {
        match self {
            FileType::Json => "application/json;charset=utf-8".to_string(),
            FileType::Csv => "text/csv;charset=utf-8".to_string(),
            FileType::Txt => "text/plain;charset=utf-8".to_string(),
            FileType::Html => "text/html;charset=utf-8".to_string(),
            FileType::Xml => "application/xml".to_string(),
            FileType::Pdf => "application/pdf".to_string(),
            FileType::Zip => "application/zip".to_string(),
            FileType::Binary => "application/octet-stream".to_string(),
            FileType::Markdown => "text/markdown;charset=utf-8".to_string(),
            FileType::Yaml => "text/yaml;charset=utf-8".to_string(),
            FileType::Custom(custom) => custom.clone(),
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum DownloadSource {
    Url(AttrValue),
    Json(String),
}

#[derive(Properties, PartialEq, Clone)]
pub struct DownloadButtonProps {
    pub source: DownloadSource,
    pub filename: AttrValue,
    #[prop_or(FileType::Json)]
    pub filetype: FileType,

    #[prop_or_else(|| AttrValue::from("Download"))]
    pub label: AttrValue,

    #[prop_or(ButtonType::Primary)]
    pub button_type: ButtonType,

    #[prop_or_default]
    pub class: Classes,
}

#[function_component(DownloadButton)]
pub fn download_button(props: &DownloadButtonProps) -> Html {
    let DownloadButtonProps {
        source,
        filename,
        filetype,
        label,
        button_type,
        class,
    } = props.clone();

    // Keep object URL alive across re-renders
    let blob_url = use_mut_ref(|| None::<ObjectUrl>);

    let onclick = {
        let source = source.clone();
        let filename = filename.clone();
        let filetype = filetype.clone();
        let blob_url = blob_url.clone();

        Callback::from(move |_| {
            let document = web_sys::window().unwrap().document().unwrap();
            let body = document.body().unwrap();

            let anchor: HtmlAnchorElement = document.create_element("a").unwrap().unchecked_into();

            match &source {
                DownloadSource::Url(url) => {
                    anchor.set_href(url);
                }
                DownloadSource::Json(json_string) => {
                    // Keep object URL alive
                    let data = json_string.clone().into_bytes();
                    let blob = Blob::new_with_options(data.as_slice(), Some(&filetype.mime_type()));
                    let object_url = ObjectUrl::from(blob);
                    anchor.set_href(&object_url);
                    *blob_url.borrow_mut() = Some(object_url);
                }
            }

            anchor.set_download(&filename);
            anchor.set_target("_blank");
            anchor.set_attribute("style", "display:none;").ok();

            body.append_child(&anchor).unwrap();
            anchor.click();
            body.remove_child(&anchor).ok();
        })
    };

    html! {
        <Button
            button_type={button_type}
            onclick={onclick}
            class={class}
        >
            { label }
        </Button>
    }
}
