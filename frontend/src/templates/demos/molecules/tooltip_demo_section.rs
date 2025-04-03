use crate::templates::demos::DemoComponent;
use tailyew::atoms::{Button, ButtonType};
use tailyew::molecules::{Tooltip, TooltipPosition};
use tailyew::organisms::table::Column;
use yew::prelude::*;

#[function_component(TooltipDemoSection)]
pub fn tooltip_demo_section() -> Html {
    let example = html! {
        <div class="flex flex-wrap gap-6 justify-center py-8">
            <Tooltip
                trigger={html! { <Button button_type={ButtonType::Secondary}>{ "Hover Me (Top)" }</Button> }}
                content={html! { "Tooltip above" }}
                position={TooltipPosition::Top}
            />

            <Tooltip
                trigger={html! { <Button button_type={ButtonType::Secondary}>{ "Hover Me (Right)" }</Button> }}
                content={html! { "Tooltip right" }}
                position={TooltipPosition::Right}
            />

            <Tooltip
                trigger={html! { <Button button_type={ButtonType::Secondary}>{ "Hover Me (Bottom)" }</Button> }}
                content={html! { "Tooltip below" }}
                position={TooltipPosition::Bottom}
            />

            <Tooltip
                trigger={html! { <Button button_type={ButtonType::Secondary}>{ "Hover Me (Left)" }</Button> }}
                content={html! { "Tooltip left" }}
                position={TooltipPosition::Left}
            />
        </div>
    };

    let usage_code = r#"
<Tooltip
    trigger={html! {
        <Button button_type={ButtonType::Secondary}>
            { "Hover Me" }
        </Button>
    }}
    content={html! { "Tooltip message" }}
    position={TooltipPosition::Top}
/>
"#;

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec!["trigger".into(), "content".into(), "position".into()],
        },
        Column {
            header: "Type".into(),
            values: vec!["Html".into(), "Html".into(), "TooltipPosition".into()],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "Element that activates the tooltip on hover.".into(),
                "Text or HTML displayed in the tooltip.".into(),
                "Position of the tooltip relative to the trigger.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            title="Tooltip Component"
            description={Some(html! {
                <p>{ "The `Tooltip` component appears when hovering over a trigger element. You can configure its position (top, right, bottom, left)." }</p>
            })}
            example={example}
            usage_code={usage_code}
            props_table={Some(props_table)}
        />
    }
}
