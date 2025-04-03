use crate::templates::demos::DemoComponent;
use tailyew::atoms::{Button, ButtonType, TagType, Typo};
use tailyew::molecules::Popover;
use tailyew::organisms::table::Column;
use yew::prelude::*;

#[function_component(PopoverDemoSection)]
pub fn popover_demo_section() -> Html {
    let show_popup = use_state(|| false);

    let toggle_popover = {
        let show_popup = show_popup.clone();
        Callback::from(move |_| show_popup.set(!*show_popup))
    };

    let close_popover = {
        let show_popup = show_popup.clone();
        Callback::from(move |_| show_popup.set(false))
    };

    let example = html! {
        <div class="text-center space-y-8">
            <Popover
                trigger={html! {
                    <Button
                        button_type={ButtonType::Primary}
                        onclick={toggle_popover.clone()}
                    >
                        { "Open Popover" }
                    </Button>
                }}
                content={html! {
                    <div class="text-gray-800 dark:text-gray-300 space-y-2">
                        <Typo tag={TagType::H3}>{ "Popover Content" }</Typo>
                        <Typo tag={TagType::P}>{ "This is an interactive popover. You can place any Yew component inside here." }</Typo>
                    </div>
                }}
                is_open={*show_popup}
                on_close={Some(close_popover)}
            />
        </div>
    };

    let usage_code = r#"
let show_popup = use_state(|| false);

let toggle_popover = {
    let show_popup = show_popup.clone();
    Callback::from(move |_| show_popup.set(!*show_popup))
};

let close_popover = {
    let show_popup = show_popup.clone();
    Callback::from(move |_| show_popup.set(false))
};

html! {
    <Popover
        trigger={html! {
            <Button button_type={ButtonType::Primary} onclick={toggle_popover}>
                { "Open Popover" }
            </Button>
        }}
        content={html! {
            <div class="text-gray-800 dark:text-gray-300">
                <p>{ "This is an interactive popover." }</p>
            </div>
        }}
        is_open={*show_popup}
        on_close={Some(close_popover)}
    />
};
"#;

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "trigger".into(),
                "content".into(),
                "is_open".into(),
                "on_close".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "Html".into(),
                "Html".into(),
                "bool".into(),
                "Option<Callback<MouseEvent>>".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "Element that triggers the popover when clicked.".into(),
                "Content shown inside the popover.".into(),
                "Controls visibility of the popover.".into(),
                "Called when popover is dismissed (outside click or ✕).".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            title="Popover Component"
            description={Some(html! {
                <p>{"The `Popover` component displays content when triggered by another element. Useful for tooltips, menus, or additional info panels."}</p>
            })}
            example={example}
            usage_code={usage_code}
            props_table={Some(props_table)}
        />
    }
}
