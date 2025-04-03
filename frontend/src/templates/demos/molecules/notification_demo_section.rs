use crate::templates::demos::DemoComponent;
use tailyew::molecules::{Notification, NotificationTypes};
use tailyew::organisms::table::Column;
use yew::prelude::*;

#[function_component(NotificationDemoSection)]
pub fn notification_demo_section() -> Html {
    let show_success = use_state(|| true);
    let show_error = use_state(|| true);
    let show_info = use_state(|| true);
    let show_warning = use_state(|| true);

    let example = html! {
        <div class="space-y-4">
            if *show_success {
                <Notification
                    message="Action completed successfully!"
                    notification_type={NotificationTypes::Success}
                    visible={true}
                    on_close={Some(Callback::from({
                        let show_success = show_success.clone();
                        move |_| show_success.set(false)
                    }))}
                    fixed={false}
                />
            }

            if *show_error {
                <Notification
                    message="Something went wrong."
                    notification_type={NotificationTypes::Error}
                    visible={true}
                    on_close={Some(Callback::from({
                        let show_error = show_error.clone();
                        move |_| show_error.set(false)
                    }))}
                    fixed={false}
                />
            }

            if *show_info {
                <Notification
                    message="This is some informational text."
                    notification_type={NotificationTypes::Info}
                    visible={true}
                    on_close={Some(Callback::from({
                        let show_info = show_info.clone();
                        move |_| show_info.set(false)
                    }))}
                    fixed={false}
                />
            }

            if *show_warning {
                <Notification
                    message="This is your last warning!"
                    notification_type={NotificationTypes::Warning}
                    visible={true}
                    on_close={Some(Callback::from({
                        let show_warning = show_warning.clone();
                        move |_| show_warning.set(false)
                    }))}
                    fixed={false}
                />
            }
        </div>
    };

    let usage_code = r#"
        <Notification
            message="This is your last warning!"
            notification_type={NotificationTypes::Warning}
            visible={true}
            on_close={Some(Callback::from({
                let show_warning = show_warning.clone();
                move |_| show_warning.set(false)
            }))}
            fixed={false}
        />
"#;

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "message".into(),
                "notification_type".into(),
                "visible".into(),
                "on_close".into(),
                "fixed".into(),
            ],
        },
        Column {
            header: "Type".into(),
            values: vec![
                "String".into(),
                "NotificationTypes".into(),
                "bool".into(),
                "Option<Callback<()>>".into(),
                "bool".into(),
            ],
        },
        Column {
            header: "Description".into(),
            values: vec![
                "Text content to display.".into(),
                "Visual variant (Success, Error, etc).".into(),
                "Initial visibility of the banner.".into(),
                "Optional callback when dismissed.".into(),
                "Whether the banner is absolutely positioned.".into(),
            ],
        },
    ];

    html! {
        <DemoComponent
            title="Notification Component"
            description={Some(html! {
                <p>{"The `Notification` component displays contextual messages like success, error, or warning. You can optionally dismiss them and trigger custom callbacks on close."}</p>
            })}
            example={example}
            usage_code={usage_code}
            props_table={Some(props_table)}
        />
    }
}
