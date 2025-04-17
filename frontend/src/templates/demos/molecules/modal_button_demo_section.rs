use crate::templates::demos::DemoComponent;
use tailyew::molecules::ModalButton;
use tailyew::organisms::table::Column;
use tailyew::{Button, ButtonType, Typo};
use yew::prelude::*;

#[function_component(ModalButtonDemoSection)]
pub fn modal_button_demo_section() -> Html {
    let confirmed = use_state(|| false);
    let declined = use_state(|| false);

    let on_confirm = {
        let confirmed = confirmed.clone();
        let declined = declined.clone();
        Callback::from(move |_| {
            confirmed.set(true);
            declined.set(false);
        })
    };

    let on_decline = {
        let confirmed = confirmed.clone();
        let declined = declined.clone();
        Callback::from(move |_| {
            declined.set(true);
            confirmed.set(false);
        })
    };

    let feedback_message = if *confirmed {
        html! { <Typo class="text-green-600 text-sm font-medium">{"✅ You confirmed the action."}</Typo> }
    } else if *declined {
        html! { <Typo class="text-red-600 text-sm font-medium">{"❌ You declined the action."}</Typo> }
    } else {
        html! {}
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
                footer={Some({
                    let on_confirm = on_confirm.clone();
                    Callback::from(move |close_modal: Callback<()>| {
                        html! {
                            <Button
                                button_type={ButtonType::Primary}
                                onclick={{
                                    let close_modal = close_modal.clone();
                                    let on_confirm = on_confirm.clone();
                                    Callback::from(move |_| {
                                        on_confirm.emit(());
                                        close_modal.emit(());
                                    })
                                }}
                            >
                                { "Confirm" }
                            </Button>
                        }
                    })
                })}
            />

            <ModalButton
                button_text={"Open Confirm + Decline Modal".to_string()}
                button_type={ButtonType::Secondary}
                modal_title={"Two-Action Modal".to_string()}
                modal_content={html! {
                    <Typo class="text-sm">{"Choose whether to proceed or cancel."}</Typo>
                }}
                footer={Some({
                    let on_confirm = on_confirm.clone();
                    let on_decline = on_decline.clone();
                    Callback::from(move |close_modal: Callback<()>| {
                        html! {
                            <>
                                <Button
                                    button_type={ButtonType::Secondary}
                                    onclick={{
                                        let close_modal = close_modal.clone();
                                        let on_decline = on_decline.clone();
                                        Callback::from(move |_| {
                                            on_decline.emit(());
                                            close_modal.emit(());
                                        })
                                    }}
                                >
                                    { "Decline" }
                                </Button>

                                <Button
                                    button_type={ButtonType::Danger}
                                    onclick={{
                                        let close_modal = close_modal.clone();
                                        let on_confirm = on_confirm.clone();
                                        Callback::from(move |_| {
                                            on_confirm.emit(());
                                            close_modal.emit(());
                                        })
                                    }}
                                >
                                    { "Confirm" }
                                </Button>
                            </>
                        }
                    })
                })}
            />

            { feedback_message }
        </div>
    };

    let usage_code = r#"
<ModalButton
    button_text={"Open Confirm + Decline Modal".to_string()}
    modal_title={"Two-Action Modal".to_string()}
    modal_content={html! { <p>{"Choose whether to proceed or cancel."}</p> }}
    footer={Some(Callback::from(move |close_modal: Callback<()>| {
        html! {
            <>
                <Button button_type={ButtonType::Secondary} onclick={...}>{"Decline"}</Button>
                <Button button_type={ButtonType::Danger} onclick={...}>{"Confirm"}</Button>
            </>
        }
    }))}
 />
"#;

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "button_text".into(),
                "button_type".into(),
                "modal_title".into(),
                "modal_content".into(),
                "is_open".into(),
                "on_modal_close".into(),
                "footer".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "String".into(),
                "ButtonType".into(),
                "String".into(),
                "Html".into(),
                "bool".into(),
                "Option<Callback<()>>".into(),
                "Option<Callback<Callback<()>>>".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "Text for the trigger button.".into(),
                "Type of the trigger button.".into(),
                "Heading displayed in the modal.".into(),
                "Main content inside the modal.".into(),
                "Optional default open state.".into(),
                "Callback when the modal is closed.".into(),
                "Optional footer renderer; receives a close callback.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            title="ModalButton Component"
            description={Some(html! {
                <p>{"The `ModalButton` component wraps a trigger and modal in one. You can provide custom footer buttons using a `Callback<Callback<()>>` that receives a modal-closing function."}</p>
            })}
            example={example}
            usage_code={usage_code}
            props_table={Some(props_table)}
        />
    }
}
