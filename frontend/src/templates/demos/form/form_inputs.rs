use tailyew::form::*;
use yew::prelude::*;

#[function_component(FormInputs)]
pub fn form_inputs() -> Html {
    // Options for inputs
    let language_options = vec![
        SelectOption {
            label: "Rust".into(),
            value: "rust".into(),
        },
        SelectOption {
            label: "Go".into(),
            value: "go".into(),
        },
        SelectOption {
            label: "TypeScript".into(),
            value: "typescript".into(),
        },
    ];

    let select_options = vec![
        SelectOption {
            label: "Option 1".into(),
            value: "1".into(),
        },
        SelectOption {
            label: "Option 2".into(),
            value: "2".into(),
        },
        SelectOption {
            label: "Option 3".into(),
            value: "3".into(),
        },
    ];

    let radio_options = vec![
        ("male".into(), "Male".into()),
        ("female".into(), "Female".into()),
        ("other".into(), "Other".into()),
    ];

    html! {
        <>
            <Input id="status"   label="Status Code" input_type={InputType::Number} default_value="200" placeholder="e.g. 200 or 500" />
            <Input id="username" label="Username"    input_type={InputType::Text} placeholder="e.g. buddy_guy"
                pattern={Some("^[a-z0-9_-]{3,16}$")}
                error_title={Some("Use 3–16 lowercase letters, numbers, underscores, or dashes.")}
                required=true
            />
            <SearchInput
                id="language"
                label="Language"
                placeholder="Choose a language"
                items={language_options.clone()}
                required={true}
            />
            <Input id="email"    label="Email"       input_type={InputType::Email}    placeholder="Enter email" />
            <Input id="password" label="Password"    input_type={InputType::Password} placeholder="Enter password" />
            <Input id="search"   label="Search"      input_type={InputType::Search}   placeholder="Search..." />
            <ColorInput id="color" label="Pick a Color" value={"#00ffcc"} />
            <RangeInput id="range" label="Volume" min="0" max="100" step="5" default_value="50" />
            <Input id="date"     label="Date of Birth" input_type={InputType::Date} placeholder="YYYY-MM-DD" />
            <Input id="age"      label="Age"           input_type={InputType::Number} placeholder="Enter age" />
            <Input id="time"     label="Time"          input_type={InputType::Time}   placeholder="HH:MM" />
            <Textarea id="textarea" label="Description" placeholder="Write something..." />
            <Select id="select" label="Select Option" options={select_options.clone()} default_value="2" />
            <RadioGroup id="gender" label="Select Gender" options={radio_options.clone()} default_value="female" />
            <FileInput id="file_upload" label="Upload File" />
            <Checkbox id="checkbox" label="Accept Terms" />
            <PhoneInput id="phone" label="Phone" placeholder="123-456-7890" />
        </>
    }
}
