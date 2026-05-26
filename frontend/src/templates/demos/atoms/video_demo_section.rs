use crate::templates::demos::DemoComponent;
use tailyew::atoms::{Typo, Video};
use tailyew::organisms::table::Column;
use yew::prelude::*;

const RE_TRAILER: &str = "https://www.youtube.com/watch?v=SJPu1spHqfk";

#[component(VideoDemoSection)]
pub fn video_demo_section() -> Html {
    let example = html! {
        <div class="space-y-6 text-left">
            <Video
                src="/static/images/demo.mov"
                title={Some(AttrValue::from("Local TailYew demo video"))}
                aria_label={Some(AttrValue::from("Local TailYew demo video"))}
                class="mx-auto shadow-md"
                width={Some("720px".to_string())}
            />

            <Video
                src={RE_TRAILER}
                title={Some(AttrValue::from("RESIDENT EVIL - Official Teaser Trailer (4K)"))}
                aria_label={Some(AttrValue::from("RESIDENT EVIL - Official Teaser Trailer (4K)"))}
                class="mx-auto shadow-md"
            />
        </div>
    };

    let usage_code = r#"
<Video
    src="/static/images/demo.mov"
    title={Some(AttrValue::from("Local TailYew demo video"))}
    aria_label={Some(AttrValue::from("Local TailYew demo video"))}
    width={Some("720px".to_string())}
/>

<Video
    src="https://www.youtube.com/watch?v=SJPu1spHqfk"
    title={Some(AttrValue::from("RESIDENT EVIL - Official Teaser Trailer (4K)"))}
    aria_label={Some(AttrValue::from("RESIDENT EVIL - Official Teaser Trailer (4K)"))}
/>
"#;

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "src".into(),
                "class".into(),
                "poster".into(),
                "title".into(),
                "preload".into(),
                "height".into(),
                "width".into(),
                "controls".into(),
                "autoplay".into(),
                "muted".into(),
                "loop_video".into(),
                "plays_inline".into(),
                "aria_label".into(),
                "aria_describedby".into(),
                "role".into(),
                "children".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "AttrValue".into(),
                "Classes".into(),
                "Option<AttrValue>".into(),
                "Option<AttrValue>".into(),
                "Option<AttrValue>".into(),
                "Option<String>".into(),
                "Option<String>".into(),
                "bool".into(),
                "bool".into(),
                "bool".into(),
                "bool".into(),
                "bool".into(),
                "Option<AttrValue>".into(),
                "Option<AttrValue>".into(),
                "Option<AttrValue>".into(),
                "Children".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "Video URL, app-served asset path, or supported YouTube URL.".into(),
                "Tailwind utility classes for the media element.".into(),
                "Poster image shown before native video playback.".into(),
                "Title used for embedded video players.".into(),
                "Native video preload behavior.".into(),
                "Optional CSS height.".into(),
                "Optional CSS width.".into(),
                "Shows native playback controls.".into(),
                "Starts native playback automatically.".into(),
                "Mutes native playback audio.".into(),
                "Loops native playback.".into(),
                "Keeps native playback inline on mobile browsers.".into(),
                "Screen reader label.".into(),
                "ID of an element that describes the video.".into(),
                "Optional role attribute.".into(),
                "Fallback content or child tracks for native videos.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            github_demo_path="atoms/video_demo_section.rs"
            github_source_path="atoms/video.rs"
            title="Video Component"
            description={Some(html! {
                <Typo>
                    {"The "}
                    <code>{"Video"}</code>
                    {" component renders local video files with native controls and supported YouTube URLs as responsive embeds."}
                </Typo>
            })}
            example={example}
            usage_code={usage_code}
            props_table={Some(props_table)}
        />
    }
}
