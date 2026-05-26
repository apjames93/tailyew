use yew::prelude::*;

/// A generic video component with default styles for TailYew.
#[derive(Properties, PartialEq, Clone)]
pub struct VideoProps {
    /// Video URL or app-served asset path.
    pub src: AttrValue,

    /// Optional Tailwind classes.
    #[prop_or_default]
    pub class: Classes,

    /// Optional poster image URL shown before playback.
    #[prop_or_default]
    pub poster: Option<AttrValue>,

    /// Optional title for embedded videos.
    #[prop_or_default]
    pub title: Option<AttrValue>,

    /// Optional preload behavior: "none", "metadata", or "auto".
    #[prop_or_default]
    pub preload: Option<AttrValue>,

    /// Optional video height (CSS style).
    #[prop_or_default]
    pub height: Option<String>,

    /// Optional video width (CSS style).
    #[prop_or_default]
    pub width: Option<String>,

    /// Whether playback controls are shown.
    #[prop_or(true)]
    pub controls: bool,

    /// Whether playback should start automatically.
    #[prop_or(false)]
    pub autoplay: bool,

    /// Whether audio should be muted.
    #[prop_or(false)]
    pub muted: bool,

    /// Whether playback should loop.
    #[prop_or(false)]
    pub loop_video: bool,

    /// Whether the video should play inline on mobile browsers.
    #[prop_or(true)]
    pub plays_inline: bool,

    /// Optional ARIA label.
    #[prop_or_default]
    pub aria_label: Option<AttrValue>,

    /// Optional ARIA describedby ID.
    #[prop_or_default]
    pub aria_describedby: Option<AttrValue>,

    /// Optional role.
    #[prop_or_default]
    pub role: Option<AttrValue>,

    /// Optional fallback content or child tracks.
    #[prop_or_default]
    pub children: Children,
}

#[component(Video)]
pub fn video(props: &VideoProps) -> Html {
    if let Some(embed_src) = youtube_embed_src(props.src.as_str()) {
        let title = props
            .title
            .clone()
            .or_else(|| props.aria_label.clone())
            .unwrap_or_else(|| AttrValue::from("Embedded video"));

        return html! {
            <iframe
                src={embed_src}
                title={title}
                class={classes!("block", "aspect-video", "w-full", "max-w-full", "rounded-lg", "bg-black", props.class.clone())}
                style={video_style(&props.width, &props.height)}
                loading="lazy"
                allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
                allowfullscreen={true}
                referrerpolicy="strict-origin-when-cross-origin"
                aria-label={props.aria_label.clone()}
                aria-describedby={props.aria_describedby.clone()}
                role={props.role.clone()}
            />
        };
    }

    let style = video_style(&props.width, &props.height);

    html! {
        <video
            src={props.src.clone()}
            class={classes!("block", "max-w-full", "h-auto", "rounded-lg", "bg-black", props.class.clone())}
            style={style}
            poster={props.poster.clone()}
            preload={props.preload.clone()}
            controls={props.controls}
            autoplay={props.autoplay}
            muted={props.muted}
            loop={props.loop_video}
            playsinline={props.plays_inline}
            aria-label={props.aria_label.clone()}
            aria-describedby={props.aria_describedby.clone()}
            role={props.role.clone()}
        >
            { for props.children.iter() }
        </video>
    }
}

fn video_style(width: &Option<String>, height: &Option<String>) -> Option<AttrValue> {
    {
        let mut parts = String::new();
        if let Some(w) = width {
            parts.push_str(&format!("width:{};", w));
        }
        if let Some(h) = height {
            parts.push_str(&format!("height:{};", h));
        }
        if parts.is_empty() {
            None
        } else {
            Some(AttrValue::from(parts))
        }
    }
}

pub(crate) fn is_embeddable_video_source(src: &str) -> bool {
    youtube_embed_src(src).is_some()
}

fn youtube_embed_src(src: &str) -> Option<AttrValue> {
    let src = src.trim();
    let lower = src.to_ascii_lowercase();

    if has_youtube_path(&lower, "embed/") {
        return Some(AttrValue::from(src.to_string()));
    }

    if has_youtube_path(&lower, "watch") {
        return query_param(src, "v").map(youtube_embed_url);
    }

    if has_youtube_path(&lower, "shorts/") {
        return path_segment_after(src, "youtube.com/shorts/").map(youtube_embed_url);
    }

    if has_youtu_be_host(&lower) {
        return path_segment_after(src, "youtu.be/").map(youtube_embed_url);
    }

    None
}

fn has_youtube_path(src: &str, path: &str) -> bool {
    host_and_path(src).starts_with(&format!("youtube.com/{}", path))
}

fn has_youtu_be_host(src: &str) -> bool {
    host_and_path(src).starts_with("youtu.be/")
}

fn host_and_path(src: &str) -> &str {
    let src = src
        .strip_prefix("https://")
        .or_else(|| src.strip_prefix("http://"))
        .unwrap_or(src);

    src.strip_prefix("www.").unwrap_or(src)
}

fn youtube_embed_url(video_id: &str) -> AttrValue {
    AttrValue::from(format!("https://www.youtube.com/embed/{}", video_id))
}

fn query_param<'a>(src: &'a str, key: &str) -> Option<&'a str> {
    let query = src.split_once('?')?.1.split('#').next().unwrap_or_default();

    query.split('&').find_map(|pair| {
        let (pair_key, value) = pair.split_once('=')?;
        if pair_key == key {
            sanitize_video_id(value)
        } else {
            None
        }
    })
}

fn path_segment_after<'a>(src: &'a str, marker: &str) -> Option<&'a str> {
    let lower = src.to_ascii_lowercase();
    let index = lower.find(marker)?;
    let segment = &src[index + marker.len()..];

    sanitize_video_id(
        segment
            .split(['?', '#', '/', '&'])
            .next()
            .unwrap_or_default(),
    )
}

fn sanitize_video_id(value: &str) -> Option<&str> {
    let value = value.trim();
    if value.is_empty()
        || !value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '-')
    {
        None
    } else {
        Some(value)
    }
}

#[cfg(test)]
mod tests {
    use super::youtube_embed_src;

    #[test]
    fn converts_youtube_watch_urls_to_embeds() {
        let embed =
            youtube_embed_src("https://www.youtube.com/watch?v=SJPu1spHqfk&feature=share").unwrap();

        assert_eq!(embed.as_str(), "https://www.youtube.com/embed/SJPu1spHqfk");
    }

    #[test]
    fn converts_short_youtube_urls_to_embeds() {
        let embed = youtube_embed_src("https://youtu.be/SJPu1spHqfk?si=example").unwrap();

        assert_eq!(embed.as_str(), "https://www.youtube.com/embed/SJPu1spHqfk");
    }

    #[test]
    fn rejects_non_youtube_urls_that_mention_youtube() {
        assert!(youtube_embed_src("https://example.com/watch?next=youtube.com/watch").is_none());
        assert!(youtube_embed_src("https://www.youtube.com/watch?feature=share").is_none());
    }
}
