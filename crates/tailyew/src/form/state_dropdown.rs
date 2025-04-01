use crate::cp::atoms::{Select, SelectOption};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct StateDropdownProps {
    #[prop_or("CO".to_string())]
    pub default_value: String,
    #[prop_or("state".to_string())]
    pub id: String,
}

#[function_component(StateDropdown)]
pub fn state_dropdown(props: &StateDropdownProps) -> Html {
    let StateDropdownProps { id, default_value } = props;

    let state_options = vec![
        SelectOption {
            label: "Alabama".to_string(),
            value: "AL".to_string(),
        },
        SelectOption {
            label: "Alaska".to_string(),
            value: "AK".to_string(),
        },
        SelectOption {
            label: "Arizona".to_string(),
            value: "AZ".to_string(),
        },
        SelectOption {
            label: "Arkansas".to_string(),
            value: "AR".to_string(),
        },
        SelectOption {
            label: "California".to_string(),
            value: "CA".to_string(),
        },
        SelectOption {
            label: "Colorado".to_string(),
            value: "CO".to_string(),
        },
        SelectOption {
            label: "Connecticut".to_string(),
            value: "CT".to_string(),
        },
        SelectOption {
            label: "Delaware".to_string(),
            value: "DE".to_string(),
        },
        SelectOption {
            label: "Florida".to_string(),
            value: "FL".to_string(),
        },
        SelectOption {
            label: "Georgia".to_string(),
            value: "GA".to_string(),
        },
        SelectOption {
            label: "Hawaii".to_string(),
            value: "HI".to_string(),
        },
        SelectOption {
            label: "Idaho".to_string(),
            value: "ID".to_string(),
        },
        SelectOption {
            label: "Illinois".to_string(),
            value: "IL".to_string(),
        },
        SelectOption {
            label: "Indiana".to_string(),
            value: "IN".to_string(),
        },
        SelectOption {
            label: "Iowa".to_string(),
            value: "IA".to_string(),
        },
        SelectOption {
            label: "Kansas".to_string(),
            value: "KS".to_string(),
        },
        SelectOption {
            label: "Kentucky".to_string(),
            value: "KY".to_string(),
        },
        SelectOption {
            label: "Louisiana".to_string(),
            value: "LA".to_string(),
        },
        SelectOption {
            label: "Maine".to_string(),
            value: "ME".to_string(),
        },
        SelectOption {
            label: "Maryland".to_string(),
            value: "MD".to_string(),
        },
        SelectOption {
            label: "Massachusetts".to_string(),
            value: "MA".to_string(),
        },
        SelectOption {
            label: "Michigan".to_string(),
            value: "MI".to_string(),
        },
        SelectOption {
            label: "Minnesota".to_string(),
            value: "MN".to_string(),
        },
        SelectOption {
            label: "Mississippi".to_string(),
            value: "MS".to_string(),
        },
        SelectOption {
            label: "Missouri".to_string(),
            value: "MO".to_string(),
        },
        SelectOption {
            label: "Montana".to_string(),
            value: "MT".to_string(),
        },
        SelectOption {
            label: "Nebraska".to_string(),
            value: "NE".to_string(),
        },
        SelectOption {
            label: "Nevada".to_string(),
            value: "NV".to_string(),
        },
        SelectOption {
            label: "New Hampshire".to_string(),
            value: "NH".to_string(),
        },
        SelectOption {
            label: "New Jersey".to_string(),
            value: "NJ".to_string(),
        },
        SelectOption {
            label: "New Mexico".to_string(),
            value: "NM".to_string(),
        },
        SelectOption {
            label: "New York".to_string(),
            value: "NY".to_string(),
        },
        SelectOption {
            label: "North Carolina".to_string(),
            value: "NC".to_string(),
        },
        SelectOption {
            label: "North Dakota".to_string(),
            value: "ND".to_string(),
        },
        SelectOption {
            label: "Ohio".to_string(),
            value: "OH".to_string(),
        },
        SelectOption {
            label: "Oklahoma".to_string(),
            value: "OK".to_string(),
        },
        SelectOption {
            label: "Oregon".to_string(),
            value: "OR".to_string(),
        },
        SelectOption {
            label: "Pennsylvania".to_string(),
            value: "PA".to_string(),
        },
        SelectOption {
            label: "Rhode Island".to_string(),
            value: "RI".to_string(),
        },
        SelectOption {
            label: "South Carolina".to_string(),
            value: "SC".to_string(),
        },
        SelectOption {
            label: "South Dakota".to_string(),
            value: "SD".to_string(),
        },
        SelectOption {
            label: "Tennessee".to_string(),
            value: "TN".to_string(),
        },
        SelectOption {
            label: "Texas".to_string(),
            value: "TX".to_string(),
        },
        SelectOption {
            label: "Utah".to_string(),
            value: "UT".to_string(),
        },
        SelectOption {
            label: "Vermont".to_string(),
            value: "VT".to_string(),
        },
        SelectOption {
            label: "Virginia".to_string(),
            value: "VA".to_string(),
        },
        SelectOption {
            label: "Washington".to_string(),
            value: "WA".to_string(),
        },
        SelectOption {
            label: "West Virginia".to_string(),
            value: "WV".to_string(),
        },
        SelectOption {
            label: "Wisconsin".to_string(),
            value: "WI".to_string(),
        },
        SelectOption {
            label: "Wyoming".to_string(),
            value: "WY".to_string(),
        },
    ];

    html! {
        <Select
            id={id.to_string()}
            label={Some("State".to_string())}
            options={state_options}
            default_value={default_value.to_string()}
        />
    }
}
