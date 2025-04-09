use crate::templates::demos::DemoComponent;
use tailyew::molecules::ModalButton;
use tailyew::organisms::table::Column;
use tailyew::ButtonType;
use tailyew::Typo;
use yew::prelude::*;

#[function_component(ModalButtonDemoSection)]
pub fn modal_button_demo_section() -> Html {
    let confirmed = use_state(|| false);

    let confirm_message = if *confirmed {
        html! { <Typo class="text-green-600 text-sm font-medium">{"You confirmed the action!"}</Typo> }
    } else {
        html! {}
    };

    let on_confirm = {
        let confirmed = confirmed.clone();
        Callback::from(move |_| {
            confirmed.set(true);
        })
    };

    let example = html! {
        <div class="space-y-4">
            <ModalButton
                button_text={"Open Info Modal".to_string()}
                modal_title={"Informational Modal".to_string()}
                modal_content={html! {
                    <div class="text-sm space-y-2">
                        <Typo>{"This modal was triggered by a button click."}</Typo>
                        <Typo>{"You can reuse this pattern anywhere you need a quick inline modal."}</Typo>
                    </div>
                }}
            />

            <ModalButton
                button_text={"Open Confirm Modal".to_string()}
                button_type={ButtonType::Primary}
                modal_title={"Confirm Action".to_string()}
                modal_content={html! {
                    <Typo class="text-sm">{"Are you sure you want to confirm this action?"}</Typo>
                }}
                on_confirm_click={Some(on_confirm)}
                confirm_button_text={"Confirm"}
            />

            { confirm_message }
        </div>
    };

    let usage_code = r#"
let confirmed = use_state(|| false);

let on_confirm = {
    let confirmed = confirmed.clone();
    Callback::from(move |_| {
        confirmed.set(true);
    })
};

<ModalButton
    button_text={"Open Confirm Modal".to_string()}
    modal_title={"Confirm Action".to_string()}
    modal_content={html! {
        <p>{"Are you sure you want to confirm this action?"}</p>
    }}
    on_confirm_click={Some(on_confirm)}
    confirm_button_text={"Confirm".into()}
/>
"#;

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "button_text".into(),
                "modal_title".into(),
                "modal_content".into(),
                "on_modal_close".into(),
                "on_confirm_click".into(),
                "confirm_button_text".into(),
                "confirm_button_type".into(),
                "confirm_disabled".into(),
                "is_open".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "String".into(),
                "String".into(),
                "Html".into(),
                "Option<Callback<()>>".into(),
                "Option<Callback<()>>".into(),
                "String".into(),
                "ButtonType".into(),
                "bool".into(),
                "bool".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "Label for the trigger button.".into(),
                "Heading shown in the modal.".into(),
                "Body content of the modal.".into(),
                "Callback when the modal is closed.".into(),
                "Callback fired on confirm button click.".into(),
                "Label for the confirm button.".into(),
                "Type of confirm button (Primary, Danger, etc).".into(),
                "Disables the confirm button.".into(),
                "Initial open state (default: false).".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            title="ModalButton Component"
            description={Some(html! {
                <p>{"The `ModalButton` component wraps a `Button` and `Modal` into a convenient toggleable unit. Great for confirmations, extra info, or inline modals."}</p>
            })}
            example={example}
            usage_code={usage_code}
            props_table={Some(props_table)}
        />
    }
}
