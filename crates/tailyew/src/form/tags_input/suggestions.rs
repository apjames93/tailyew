use crate::{Button, ButtonSize, ButtonType};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub(super) struct TagsSuggestionsProps {
    pub suggestions: Vec<String>,
    pub tags: Vec<String>,
    pub can_add: bool,
    pub allow_duplicates: bool,
    pub on_add: Callback<String>,
}

#[component(TagsSuggestions)]
pub(super) fn tags_suggestions(props: &TagsSuggestionsProps) -> Html {
    if props.suggestions.is_empty() {
        return html! {};
    }

    html! {
        <div class="flex flex-wrap items-center gap-2">
            <span class="text-xs font-medium text-gray-500 dark:text-gray-400">{ "Suggestions" }</span>
            { for props.suggestions.iter().map(|suggestion| {
                let suggestion = suggestion.clone();
                let disabled = !props.can_add
                    || (!props.allow_duplicates && props.tags.contains(&suggestion));
                let on_add = props.on_add.clone();
                let suggestion_for_click = suggestion.clone();
                let on_click = Callback::from(move |_| {
                    on_add.emit(suggestion_for_click.clone());
                });

                html! {
                    <Button
                        button_type={ButtonType::Ghost}
                        size={ButtonSize::Small}
                        disabled={disabled}
                        on_click={on_click}
                        class="h-8 shadow-none"
                    >
                        <span>{ suggestion }</span>
                    </Button>
                }
            }) }
        </div>
    }
}
