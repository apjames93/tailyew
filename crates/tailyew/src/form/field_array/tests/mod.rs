mod custom_validation_test;
mod deleted_rows_test;
mod extract_test;
mod normalize_test;
mod object_field_test;
mod object_row_test;
mod props_test;
mod scalar_editor_test;
mod serialize_test;
mod styles_test;
mod validation_test;

use super::object_field::FieldArrayObjectField;
use crate::form::JsonValueType;

fn game_fields() -> Vec<FieldArrayObjectField> {
    vec![
        FieldArrayObjectField::hidden("id", JsonValueType::Number),
        FieldArrayObjectField::string("name", "Game").required(true),
        FieldArrayObjectField::string("hours_played", "Hours played"),
        FieldArrayObjectField::boolean("beat", "Beat"),
    ]
}
