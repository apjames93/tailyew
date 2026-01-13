use crate::templates::demos::DemoComponent;
use tailyew::atoms::{Button, ButtonType, Typo};
use tailyew::molecules::modal::{Modal, ModalSize};
use tailyew::organisms::table::Column;
use yew::prelude::*;

#[component(ModalDemoSection)]
pub fn modal_demo_section() -> Html {
    let show_modal = use_state(|| false);
    let toggle_modal = {
        let show_modal = show_modal.clone();
        Callback::from(move |_| show_modal.set(true))
    };
    let close_modal = {
        let show_modal = show_modal.clone();
        Callback::from(move |_| show_modal.set(false))
    };

    let example = html! {
        <>
            <Button button_type={ButtonType::Primary} on_click={toggle_modal.clone()}>
                { "Open Modal" }
            </Button>

            <Modal
                title="Example Modal"
                is_open={*show_modal}
                on_close={close_modal}
                size={ModalSize::Medium}
            >
                <p>{ "This is a reusable modal component. Click outside, press Escape, or the X icon to close." }</p>
            </Modal>
        </>
    };

    let usage_code = r#"
let show_modal = use_state(|| false);
let toggle_modal = {
    let show_modal = show_modal.clone();
    Callback::from(move |_| show_modal.set(true))
};
let close_modal = {
    let show_modal = show_modal.clone();
    Callback::from(move |_| show_modal.set(false))
};

html! {
    <>
        <Button button_type={ButtonType::Primary} on_click={toggle_modal}>
            { "Open Modal" }
        </Button>

        <Modal
            title="Example Modal"
            is_open={*show_modal}
            on_close={close_modal}
            size={ModalSize::Medium}
        >
            <p>{ "This is a reusable modal component." }</p>
        </Modal>
    </>
};
"#;

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "title".into(),
                "is_open".into(),
                "on_close".into(),
                "children".into(),
                "size".into(),
                "aria_label".into(),
                "aria_labelledby".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "String".into(),
                "bool".into(),
                "Callback<()>".into(),
                "Children".into(),
                "ModalSize".into(),
                "Option<AttrValue>".into(),
                "Option<AttrValue>".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "Title displayed at the top of the modal.".into(),
                "Whether the modal is open.".into(),
                "Called when the modal is closed (click, ESC, X icon).".into(),
                "Content inside the modal.".into(),
                "`Small`, `Medium`, or `Large` modal width.".into(),
                "ARIA label for accessibility.".into(),
                "ARIA label for accessibility.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            github_demo_path="molecules/modal_demo_section.rs"
            github_source_path="molecules/modal.rs"
            title="Modal Component"
            description={Some(html! {
                <Typo>{"The `Modal` component creates a centered dialog with optional close behavior via overlay, escape key, or close button. It supports configurable sizes and can be styled freely inside."}</Typo>
            })}
            example={example}
            usage_code={usage_code}
            props_table={Some(props_table)}
        />
    }
}
