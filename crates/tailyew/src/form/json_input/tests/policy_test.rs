use super::super::model::{
    model_from_value, new_model_for_type, node_kind_mut_at_path, value_from_model,
};
use super::super::policy::{
    default_new_type_for_path, policy_matches_for_test, resolve_policy,
    type_is_fixed_to_current_node,
};
use super::super::types::{
    JsonArrayItemNode, JsonInputPath, JsonInputPathPolicy, JsonInputPathSegment, JsonNodeKind,
    JsonPathSegment, JsonValueType,
};
use super::{first_property_mut, test_config};
use serde_json::json;
use uuid::Uuid;

#[test]
fn no_path_policy_preserves_freeform_behavior() {
    let config = test_config();
    let policy = resolve_policy(&config, &JsonInputPath::key("metadata"));

    assert!(policy.key_editable);
    assert!(policy.type_editable);
    assert!(policy.value_editable);
    assert!(policy.removable);
    assert!(policy.allow_add_children);
    assert!(policy.allow_remove_children);
    assert_eq!(policy.allowed_types, JsonValueType::all());
}

#[test]
fn exact_path_policy_applies_to_matching_key() {
    let mut config = test_config();
    config.path_policies = vec![
        JsonInputPathPolicy::for_key("tags")
            .key_editable(false)
            .type_editable(false)
            .allowed_types(vec![JsonValueType::Array])
            .removable(false),
    ];

    let policy = resolve_policy(&config, &JsonInputPath::key("tags"));

    assert!(!policy.key_editable);
    assert!(!policy.type_editable);
    assert!(!policy.removable);
    assert_eq!(policy.allowed_types, vec![JsonValueType::Array]);
}

#[test]
fn any_index_policy_applies_to_array_items() {
    let mut config = test_config();
    config.path_policies = vec![
        JsonInputPathPolicy::for_key("tags")
            .any_index()
            .type_editable(false)
            .allowed_types(vec![JsonValueType::String]),
    ];

    let policy = resolve_policy(&config, &JsonInputPath::key("tags").index(3));

    assert!(!policy.type_editable);
    assert_eq!(policy.allowed_types, vec![JsonValueType::String]);
}

#[test]
fn any_key_policy_applies_to_object_children() {
    let mut config = test_config();
    config.path_policies = vec![
        JsonInputPathPolicy::for_path(JsonInputPath::key("headers").any_key())
            .type_editable(false)
            .allowed_types(vec![JsonValueType::String]),
    ];

    let policy = resolve_policy(&config, &JsonInputPath::keys(["headers", "Accept"]));

    assert!(!policy.type_editable);
    assert_eq!(policy.allowed_types, vec![JsonValueType::String]);
}

#[test]
fn exact_index_policy_beats_any_index_policy() {
    let mut config = test_config();
    config.path_policies = vec![
        JsonInputPathPolicy::for_key("tags")
            .any_index()
            .allowed_types(vec![JsonValueType::String]),
        JsonInputPathPolicy::for_key("tags")
            .index(1)
            .allowed_types(vec![JsonValueType::Number]),
    ];

    let first = resolve_policy(&config, &JsonInputPath::key("tags").index(0));
    let second = resolve_policy(&config, &JsonInputPath::key("tags").index(1));

    assert_eq!(first.allowed_types, vec![JsonValueType::String]);
    assert_eq!(second.allowed_types, vec![JsonValueType::Number]);
}

#[test]
fn path_allowed_types_override_global_allowed_types() {
    let mut config = test_config();
    config.allowed_types = vec![JsonValueType::String];
    config.path_policies =
        vec![JsonInputPathPolicy::for_key("enabled").allowed_types(vec![JsonValueType::Boolean])];

    let global = resolve_policy(&config, &JsonInputPath::key("name"));
    let enabled = resolve_policy(&config, &JsonInputPath::key("enabled"));

    assert_eq!(global.allowed_types, vec![JsonValueType::String]);
    assert_eq!(enabled.allowed_types, vec![JsonValueType::Boolean]);
}

#[test]
fn default_new_type_resolves_from_path_policy() {
    let mut config = test_config();
    config.path_policies = vec![
        JsonInputPathPolicy::for_key("tags")
            .any_index()
            .allowed_types(vec![JsonValueType::String])
            .default_new_type(JsonValueType::String),
    ];

    assert_eq!(
        default_new_type_for_path(&config, &JsonInputPath::key("tags").index(0)),
        JsonValueType::String
    );
}

#[test]
fn locked_editing_flags_resolve_from_policy() {
    let mut config = test_config();
    config.path_policies = vec![
        JsonInputPathPolicy::for_key("enabled")
            .key_editable(false)
            .type_editable(false)
            .value_editable(true)
            .removable(false),
    ];

    let policy = resolve_policy(&config, &JsonInputPath::key("enabled"));

    assert!(!policy.key_editable);
    assert!(!policy.type_editable);
    assert!(policy.value_editable);
    assert!(!policy.removable);
}

#[test]
fn child_add_and_remove_permissions_resolve_from_parent_policy() {
    let mut config = test_config();
    config.path_policies = vec![
        JsonInputPathPolicy::for_path(JsonInputPath::root())
            .allow_add_children(false)
            .allow_remove_children(false),
    ];

    let policy = resolve_policy(&config, &JsonInputPath::root());

    assert!(!policy.allow_add_children);
    assert!(!policy.allow_remove_children);
}

#[test]
fn exact_path_and_any_index_match_scores_are_specific() {
    let any_index = JsonInputPath::key("tags").any_index();
    let exact_index = JsonInputPath::key("tags").index(1);
    let target = JsonInputPath::key("tags").index(1);

    assert!(
        policy_matches_for_test(&exact_index, &target)
            > policy_matches_for_test(&any_index, &target)
    );
    assert_eq!(
        policy_matches_for_test(&JsonInputPath::key("tags"), &target),
        None
    );
}

#[test]
fn locked_keys_still_serialize_correctly() {
    let model = model_from_value(&json!({
        "enabled": true,
        "tags": ["beta", "internal"]
    }));

    assert_eq!(
        value_from_model(&model).unwrap(),
        json!({
            "enabled": true,
            "tags": ["beta", "internal"]
        })
    );
}

#[test]
fn adding_tag_with_any_index_policy_creates_string_item() {
    let mut config = test_config();
    config.path_policies = vec![
        JsonInputPathPolicy::for_key("tags")
            .any_index()
            .allowed_types(vec![JsonValueType::String])
            .default_new_type(JsonValueType::String),
    ];
    let mut model = model_from_value(&json!({ "tags": [] }));
    let tags_id = first_property_mut(&mut model, "tags").id;
    let tags_path = vec![JsonPathSegment::Property(tags_id)];
    let new_type = default_new_type_for_path(&config, &JsonInputPath::key("tags").index(0));

    if let Some(JsonNodeKind::Array(items)) = node_kind_mut_at_path(&mut model, &tags_path) {
        items.push(JsonArrayItemNode {
            id: Uuid::new_v4(),
            value: new_model_for_type(new_type, false),
        });
    }

    assert_eq!(value_from_model(&model).unwrap(), json!({ "tags": [""] }));
}

#[test]
fn type_locked_policy_marks_node_type_as_fixed() {
    let mut config = test_config();
    config.path_policies = vec![
        JsonInputPathPolicy::for_key("enabled")
            .type_editable(false)
            .allowed_types(vec![JsonValueType::Boolean]),
    ];
    let model = model_from_value(&json!(true));

    assert!(type_is_fixed_to_current_node(
        &config,
        &JsonInputPath::key("enabled"),
        &model
    ));
}

#[test]
fn path_builder_supports_key_index_and_any_index_segments() {
    let path = JsonInputPath::key("tags").index(2);
    let wildcard_path = JsonInputPath::key("tags").any_index();
    let wildcard_key_path = JsonInputPath::key("headers").any_key();

    assert_eq!(
        path.segments,
        vec![
            JsonInputPathSegment::Key("tags".to_owned()),
            JsonInputPathSegment::Index(2)
        ]
    );
    assert_eq!(
        wildcard_path.segments,
        vec![
            JsonInputPathSegment::Key("tags".to_owned()),
            JsonInputPathSegment::AnyIndex
        ]
    );
    assert_eq!(
        wildcard_key_path.segments,
        vec![
            JsonInputPathSegment::Key("headers".to_owned()),
            JsonInputPathSegment::AnyKey
        ]
    );
}
