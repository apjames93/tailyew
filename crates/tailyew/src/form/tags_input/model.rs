use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TagAddOutcome {
    Added,
    IgnoredEmpty,
}

pub(crate) fn normalize_tags(tags: Vec<String>, allow_duplicates: bool) -> Vec<String> {
    let mut normalized = Vec::new();
    for tag in tags {
        let Some(tag) = sanitize_tag(&tag) else {
            continue;
        };
        if allow_duplicates || !normalized.contains(&tag) {
            normalized.push(tag);
        }
    }
    normalized
}

pub(crate) fn sanitize_tag(raw: &str) -> Option<String> {
    let tag = raw.trim().trim_end_matches(',').trim().to_owned();
    (!tag.is_empty()).then_some(tag)
}

pub(crate) fn add_tag_to_list(
    tags: &mut Vec<String>,
    raw_tag: &str,
    allow_duplicates: bool,
    allow_custom_tags: bool,
    suggestions: &[String],
    max_tags: Option<usize>,
) -> Result<TagAddOutcome, String> {
    let Some(tag) = sanitize_tag(raw_tag) else {
        return Ok(TagAddOutcome::IgnoredEmpty);
    };

    if !tag_is_allowed(&tag, allow_custom_tags, suggestions) {
        return Err("Select a suggested tag.".to_owned());
    }

    if max_tags.is_some_and(|max_tags| tags.len() >= max_tags) {
        return Err("Maximum tag count reached.".to_owned());
    }

    if !allow_duplicates && tags.contains(&tag) {
        return Err(format!("Tag `{tag}` already exists."));
    }

    tags.push(tag);
    Ok(TagAddOutcome::Added)
}

pub(crate) fn tag_is_allowed(tag: &str, allow_custom_tags: bool, suggestions: &[String]) -> bool {
    allow_custom_tags
        || suggestions
            .iter()
            .filter_map(|suggestion| sanitize_tag(suggestion))
            .any(|suggestion| suggestion == tag)
}

pub(crate) fn remove_tag_at(tags: &mut Vec<String>, index: usize) -> bool {
    if index >= tags.len() {
        return false;
    }

    tags.remove(index);
    true
}

pub(crate) fn tags_to_value(tags: &[String]) -> Value {
    Value::Array(tags.iter().cloned().map(Value::String).collect())
}

pub(crate) fn can_add_more(tags: &[String], max_tags: Option<usize>) -> bool {
    max_tags.is_none_or(|max_tags| tags.len() < max_tags)
}

pub(crate) fn can_remove_more(tags: &[String], min_tags: Option<usize>) -> bool {
    min_tags.is_none_or(|min_tags| tags.len() > min_tags)
}

pub(crate) fn tag_validation_message(
    tags: &[String],
    min_tags: Option<usize>,
    max_tags: Option<usize>,
) -> Option<String> {
    if let Some(min_tags) = min_tags
        && tags.len() < min_tags
    {
        return Some(format!(
            "Add at least {min_tags} {} before submitting.",
            tag_count_noun(min_tags)
        ));
    }

    if let Some(max_tags) = max_tags
        && tags.len() > max_tags
    {
        return Some(format!(
            "Use at most {max_tags} {} before submitting.",
            tag_count_noun(max_tags)
        ));
    }

    None
}

fn tag_count_noun(count: usize) -> &'static str {
    if count == 1 { "tag" } else { "tags" }
}
