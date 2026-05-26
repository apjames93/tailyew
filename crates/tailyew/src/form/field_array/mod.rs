mod custom_validation;
mod deleted_rows;
mod extract;
mod model;
mod normalize;
mod object_editor;
mod object_field;
mod object_row;
mod props;
mod scalar_editor;
mod serialize;
mod styles;
mod validation;

#[cfg(test)]
mod tests;

pub use custom_validation::{
    FieldArrayCustomIssue, FieldArrayCustomIssueTarget, FieldArrayValidationContext,
    FieldArrayValidationRow, FieldArrayValidator,
};
pub use extract::{FieldArrayRow, FieldArrayRows, FieldArrayRowsError, FieldArrayRowsOptions};
pub use object_field::{
    FieldArrayFieldValidationContext, FieldArrayFieldValidator, FieldArrayObjectField,
    FieldArrayObjectFieldEditor, FieldArraySelectOption,
};
pub use props::{FieldArrayDeleteBehavior, FieldArrayProps, FieldArrayText};

use object_editor::ObjectFieldArray;
use scalar_editor::ScalarFieldArray;
use yew::prelude::*;

/// Edits a JSON array as scalar rows or repeated object-record rows.
#[component(FieldArray)]
pub fn field_array(props: &FieldArrayProps) -> Html {
    if props.object_fields.is_some() {
        html! { <ObjectFieldArray props={props.clone()} /> }
    } else {
        html! { <ScalarFieldArray props={props.clone()} /> }
    }
}

#[derive(Properties, PartialEq, Clone)]
pub(super) struct FieldArrayModeProps {
    pub props: FieldArrayProps,
}
