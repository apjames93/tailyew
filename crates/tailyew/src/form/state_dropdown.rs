use super::select::{Select, SelectOption};
use crate::form_deserializer::{de_attr, de_classes, de_option_attr};
use serde::Deserialize;
use yew::prelude::*;

const STATES: &[(&str, &str)] = &[
    ("AL", "Alabama"),
    ("AK", "Alaska"),
    ("AZ", "Arizona"),
    ("AR", "Arkansas"),
    ("CA", "California"),
    ("CO", "Colorado"),
    ("CT", "Connecticut"),
    ("DE", "Delaware"),
    ("FL", "Florida"),
    ("GA", "Georgia"),
    ("HI", "Hawaii"),
    ("ID", "Idaho"),
    ("IL", "Illinois"),
    ("IN", "Indiana"),
    ("IA", "Iowa"),
    ("KS", "Kansas"),
    ("KY", "Kentucky"),
    ("LA", "Louisiana"),
    ("ME", "Maine"),
    ("MD", "Maryland"),
    ("MA", "Massachusetts"),
    ("MI", "Michigan"),
    ("MN", "Minnesota"),
    ("MS", "Mississippi"),
    ("MO", "Missouri"),
    ("MT", "Montana"),
    ("NE", "Nebraska"),
    ("NV", "Nevada"),
    ("NH", "New Hampshire"),
    ("NJ", "New Jersey"),
    ("NM", "New Mexico"),
    ("NY", "New York"),
    ("NC", "North Carolina"),
    ("ND", "North Dakota"),
    ("OH", "Ohio"),
    ("OK", "Oklahoma"),
    ("OR", "Oregon"),
    ("PA", "Pennsylvania"),
    ("RI", "Rhode Island"),
    ("SC", "South Carolina"),
    ("SD", "South Dakota"),
    ("TN", "Tennessee"),
    ("TX", "Texas"),
    ("UT", "Utah"),
    ("VT", "Vermont"),
    ("VA", "Virginia"),
    ("WA", "Washington"),
    ("WV", "West Virginia"),
    ("WI", "Wisconsin"),
    ("WY", "Wyoming"),
];

fn us_state_options() -> Vec<SelectOption> {
    STATES
        .iter()
        .map(|(abbr, name)| SelectOption {
            label: name.to_string(),
            value: abbr.to_string(),
        })
        .collect()
}

fn default_state_label() -> AttrValue {
    "State".into()
}

fn default_true() -> bool {
    true
}

#[derive(Properties, PartialEq, Clone, Deserialize)]
pub struct StateDropdownProps {
    #[prop_or_default]
    #[serde(default, deserialize_with = "de_attr")]
    pub id: AttrValue,

    #[prop_or_default]
    #[serde(default, deserialize_with = "de_option_attr")]
    pub name: Option<AttrValue>,

    #[prop_or("State".into())]
    #[serde(default = "default_state_label", deserialize_with = "de_attr")]
    pub label: AttrValue,

    #[prop_or_default]
    #[serde(default, deserialize_with = "de_attr")]
    pub default_value: AttrValue,

    #[prop_or_default]
    #[serde(default, deserialize_with = "de_option_attr")]
    pub value: Option<AttrValue>,

    #[prop_or(false)]
    #[serde(default)]
    pub visually_hidden_label: bool,

    #[prop_or_default]
    #[serde(default, rename = "aria-label", deserialize_with = "de_option_attr")]
    pub aria_label: Option<AttrValue>,

    #[prop_or_default]
    #[serde(default, deserialize_with = "de_option_attr")]
    pub helper_text: Option<AttrValue>,

    #[prop_or_default]
    #[serde(default, deserialize_with = "de_option_attr")]
    pub error: Option<AttrValue>,

    #[prop_or_default]
    #[serde(default)]
    pub aria_invalid: Option<bool>,

    #[prop_or_default]
    #[serde(
        default,
        rename = "aria-describedby",
        deserialize_with = "de_option_attr"
    )]
    pub aria_describedby: Option<AttrValue>,

    #[prop_or(true)]
    #[serde(default = "default_true")]
    pub required: bool,

    #[prop_or(false)]
    #[serde(default)]
    pub disabled: bool,

    #[prop_or_default]
    #[serde(default, deserialize_with = "de_classes")]
    pub class: Classes,

    #[prop_or_default]
    #[serde(default, deserialize_with = "de_classes")]
    pub container_class: Classes,

    #[prop_or_default]
    #[serde(skip)]
    pub on_change: Option<Callback<String>>,

    #[prop_or_default]
    #[serde(skip)]
    pub on_blur: Option<Callback<FocusEvent>>,
}

impl Default for StateDropdownProps {
    fn default() -> Self {
        Self {
            id: AttrValue::default(),
            name: None,
            label: "State".into(),
            default_value: AttrValue::default(),
            value: None,
            visually_hidden_label: false,
            aria_label: None,
            helper_text: None,
            error: None,
            aria_invalid: None,
            aria_describedby: None,
            required: true,
            disabled: false,
            class: Classes::default(),
            container_class: Classes::default(),
            on_change: None,
            on_blur: None,
        }
    }
}

#[component(StateDropdown)]
pub fn state_dropdown(props: &StateDropdownProps) -> Html {
    html! {
        <Select
            id={props.id.clone()}
            name={props.name.clone()}
            label={props.label.clone()}
            options={us_state_options()}
            default_value={props.default_value.clone()}
            value={props.value.clone()}
            visually_hidden_label={props.visually_hidden_label}
            aria_label={props.aria_label.clone()}
            helper_text={props.helper_text.clone()}
            error={props.error.clone()}
            aria_invalid={props.aria_invalid}
            aria_describedby={props.aria_describedby.clone()}
            required={props.required}
            disabled={props.disabled}
            class={props.class.clone()}
            container_class={props.container_class.clone()}
            on_change={props.on_change.clone()}
            on_blur={props.on_blur.clone()}
        />
    }
}
