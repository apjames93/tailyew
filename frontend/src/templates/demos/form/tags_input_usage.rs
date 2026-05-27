use serde_json::Value;
use tailyew::form::{Form, TagsInput, async_callback, e_input_value};
use web_sys::SubmitEvent;
use yew::prelude::*;

#[component(TagsInputExample)]
pub fn tags_input_example() -> Html {
    let submitted_json = use_state(|| Value::Null);

    let onsubmit = async_callback({
        let submitted_json = submitted_json.clone();
        move |e: SubmitEvent| {
            let submitted_json = submitted_json.clone();
            async move {
                let raw_json = e_input_value("release_tags", &e);
                let value: Value = serde_json::from_str(&raw_json)
                    .map_err(|_| "TagsInput submitted invalid JSON".to_owned())?;

                submitted_json.set(value);
                Ok(None)
            }
        }
    });

    html! {
        <Form onsubmit_callback={onsubmit} button_label={"Submit tags".to_owned()}>
            <TagsInput
                id="release_tags_editor"
                name="release_tags"
                label="Release tags"
                helper_text={Some("Submitted as a JSON array of strings.")}
                initial_tags={vec!["beta".to_owned(), "internal".to_owned()]}
                allow_custom_tags={false}
                suggestions={vec![
                    "stable".to_owned(),
                    "beta".to_owned(),
                    "internal".to_owned(),
                    "customer-facing".to_owned(),
                ]}
            />

            if *submitted_json != Value::Null {
                <pre>{ serde_json::to_string_pretty(&*submitted_json).unwrap() }</pre>
            }
        </Form>
    }
}
