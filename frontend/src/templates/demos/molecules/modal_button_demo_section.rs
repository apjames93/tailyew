use crate::templates::demos::DemoComponent;
use tailyew::molecules::ModalButton;
use tailyew::molecules::ModalSize;
use tailyew::organisms::table::Column;
use tailyew::AddIcon;
use tailyew::{Button, ButtonType, TagType, Typo};
use yew::prelude::*;

use tailyew::organisms::Markdown;
const MARKDOWN_DOC: &str = include_str!("../organisms/markdown_demo.md");

#[component(ModalButtonDemoSection)]
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
        html! { <Typo>{"You confirmed the action."}</Typo> }
    } else if *declined {
        html! { <Typo tag={TagType::Error}>{"You declined the action."}</Typo> }
    } else {
        html! {}
    };

    let example = html! {
        <div class="space-y-4">
            <ModalButton
                modal_size={ModalSize::Fullscreen}
                trigger_children={html! { "Fullscreen Modal" }}
                modal_title={"Informational Modal".to_string()}
                modal_content={html! {
                    <div class="text-sm space-y-2">
                        <Typo>{"This modal was triggered by a button click and is full screen."}</Typo>
                        <Markdown content={MARKDOWN_DOC} />
                    </div>
                }}
            />


            <ModalButton
                trigger_children={html! { "Open Confirm Modal full screen With Footer button" }}
                button_type={ButtonType::Secondary}
                modal_size={ModalSize::Fullscreen}
                modal_title={"Confirm Action".to_string()}
                modal_content={html! {
                    <div class="text-sm space-y-2">
                        <Typo>{"Are you sure you want to confirm this action?"}</Typo>
                        <Markdown content={MARKDOWN_DOC} />
                    </div>
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
                trigger_children={html! { <AddIcon /> }}
                modal_title={"Informational Modal".to_string()}
                button_type={ButtonType::Danger}
                modal_content={html! {
                    <div class="text-sm space-y-2">
                        <Typo>{"This modal was triggered by a button click."}</Typo>
                        <Typo>{"You can reuse this pattern anywhere you need a quick inline modal."}</Typo>
                        <Markdown content={MARKDOWN_DOC} />
                    </div>
                }}
            />

            <ModalButton
                trigger_children={html! { "Open Confirm Modal" }}
                modal_title={"Confirm Action".to_string()}
                button_type={ButtonType::Ghost}
                modal_content={html! {
                    <div class="text-sm space-y-2">
                        <Typo>{"Are you sure you want to confirm this action?"}</Typo>
                        <Markdown content={MARKDOWN_DOC} />
                    </div>
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
                trigger_children={html! { "Open Confirm + Decline Modal" }}
                button_type={ButtonType::Button}
                modal_title={"Two-Action Modal".to_string()}
                modal_content={html! {
                    <div class="text-sm space-y-2">
                        <Markdown content={MARKDOWN_DOC} />
                        <Typo>{"Choose whether to proceed or cancel."}</Typo>
                    </div>
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
    trigger_children={html! { "Open Confirm + Decline Modal" }}
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
                "trigger_children".into(),
                "button_type".into(),
                "modal_title".into(),
                "modal_content".into(),
                "is_open".into(),
                "on_modal_close".into(),
                "footer".into(),
                "aria_label".into(),
                "aria_labelledby".into(),
                "modal_size".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "Html".into(),
                "ButtonType".into(),
                "String".into(),
                "Html".into(),
                "bool".into(),
                "Option<Callback<()>>".into(),
                "Option<Callback<Callback<()>, Html>>".into(),
                "Option<AttrValue>".into(),
                "Option<AttrValue>".into(),
                "ModalSize".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "Content for the trigger button.".into(),
                "Type of the trigger button.".into(),
                "Heading displayed in the modal.".into(),
                "Main content inside the modal.".into(),
                "Optional default open state.".into(),
                "Callback when the modal is closed.".into(),
                "Optional footer renderer; receives a close callback.".into(),
                "ARIA label for accessibility.".into(),
                "ARIA label for accessibility.".into(),
                "Size of the modal. Small, Medium, Large (default), Fullscreen.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            github_demo_path="molecules/modal_button_demo_section.rs"
            github_source_path="molecules/modal_button.rs"
            title="ModalButton Component"
            description={Some(html! {
                <Typo>{"The `ModalButton` component wraps a trigger and modal in one. You can provide custom footer buttons using a `Callback<Callback<()>>` that receives a modal-closing function."}</Typo>
            })}
            example={example}
            usage_code={usage_code}
            props_table={Some(props_table)}
        />
    }
}
