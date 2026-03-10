use crate::templates::demos::DemoComponent;
use tailyew::form::{Input, InputType, Label};
use tailyew::organisms::table::Column;
use yew::prelude::*;

const USAGE_CODE: &str = r#"
html! {
    <div class="space-y-4 max-w-md mx-auto">
        <Label for_id="email" text="Email" />

        <Label for_id="password" text="Password" required=true />
        <Input
            id="password"
            label=""
            input_type={InputType::Password}
            placeholder="Enter your password"
            required=true
        />

        <Label
            for_id="team"
            text="Team Name"
            class={classes!("text-emerald-700", "dark:text-emerald-300")}
        />
        <Input
            id="team"
            label=""
            input_type={InputType::Text}
            placeholder="TailYew Core"
        />
    </div>
}
"#;

#[component(LabelDemoSection)]
pub fn label_demo_section() -> Html {
    let example = html! {
        <div class="space-y-4 max-w-md mx-auto">
            <Label for_id="label-demo-email" text="Email" />

            <Label for_id="label-demo-password" text="Password" required=true />
            <Input
                id="label-demo-password"
                label=""
                input_type={InputType::Password}
                placeholder="Enter your password"
                required=true
            />

            <Label
                for_id="label-demo-team"
                text="Team Name"
                class={classes!("text-emerald-700", "dark:text-emerald-300")}
            />
            <Input
                id="label-demo-team"
                label=""
                input_type={InputType::Text}
                placeholder="TailYew Core"
            />
        </div>
    };

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec!["id", "for_id", "text", "required", "class"]
                .into_iter()
                .map(Html::from)
                .collect(),
        },
        Column {
            header: "Type".into(),
            values: vec![
                "Option<AttrValue>",
                "AttrValue",
                "AttrValue",
                "bool",
                "Classes",
            ]
            .into_iter()
            .map(Html::from)
            .collect(),
        },
        Column {
            header: "Description".into(),
            values: vec![
                "Optional id for the rendered label element.",
                "Target input id for the label's `for` attribute.",
                "Visible label text.",
                "Shows a trailing `*` indicator when true.",
                "Optional class overrides.",
            ]
            .into_iter()
            .map(Html::from)
            .collect(),
        },
    ];

    html! {
        <DemoComponent
            github_demo_path="form/label_demo_section.rs"
            github_source_path="form/label.rs"
            title="Label Component"
            description={Some(html! {
                <p>{"`Label` is a reusable form label primitive that standardizes typography and required indicators across TailYew form controls."}</p>
            })}
            example={example}
            usage_code={USAGE_CODE}
            props_table={Some(props_table)}
        />
    }
}
