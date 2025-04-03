use crate::templates::demos::DemoComponent;
use tailyew::molecules::ModalButton;
use tailyew::organisms::table::Column;
use yew::prelude::*;

#[function_component(ModalButtonDemoSection)]
pub fn modal_button_demo_section() -> Html {
    let example = html! {
        <ModalButton
            button_text={"Open Info Modal".to_string()}
            modal_title={"Important Information".to_string()}
            modal_content={html! {
                <div class="text-sm space-y-2">
                    <p>{"This modal was triggered by a button click."}</p>
                    <p>{"You can reuse this pattern anywhere you need a quick inline modal."}</p>
                </div>
            }}
        />
    };

    let usage_code = r#"
<ModalButton
    button_text={"Open Info Modal".to_string()}
    modal_title={"Important Information".to_string()}
    modal_content={html! {
        <div class="text-sm space-y-2">
            <p>{"This modal was triggered by a button click."}</p>
            <p>{"You can reuse this pattern anywhere you need a quick inline modal."}</p>
        </div>
    }}
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
