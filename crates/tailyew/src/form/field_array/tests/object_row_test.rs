use super::super::styles::object_fields_grid_class;

#[test]
fn field_array_object_fields_grid_uses_labeled_record_columns_without_action_column() {
    let class = object_fields_grid_class().to_string();

    assert!(class.contains("grid-cols-1"));
    assert!(class.contains("sm:grid-cols-2"));
    assert!(!class.contains("field-array-object-columns"));
}
