#[cfg(test)]
use super::model::value_type_for_node;
use super::types::*;

#[derive(Clone, PartialEq, Debug)]
pub(super) struct ResolvedJsonInputPolicy {
    pub key_editable: bool,
    pub type_editable: bool,
    pub value_editable: bool,
    pub allowed_types: Vec<JsonValueType>,
    pub default_new_type: JsonValueType,
    pub removable: bool,
    pub allow_add_children: bool,
    pub allow_remove_children: bool,
}

pub(super) fn resolve_policy(
    config: &JsonInputConfig,
    path: &JsonInputPath,
) -> ResolvedJsonInputPolicy {
    let mut policy = ResolvedJsonInputPolicy {
        key_editable: !config.disable_keys,
        type_editable: !config.disable_values,
        value_editable: !config.disable_values,
        allowed_types: config.allowed_types.clone(),
        default_new_type: config.default_new_type,
        removable: true,
        allow_add_children: true,
        allow_remove_children: true,
    };

    let mut matching_policies = config
        .path_policies
        .iter()
        .enumerate()
        .filter_map(|(index, candidate)| {
            policy_match_score(&candidate.path, path).map(|score| (score, index, candidate))
        })
        .collect::<Vec<_>>();

    matching_policies.sort_by_key(|(score, index, _)| (*score, *index));

    for (_, _, candidate) in matching_policies {
        if let Some(value) = candidate.key_editable {
            policy.key_editable = value;
        }
        if let Some(value) = candidate.type_editable {
            policy.type_editable = value;
        }
        if let Some(value) = candidate.value_editable {
            policy.value_editable = value;
        }
        if let Some(value) = candidate
            .allowed_types
            .as_ref()
            .filter(|types| !types.is_empty())
        {
            policy.allowed_types = value.clone();
        }
        if let Some(value) = candidate.default_new_type {
            policy.default_new_type = value;
        }
        if let Some(value) = candidate.removable {
            policy.removable = value;
        }
        if let Some(value) = candidate.allow_add_children {
            policy.allow_add_children = value;
        }
        if let Some(value) = candidate.allow_remove_children {
            policy.allow_remove_children = value;
        }
    }

    policy.allowed_types =
        filter_allowed_types_for_depth(config, path.depth(), &policy.allowed_types);
    if !policy.allowed_types.contains(&policy.default_new_type) {
        policy.default_new_type = policy
            .allowed_types
            .first()
            .copied()
            .unwrap_or(JsonValueType::String);
    }

    policy
}

pub(super) fn allowed_types_for_path(
    config: &JsonInputConfig,
    path: &JsonInputPath,
) -> Vec<JsonValueType> {
    resolve_policy(config, path).allowed_types
}

pub(super) fn default_new_type_for_path(
    config: &JsonInputConfig,
    path: &JsonInputPath,
) -> JsonValueType {
    resolve_policy(config, path).default_new_type
}

#[cfg(test)]
pub(super) fn type_is_fixed_to_current_node(
    config: &JsonInputConfig,
    path: &JsonInputPath,
    node: &JsonModel,
) -> bool {
    let policy = resolve_policy(config, path);
    !policy.type_editable
        || (policy.allowed_types.len() == 1
            && policy.allowed_types.first().copied() == Some(value_type_for_node(node)))
}

fn filter_allowed_types_for_depth(
    config: &JsonInputConfig,
    depth: usize,
    allowed_types: &[JsonValueType],
) -> Vec<JsonValueType> {
    allowed_types
        .iter()
        .copied()
        .filter(|value_type| {
            !value_type.is_composite() || config.max_depth.is_none_or(|max_depth| depth < max_depth)
        })
        .collect()
}

fn policy_match_score(policy_path: &JsonInputPath, target_path: &JsonInputPath) -> Option<usize> {
    if policy_path.segments.len() != target_path.segments.len() {
        return None;
    }

    let mut score = 0;
    for (policy_segment, target_segment) in policy_path.segments.iter().zip(&target_path.segments) {
        score += match (policy_segment, target_segment) {
            (JsonInputPathSegment::Key(policy_key), JsonInputPathSegment::Key(target_key))
                if policy_key == target_key =>
            {
                3
            }
            (JsonInputPathSegment::AnyKey, JsonInputPathSegment::Key(_)) => 1,
            (
                JsonInputPathSegment::Index(policy_index),
                JsonInputPathSegment::Index(target_index),
            ) if policy_index == target_index => 3,
            (JsonInputPathSegment::AnyIndex, JsonInputPathSegment::Index(_)) => 1,
            _ => return None,
        };
    }

    Some(score)
}

#[cfg(test)]
pub(super) fn policy_matches_for_test(
    policy_path: &JsonInputPath,
    target_path: &JsonInputPath,
) -> Option<usize> {
    policy_match_score(policy_path, target_path)
}
