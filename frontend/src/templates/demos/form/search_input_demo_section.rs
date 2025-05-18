// SearchInputDemoSection
// Provides both static and dynamic demos for the `SearchInput` component

use crate::templates::demos::DemoComponent;
use gloo_net::http::Request;
use serde::Deserialize;
use std::rc::Rc;
use tailyew::atoms::Typo;
use tailyew::form::search_input::{Item, SearchInput};
use tailyew::organisms::table::Column;
use yew::prelude::*;

const USAGE_CODE: &str = r#"
let suggestions = Rc::new(vec![
    Item { label: "Rust".into(), value: "rust".into() },
    Item { label: "Go".into(), value: "go".into() },
    Item { label: "TypeScript".into(), value: "typescript".into() },
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

#[function_component(SearchInputDemoSection)]
pub fn search_input_demo_section() -> Html {
    // Static example — preloaded values
    let selected = use_state(|| "".to_string());
    let suggestions = Rc::new(vec![
        Item {
            label: "Rust".into(),
            value: "rust".into(),
        },
        Item {
            label: "Go".into(),
            value: "go".into(),
        },
        Item {
            label: "TypeScript".into(),
            value: "typescript".into(),
        },
        Item {
            label: "Python".into(),
            value: "python".into(),
        },
        Item {
            label: "JavaScript".into(),
            value: "javascript".into(),
        },
        Item {
            label: "Java".into(),
            value: "java".into(),
        },
        Item {
            label: "C".into(),
            value: "c".into(),
        },
        Item {
            label: "C++".into(),
            value: "c++".into(),
        },
        Item {
            label: "C#".into(),
            value: "csharp".into(),
        },
        Item {
            label: "Kotlin".into(),
            value: "kotlin".into(),
        },
        Item {
            label: "Swift".into(),
            value: "swift".into(),
        },
        Item {
            label: "Objective-C".into(),
            value: "objective-c".into(),
        },
        Item {
            label: "Dart".into(),
            value: "dart".into(),
        },
        Item {
            label: "Ruby".into(),
            value: "ruby".into(),
        },
        Item {
            label: "Elixir".into(),
            value: "elixir".into(),
        },
        Item {
            label: "Erlang".into(),
            value: "erlang".into(),
        },
        Item {
            label: "Scala".into(),
            value: "scala".into(),
        },
        Item {
            label: "Haskell".into(),
            value: "haskell".into(),
        },
        Item {
            label: "OCaml".into(),
            value: "ocaml".into(),
        },
        Item {
            label: "F#".into(),
            value: "fsharp".into(),
        },
        Item {
            label: "Perl".into(),
            value: "perl".into(),
        },
        Item {
            label: "Lua".into(),
            value: "lua".into(),
        },
        Item {
            label: "Shell".into(),
            value: "shell".into(),
        },
        Item {
            label: "Bash".into(),
            value: "bash".into(),
        },
        Item {
            label: "Zig".into(),
            value: "zig".into(),
        },
        Item {
            label: "Nim".into(),
            value: "nim".into(),
        },
        Item {
            label: "Crystal".into(),
            value: "crystal".into(),
        },
        Item {
            label: "R".into(),
            value: "r".into(),
        },
        Item {
            label: "MATLAB".into(),
            value: "matlab".into(),
        },
        Item {
            label: "Julia".into(),
            value: "julia".into(),
        },
        Item {
            label: "SQL".into(),
            value: "sql".into(),
        },
        Item {
            label: "PL/SQL".into(),
            value: "plsql".into(),
        },
        Item {
            label: "Assembly".into(),
            value: "assembly".into(),
        },
        Item {
            label: "COBOL".into(),
            value: "cobol".into(),
        },
        Item {
            label: "Fortran".into(),
            value: "fortran".into(),
        },
        Item {
            label: "Groovy".into(),
            value: "groovy".into(),
        },
        Item {
            label: "VB.NET".into(),
            value: "vbnet".into(),
        },
        Item {
            label: "PowerShell".into(),
            value: "powershell".into(),
        },
        Item {
            label: "Racket".into(),
            value: "racket".into(),
        },
        Item {
            label: "V".into(),
            value: "v".into(),
        },
        Item {
            label: "Elm".into(),
            value: "elm".into(),
        },
        Item {
            label: "ReasonML".into(),
            value: "reasonml".into(),
        },
        Item {
            label: "Svelte".into(),
            value: "svelte".into(),
        },
        Item {
            label: "SolidJS".into(),
            value: "solidjs".into(),
        },
        Item {
            label: "Q#".into(),
            value: "qsharp".into(),
        },
    ]);

    let on_select = {
        let selected = selected.clone();
        Callback::from(move |val: String| {
            web_sys::console::log_1(&format!("Selected language: {val}").into());
            selected.set(val);
        })
    };

    // Dynamic example — fetch Pokémon from public API
    let poke_selected = use_state(|| "".to_string());
    let poke_items = use_state(|| {
        Rc::new(vec![
            Item {
                label: "bulbasaur".into(),
                value: "bulbasaur".into(),
            },
            Item {
                label: "charmander".into(),
                value: "charmander".into(),
            },
        ])
    });

    let on_poke_select = {
        let poke_selected = poke_selected.clone();
        Callback::from(move |val: String| {
            web_sys::console::log_1(&format!("Selected Pokémon: {val}").into());
            poke_selected.set(val)
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
                {
                    if let Ok(parsed) = resp.json::<PokemonResponse>().await {
                        let new_items: Vec<Item> = parsed
                            .results
                            .into_iter()
                            .map(|p| Item {
                                label: p.name.clone(),
                                value: p.name,
                            })
                            .collect();

                        let mut current = (*poke_items).iter().cloned().collect::<Vec<_>>();
                        current.extend(new_items);
                        poke_items.set(Rc::new(current));
                    }
                }
            });
        })
    };

    let example = html! {
        <div class="space-y-6">
            <div>
                <Typo class="font-bold text-lg mb-2">{"Static Language Example"}</Typo>
                <SearchInput
                    items={suggestions}
                    debounce_ms={250}
                    on_select={Some(on_select)}
                    id="language"
                    label="Language"
                    placeholder="Type a language"
                    required={true}
                    default_selected={Some(Item { label: "Rust".into(), value: "rust".into() })}
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
                    on_select={Some(on_poke_select)}
                    on_fetch_more={Some(on_fetch_more)}
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
                "default_value",
                "required",
                "class",
                "aria_label",
                "aria_labelledby",
                "aria_describedby",
                "pattern",
            ]
            .into_iter()
            .map(Html::from)
            .collect(),
        },
        Column {
            header: "Type".into(),
            values: vec![
                "Rc<Vec<Item>>",
                "Option<Callback<String>>",
                "Option<Callback<()>>",
                "u32",
                "Option<AttrValue>",
                "Option<AttrValue>",
                "Option<AttrValue>",
                "Option<AttrValue>",
                "bool",
                "Classes",
                "Option<AttrValue>",
                "Option<AttrValue>",
                "Option<AttrValue>",
                "Option<AttrValue>",
            ]
            .into_iter()
            .map(Html::from)
            .collect(),
        },
        Column {
            header: "Description".into(),
            values: vec![
                "List of selectable items with `label` and `value`.",
                "Callback fired when an item is selected.",
                "Called when no results are found (e.g. fetch more).",
                "Debounce interval (ms) for filtering.",
                "Input ID (used for `name` and `aria-labelledby`).",
                "Visible label for the input.",
                "Placeholder text inside the input.",
                "Initial value shown in the input.",
                "Marks the input as required.",
                "Custom Tailwind CSS classes.",
                "ARIA label for accessibility.",
                "ARIA labelledby reference.",
                "ARIA describedby reference.",
                "Optional regex pattern for validation.",
            ]
            .into_iter()
            .map(Html::from)
            .collect(),
        },
    ];

    html! {
        <DemoComponent
            title="SearchInput Component"
            description={Some(html! {
                <p>
                    {"`SearchInput` is a debounced autocomplete input with dynamic loading, form integration, and full Tailwind + ARIA support."}
                </p>
            })}
            example={example}
            usage_code={USAGE_CODE}
            props_table={Some(props_table)}
        />
    }
}
