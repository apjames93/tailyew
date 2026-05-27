use super::types::*;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub(super) struct JsonValidationSummaryProps {
    pub validity: JsonInputValidity,
}

#[component(JsonValidationSummary)]
pub(super) fn json_validation_summary(props: &JsonValidationSummaryProps) -> Html {
    let count = props.validity.errors.len();

    html! {
        <div class="rounded-md border border-red-200 bg-red-50 px-3 py-2 text-sm text-red-700 dark:border-red-900 dark:bg-red-950 dark:text-red-200" role="alert">
            <p class="font-medium">
                { format!("{} {}", count, if count == 1 { "issue needs" } else { "issues need" }) }
                { " attention" }
            </p>
            <ul class="mt-1 list-disc space-y-1 pl-5">
                { for props.validity.errors.iter().map(|error| html! {
                    <li>
                        <span class="font-medium">{ &error.path }</span>
                        { ": " }
                        { &error.message }
                    </li>
                }) }
            </ul>
        </div>
    }
}
