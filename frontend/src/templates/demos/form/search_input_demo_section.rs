use crate::templates::demos::DemoComponent;
use gloo_net::http::Request;
use serde::Deserialize;
use tailyew::atoms::Typo;
use tailyew::form::{SearchInput, SelectOption};
use tailyew::organisms::table::Column;
use yew::prelude::*;

const USAGE_CODE: &str = r#"
let suggestions = Rc::new(vec![
    SelectOption { label: "Rust".into(), value: "rust".into() },
    SelectOption { label: "Go".into(), value: "go".into() },
    SelectOption { label: "TypeScript".into(), value: "typescript".into() },
]);

let selected = use_state(|| "".to_string());
let on_select = {
    let selected = selected.clone();
    Callback::from(move |val: String| selected.set(val))
};

html! {
    <SearchInput
        items={suggestions}
        debounce_ms={250}
        on_select={Some(on_select)}
        id="language"
        label="Language"
        placeholder="Type a language"
        required={true}
        default_selected={Some(SelectOption { label: "Rust".into(), value: "rust".into() })}
        no_autocomplete={true}       // ← new prop
    />
}
"#;

#[derive(Deserialize, Debug)]
struct PokemonResult {
    name: String,
}

#[derive(Deserialize, Debug)]
struct PokemonResponse {
    results: Vec<PokemonResult>,
}

#[component(SearchInputDemoSection)]
pub fn search_input_demo_section() -> Html {
    // 1. Static language example
    let selected = use_state(|| "".to_string());
    let suggestions = vec![
        SelectOption {
            label: "Rust".into(),
            value: "rust".into(),
        },
        SelectOption {
            label: "Go".into(),
            value: "go".into(),
        },
        SelectOption {
            label: "TypeScript".into(),
            value: "typescript".into(),
        },
    ];

    let on_select = {
        let selected = selected.clone();
        Callback::from(move |val: String| {
            web_sys::console::log_1(&format!("Selected language: {}", val).into());
            selected.set(val);
        })
    };

    // 2. Dynamic Pokémon API example
    let poke_selected = use_state(|| "".to_string());
    let poke_items = use_state(|| {
        vec![
            SelectOption {
                label: "bulbasaur".into(),
                value: "bulbasaur".into(),
            },
            SelectOption {
                label: "charmander".into(),
                value: "charmander".into(),
            },
        ]
    });

    let on_poke_select = {
        let poke_selected = poke_selected.clone();
        Callback::from(move |val: String| {
            web_sys::console::log_1(&format!("Selected Pokémon: {}", val).into());
            poke_selected.set(val);
        })
    };

    let on_fetch_more = {
        let poke_items = poke_items.clone();
        Callback::from(move |_| {
            let poke_items = poke_items.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(resp) = Request::get("https://pokeapi.co/api/v2/pokemon?limit=20")
                    .send()
                    .await
                    && let Ok(parsed) = resp.json::<PokemonResponse>().await
                {
                    let new_items = parsed
                        .results
                        .into_iter()
                        .map(|p| SelectOption {
                            label: p.name.clone(),
                            value: p.name,
                        })
                        .collect::<Vec<_>>();
                    let mut all = (*poke_items).clone();
                    all.extend(new_items);
                    poke_items.set(all);
                }
            });
        })
    };

    let example = html! {
        <div class="space-y-8">
            <div>
                <Typo class="font-bold text-lg mb-2">{"Static Language Example"}</Typo>
                <SearchInput
                    items={suggestions.clone()}
                    debounce_ms={250}
                    on_select={Some(on_select.clone())}
                    id="language"
                    label="Language"
                    placeholder="Type a language"
                    required={true}
                    default_selected={Some(SelectOption { label: "Rust".into(), value: "rust".into() })}
                />
                <p class="text-sm text-gray-600 dark:text-gray-300 mt-2">
                    { format!("Selected: {}", *selected) }
                </p>
            </div>

            <div>
                <Typo class="font-bold text-lg mb-2">{"Disabled Example"}</Typo>
                <SearchInput
                    items={suggestions.clone()}
                    debounce_ms={250}
                    on_select={Some(on_select.clone())}
                    id="language"
                    label="Language"
                    placeholder="Type a language"
                    required={true}
                    default_selected={Some(SelectOption { label: "Rust".into(), value: "rust".into() })}
                    disabled={true}
                />
                <p class="text-sm text-gray-600 dark:text-gray-300 mt-2">
                    { format!("Selected: {}", *selected) }
                </p>
            </div>

            <div>
                <Typo class="font-bold text-lg mb-2">{"Dynamic Pokémon API Example"}</Typo>
                <SearchInput
                    items={(*poke_items).clone()}
                    debounce_ms={300}
                    on_select={Some(on_poke_select.clone())}
                    on_fetch_more={Some(on_fetch_more.clone())}
                    id="pokemon"
                    label="Pokémon"
                    placeholder="Search for a Pokémon"
                />
                <Typo class="text-xs text-gray-500 dark:text-gray-400 mt-1">
                    {"If no matches are found, this will load more results from the API."}
                </Typo>
                <p class="text-sm text-gray-600 dark:text-gray-300 mt-2">
                    { format!("Selected: {}", *poke_selected) }
                </p>
            </div>
        </div>
    };

    let props_table = vec![
        Column {
            header: "Prop".into(),
            values: vec![
                "items",
                "on_select",
                "on_fetch_more",
                "debounce_ms",
                "id",
                "label",
                "placeholder",
                "default_selected",
                "required",
                "class",
                "aria_label",
                "aria_labelledby",
                "aria_describedby",
                "error_title",
                "disabled",
            ]
            .into_iter()
            .map(Html::from)
            .collect(),
        },
        Column {
            header: "Type".into(),
            values: vec![
                "Rc<Vec<SelectOption>>",
                "Option<Callback<String>>",
                "Option<Callback<()>>",
                "u32",
                "Option<AttrValue>",
                "Option<AttrValue>",
                "Option<AttrValue>",
                "Option<SelectOption>",
                "bool",
                "Classes",
                "Option<AttrValue>",
                "Option<AttrValue>",
                "Option<AttrValue>",
                "AttrValue",
                "bool",
                "bool",
            ]
            .into_iter()
            .map(Html::from)
            .collect(),
        },
        Column {
            header: "Description".into(),
            values: vec![
                "List of selectable items",
                "Callback when an item is chosen",
                "Called when no matches are found",
                "Debounce interval (ms)",
                "Input ID/name",
                "Visible label",
                "Placeholder text",
                "Initial selected item",
                "Marks input required",
                "Custom CSS classes",
                "ARIA label",
                "ARIA labelledby",
                "ARIA describedby",
                "Validation error title",
                "Disable the input",
            ]
            .into_iter()
            .map(Html::from)
            .collect(),
        },
    ];

    html! {
        <DemoComponent
            github_demo_path="form/search_input_demo_section.rs"
            github_source_path="form/search_input.rs"
            title="SearchInput Component"
            description={Some(html! {
                <p>
                    {"`SearchInput` is a debounced autocomplete text field with dynamic loading, full Tailwind & ARIA support, plus `disabled` and `no_autocomplete` flags."}
                </p>
            })}
            example={example}
            usage_code={USAGE_CODE}
            props_table={Some(props_table)}
        />
    }
}
