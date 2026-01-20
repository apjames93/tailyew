use crate::templates::demos::DemoComponent;
use tailyew::organisms::table::Column;
use tailyew::{Button, ButtonType, Notification, NotificationTypes};
use yew::prelude::*;

#[component(NotificationDemoSection)]
pub fn notification_demo_section() -> Html {
    let show_success = use_state(|| true);
    let show_error = use_state(|| true);
    let show_info = use_state(|| true);
    let show_warning = use_state(|| true);

    let on_show_all = {
        let show_success = show_success.clone();
        let show_error = show_error.clone();
        let show_info = show_info.clone();
        let show_warning = show_warning.clone();
        Callback::from(move |_| {
            show_success.set(true);
            show_error.set(true);
            show_info.set(true);
            show_warning.set(true);
        })
    };

    let example = html! {
        <div class="space-y-4">
            <Button button_type={ButtonType::Primary} on_click={on_show_all.clone()}>
                { "Show All Notifications" }
            </Button>

            <Notification
                message="Action completed successfully!"
                notification_type={NotificationTypes::Success}
                visible={*show_success}
                on_close={Some(Callback::from({
                    let show_success = show_success.clone();
                    move |_| show_success.set(false)
                }))}
                fixed={false}
            />

            <Notification
                message="An unexpected error occurred while processing your request. This may be due to a server timeout, invalid input, or a permissions issue. Please review your form data, check your internet connection, and try again. If this issue continues, contact technical support with the error code #ERR-78291."
                notification_type={NotificationTypes::Error}
                visible={*show_error}
                on_close={Some(Callback::from({
                    let show_error = show_error.clone();
                    move |_| show_error.set(false)
                }))}
                fixed={false}
                max_width={"max-w-full"}
            />

            <Notification
                message="This is some informational text."
                notification_type={NotificationTypes::Info}
                visible={*show_info}
                on_close={Some(Callback::from({
                    let show_info = show_info.clone();
                    move |_| show_info.set(false)
                }))}
                fixed={false}
                max_width={"max-w-full"}
            />

            <Notification
                message="This is your last warning!"
                notification_type={NotificationTypes::Warning}
                visible={*show_warning}
                on_close={Some(Callback::from({
                    let show_warning = show_warning.clone();
                    move |_| show_warning.set(false)
                }))}
                fixed={false}
                max_width={"max-w-full"}
            />
        </div>
    };

    let usage_code = r#"
<Button on_click={on_show_all} button_type={ButtonType::Primary}>
    "Show All Notifications"
</Button>

<Notification
    message="This is your last warning!"
    notification_type={NotificationTypes::Warning}
    visible={show_warning}
    on_close={Some(on_close_cb)}
    fixed={false}
    max_width={"max-w-full"}
/>
"#;

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "message",
                "notification_type",
                "visible",
                "show_close",
                "on_close",
                "max_width",
                "fixed",
            ]
            .into_iter()
            .map(|s| html! { s })
            .collect(),
        },
        Column {
            header: "Type".into(),
            values: vec![
                "String",
                "NotificationTypes",
                "bool",
                "bool",
                "Option<Callback<()>>",
                "String",
                "bool",
            ]
            .into_iter()
            .map(|s| html! { s })
            .collect(),
        },
        Column {
            header: "Description".into(),
            values: vec![
                "Text content to display.",
                "Visual variant (Success, Error, etc).",
                "Controls visibility of the banner.",
                "Whether a close button is shown.",
                "Callback when the banner is dismissed.",
                "Tailwind max-width classes for the banner.",
                "Fixed vs relative positioning.",
            ]
            .into_iter()
            .map(|s| html! { s })
            .collect(),
        },
    ];

    html! {
        <DemoComponent
            github_demo_path="molecules/notification_demo_section.rs"
            github_source_path="molecules/notification.rs"
            title="Notification Component"
            description={Some(html! {
                <p>{"The `Notification` component displays contextual messages like success, error, or warning. It supports full-width layout, optional dismissal, and a ‘show all’ invocation."}</p>
            })}
            example={example}
            usage_code={usage_code}
            props_table={Some(props_table)}
        />
    }
}
