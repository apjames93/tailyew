use super::select::{Select, SelectOption};
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

#[derive(Properties, PartialEq, Clone, Default)]
pub struct StateDropdownProps {
    #[prop_or("CO".to_string())]
    pub default_value: String,
    #[prop_or("state".to_string())]
    pub id: String,
    #[prop_or_default]
    pub on_change: Option<Callback<String>>,
}

#[function_component(StateDropdown)]
pub fn state_dropdown(props: &StateDropdownProps) -> Html {
    html! {
        <Select
            id={props.id.clone()}
            label={Some("State".to_string())}
            options={us_state_options()}
            default_value={props.default_value.clone()}
            on_change={props.on_change.clone()}
        />
    }
}
