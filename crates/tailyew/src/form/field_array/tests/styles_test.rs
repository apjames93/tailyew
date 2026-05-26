use super::super::styles::{
    field_array_add_action_class, field_array_list_class, field_array_root_class,
    field_array_row_class, field_array_scalar_wrapper_class,
};

#[test]
fn field_array_root_uses_neutral_field_group_spacing() {
    let class = field_array_root_class().to_string();

    assert!(class.contains("space-y-3"));
    assert!(!class.contains("rounded-xl"));
    assert!(!class.contains("shadow-sm"));
}

#[test]
fn field_array_list_and_rows_use_grouped_record_surfaces() {
    let list_class = field_array_list_class().to_string();
    let row_class = field_array_row_class(false).to_string();

    assert!(list_class.contains("space-y-3"));
    assert!(!list_class.contains("divide-y"));
    assert!(row_class.contains("rounded-lg"));
    assert!(row_class.contains("border"));
    assert!(row_class.contains("border-gray-200"));
    assert!(row_class.contains("bg-white"));
    assert!(row_class.contains("p-3"));
    assert!(!row_class.contains("shadow"));
}

#[test]
fn field_array_deleted_rows_use_muted_destructive_grouping() {
    let class = field_array_row_class(true).to_string();

    assert!(class.contains("rounded-lg"));
    assert!(class.contains("border-amber-200"));
    assert!(class.contains("bg-amber-50"));
    assert!(class.contains("opacity-80"));
}

#[test]
fn field_array_add_action_is_low_emphasis() {
    let class = field_array_add_action_class().to_string();

    assert!(class.contains("h-8"));
    assert!(class.contains("text-xs"));
    assert!(class.contains("shadow-none"));
}

#[test]
fn field_array_scalar_wrapper_quiets_delegated_json_input_chrome() {
    let class = field_array_scalar_wrapper_class().to_string();

    assert!(class.contains("[&>section]:border-0"));
    assert!(class.contains("[&>section]:shadow-none"));
    assert!(class.contains("[&>section>div:first-child]:px-0"));
}
