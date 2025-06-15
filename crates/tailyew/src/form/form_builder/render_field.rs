use crate::form::*;
use crate::{TagType, Typo};
use serde::Deserialize;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone, Default, Deserialize)]
pub struct RenderFieldProps {
    #[prop_or_default]
    pub input: Option<InputProps>,
    #[prop_or_default]
    pub textarea: Option<TextareaProps>,
    #[prop_or_default]
    pub select: Option<SelectProps>,
    #[prop_or_default]
    pub radio: Option<RadioGroupProps>,
    #[prop_or_default]
    pub checkbox: Option<CheckboxProps>,
    #[prop_or_default]
    pub color: Option<ColorInputProps>,
    #[prop_or_default]
    pub file: Option<FileInputProps>,
    #[prop_or_default]
    pub phone: Option<PhoneInputProps>,
    #[prop_or_default]
    pub range: Option<RangeInputProps>,
    #[prop_or_default]
    pub state: Option<StateDropdownProps>,
    #[prop_or_default]
    pub search: Option<SearchInputProps>,

    /// per-field wrapper classes
    #[prop_or_default]
    #[serde(default, deserialize_with = "de_classes")]
    pub class: Classes,
}

#[function_component(RenderField)]
pub fn render_field(props: &RenderFieldProps) -> Html {
    if let Some(input_props) = props.input.clone() {
        html! { <Input ..input_props /> }
    } else if let Some(textarea_props) = props.textarea.clone() {
        html! { <Textarea ..textarea_props /> }
    } else if let Some(select_props) = props.select.clone() {
        html! { <Select ..select_props /> }
    } else if let Some(radio_props) = props.radio.clone() {
        html! { <RadioGroup ..radio_props /> }
    } else if let Some(checkbox_props) = props.checkbox.clone() {
        html! { <Checkbox ..checkbox_props /> }
    } else if let Some(color_props) = props.color.clone() {
        html! { <ColorInput ..color_props /> }
    } else if let Some(file_props) = props.file.clone() {
        html! { <FileInput ..file_props /> }
    } else if let Some(phone_props) = props.phone.clone() {
        html! { <PhoneInput ..phone_props /> }
    } else if let Some(range_props) = props.range.clone() {
        html! { <RangeInput ..range_props /> }
    } else if let Some(state_props) = props.state.clone() {
        html! { <StateDropdown ..state_props /> }
    } else if let Some(search_props) = props.search.clone() {
        html! { <SearchInput ..search_props /> }
    } else {
        html! { <Typo tag={TagType::Error}>{"Unknown field type"}</Typo> }
    }
}
