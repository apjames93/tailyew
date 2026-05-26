use yew::prelude::*;

pub mod json_input;
pub use json_input::*;

pub mod field_array;
pub use field_array::*;

pub mod key_value_input;
pub use key_value_input::*;

pub mod static_values_input;
pub use static_values_input::*;

pub mod tags_input;
pub use tags_input::*;

pub mod label;
pub use label::*;

pub mod input;
pub use input::*;

pub mod checkbox;
pub use checkbox::*;

pub mod color_input;
pub use color_input::*;

pub mod file_input;
pub use file_input::*;

pub mod phone_input;
pub use phone_input::*;

pub mod radio_group;
pub use radio_group::*;

pub mod range_input;
pub use range_input::*;

pub mod select;
pub use select::*;

pub mod textarea;
pub use textarea::*;

pub mod form_container;
pub use form_container::*;

pub mod state_dropdown;
pub use state_dropdown::*;

pub mod form_modal;
pub use form_modal::*;

pub mod form_builder;
pub use form_builder::*;

pub mod search_input;
pub use search_input::*;

pub mod form_helpers;
pub use form_helpers::*;

pub mod form_deserializer;
pub use form_deserializer::*;

pub mod switch;
pub use switch::*;

pub(crate) fn render_label_with_required_indicator(label: &AttrValue, required: bool) -> Html {
    html! {
        <>
            { label.clone() }
            {
                if required && !label.is_empty() {
                    html! { <span aria-hidden="true">{ " *" }</span> }
                } else {
                    html! {}
                }
            }
        </>
    }
}

pub(crate) fn submitted_name(id: &AttrValue, name: &Option<AttrValue>) -> AttrValue {
    name.clone().unwrap_or_else(|| id.clone())
}

pub(crate) fn join_aria_ids(
    values: impl IntoIterator<Item = Option<AttrValue>>,
) -> Option<AttrValue> {
    let ids = values
        .into_iter()
        .flatten()
        .map(|value| value.to_string())
        .filter(|value| !value.is_empty())
        .collect::<Vec<_>>();

    (!ids.is_empty()).then(|| AttrValue::from(ids.join(" ")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn submitted_name_defaults_to_id_or_uses_explicit_name() {
        let id = AttrValue::from("dom_id");

        assert_eq!(submitted_name(&id, &None), AttrValue::from("dom_id"));
        assert_eq!(
            submitted_name(&id, &Some(AttrValue::from("payload_key"))),
            AttrValue::from("payload_key")
        );
    }

    #[test]
    fn join_aria_ids_omits_empty_values_and_preserves_order() {
        assert_eq!(
            join_aria_ids(vec![
                Some(AttrValue::from("help")),
                Some(AttrValue::from("")),
                None,
                Some(AttrValue::from("error")),
            ]),
            Some(AttrValue::from("help error"))
        );
    }

    #[test]
    fn standard_form_controls_accept_common_contract_props() {
        let _controls: Html = html! {
            <>
                <ColorInput
                    id="accent_color_picker"
                    name="accent_color"
                    label="Accent color"
                    helper_text="Pick a brand color."
                    error="Choose a valid color."
                    aria_invalid={Some(true)}
                    required={true}
                    disabled={true}
                    on_blur={Some(Callback::from(|_: FocusEvent| {}))}
                />
                <FileInput
                    id="resume_upload"
                    name="resume"
                    label="Resume"
                    helper_text="Upload a PDF."
                    error="Choose a file."
                    required={true}
                    disabled={true}
                    on_blur={Some(Callback::from(|_: FocusEvent| {}))}
                />
                <PhoneInput
                    id="phone_editor"
                    name="phone"
                    label="Phone"
                    value={Some(AttrValue::from("555-123-4567"))}
                    helper_text="Use 555-123-4567 format."
                    error="Enter a valid phone number."
                    required={true}
                    disabled={true}
                    on_change={Some(Callback::from(|_: String| {}))}
                    on_blur={Some(Callback::from(|_: FocusEvent| {}))}
                />
                <RadioGroup
                    id="role_picker"
                    name="role"
                    label="Role"
                    options={vec![("admin".to_string(), "Admin".to_string())]}
                    value={Some(AttrValue::from("admin"))}
                    helper_text="Choose one role."
                    error="Choose a role."
                    required={true}
                    disabled={true}
                    on_blur={Some(Callback::from(|_: FocusEvent| {}))}
                />
                <RangeInput
                    id="volume_slider"
                    name="volume"
                    label="Volume"
                    value={Some(AttrValue::from("50"))}
                    helper_text="Set the volume."
                    error="Choose a value."
                    disabled={true}
                    on_blur={Some(Callback::from(|_: FocusEvent| {}))}
                />
                <SearchInput
                    id="language_search"
                    name="language"
                    label="Language"
                    helper_text="Choose from the suggestions."
                    error="Choose a language."
                    aria_invalid={Some(true)}
                    on_blur={Some(Callback::from(|_: FocusEvent| {}))}
                />
                <StateDropdown
                    id="state_selector"
                    name="shipping_state"
                    label="Shipping state"
                    helper_text="Choose a state."
                    error="Choose a state."
                    required={true}
                    disabled={true}
                    on_blur={Some(Callback::from(|_: FocusEvent| {}))}
                />
                <Switch
                    id="notifications_switch"
                    name="notifications"
                    label="Notifications"
                    helper_text="Receive updates."
                    error="Enable or disable notifications."
                    aria_invalid={Some(true)}
                    on_blur={Some(Callback::from(|_: FocusEvent| {}))}
                />
                <Checkbox
                    id="terms_checkbox"
                    name="terms"
                    label="Terms"
                    helper_text="Accept the terms."
                    error="Accept the terms."
                    aria_invalid={Some(true)}
                    on_blur={Some(Callback::from(|_: FocusEvent| {}))}
                />
                <Select
                    id="team_select"
                    name="team"
                    label="Team"
                    helper_text="Choose a team."
                    error="Choose a team."
                    on_blur={Some(Callback::from(|_: FocusEvent| {}))}
                />
                <Textarea
                    id="bio_textarea"
                    name="bio"
                    label="Bio"
                    helper_text="Write a short bio."
                    error="Bio is required."
                    aria_invalid={Some(true)}
                    disabled={true}
                    on_blur={Some(Callback::from(|_: FocusEvent| {}))}
                />
            </>
        };
    }
}
