use crate::{
    Accordion, AddIcon, DeleteIcon,
    atoms::{Button, ButtonType},
};
use serde_json::{Map, Value, json};
use uuid::Uuid;
use yew::prelude::*;

use super::{Input, InputType};

type EntryUpdater = Box<dyn FnOnce(&mut Vec<JsonEntry>)>;
type JsonEntry = (Uuid, String, Value);

#[derive(Properties, PartialEq, Clone)]
pub struct JsonInputProps {
    pub id: String,
    pub label: String,
    #[prop_or_default]
    pub initial_value: Option<Value>,
    #[prop_or_default]
    pub on_json_change: Option<Callback<Value>>,
    #[prop_or(true)]
    pub display_buttons: bool,
    #[prop_or(false)]
    pub require_at_least_one: bool,
    #[prop_or_default]
    pub disable_keys: bool,
    #[prop_or_default]
    pub disable_values: bool,
}

#[component(JsonInput)]
pub fn json_input(props: &JsonInputProps) -> Html {
    // Build initial entries from value or start blank
    let json_state = use_state(|| props.initial_value.clone().unwrap_or(json!({})));
    let initial_entries: Vec<JsonEntry> = json_state
        .as_object()
        .map(|obj| {
            obj.iter()
                .map(|(k, v)| (Uuid::new_v4(), k.clone(), v.clone()))
                .collect()
        })
        .unwrap_or_else(|| vec![(Uuid::new_v4(), "".into(), Value::String("".into()))]);
    let entries = use_state(move || initial_entries.clone());

    // Regenerate JSON on entries change, filter out empty/duplicate keys
    {
        let entries = entries.clone();
        let json_state = json_state.clone();
        let on_json_change = props.on_json_change.clone();
        use_effect_with(entries.clone(), move |_| {
            let mut map = Map::new();
            for (_, key, val) in entries.iter() {
                if !key.is_empty() && !map.contains_key(key) {
                    map.insert(key.clone(), val.clone());
                }
            }
            let new_json = Value::Object(map);
            if *json_state != new_json {
                json_state.set(new_json.clone());
                if let Some(cb) = on_json_change.as_ref() {
                    cb.emit(new_json.clone());
                }
            }
        });
    }

    // Central update helper
    let update_entries = {
        let entries = entries.clone();
        Callback::from(move |f: EntryUpdater| {
            let mut list = (*entries).clone();
            f(&mut list);
            entries.set(list);
        })
    };

    // Add: always blank row
    let on_add = {
        let update_entries = update_entries.clone();
        Callback::from(move |_| {
            update_entries.emit(Box::new(move |list: &mut Vec<JsonEntry>| {
                list.push((Uuid::new_v4(), "".into(), Value::String("".into())));
            }));
        })
    };

    let total = entries.len();
    let keys: Vec<String> = entries.iter().map(|(_, k, _)| k.clone()).collect();

    html! {
        <div class="p-4 bg-white dark:bg-gray-900 rounded-lg shadow-md border border-gray-200 dark:border-gray-700">
            <h3 class="text-lg font-semibold text-gray-800 dark:text-gray-200 mb-4">{ &props.label }</h3>
            { for entries.iter().enumerate().map(|(i, (id, key, val))| {
                let id = *id;
                let key_val = key.clone();
                let val_clone = val.clone();

                // Check for duplicate or empty key
                let is_empty = key_val.trim().is_empty();
                let is_duplicate = keys.iter().enumerate().any(|(j, k)| i != j && k == &key_val);

                // Remove callback
                let remove_cb = {
                    let update_entries = update_entries.clone();
                    let require_at_least_one = props.require_at_least_one;
                    Callback::from(move |_| {
                        update_entries.emit(Box::new(move |list: &mut Vec<JsonEntry>| {
                            if !require_at_least_one ||list.len() > 1  {
                                list.retain(|(uid, _, _)| *uid != id);
                            }
                        }));
                    })
                };

                // Value update callback
                let update_value_cb = {
                    let update_entries = update_entries.clone();
                    Callback::from(move |new_val: String| {
                        update_entries.emit(Box::new(move |list: &mut Vec<JsonEntry>| {
                            if let Some((_, _, v)) = list.iter_mut().find(|(uid, _, _)| *uid == id) {
                                *v = Value::String(new_val.clone());
                            }
                        }));
                    })
                };

                // Key update callback
                let update_key_cb = {
                    let update_entries = update_entries.clone();
                    Callback::from(move |new_key: String| {
                        update_entries.emit(Box::new(move |list: &mut Vec<JsonEntry>| {
                            if let Some((_, k, _)) = list.iter_mut().find(|(uid, _, _)| *uid == id) {
                                *k = new_key.clone();
                            }
                        }));
                    })
                };
                let entries = entries.clone();
                let validate_cb = {
                    let entries = entries.clone();
                    Callback::from(move |val: String| {
                        let val_trim = val.trim();
                        if val_trim.is_empty() {
                            return Some("Key required".into());
                        }
                        // Check for duplicate among *other* keys in latest state:
                        let count = entries.iter().filter(|(_, k, _)| k == val_trim).count();
                        if count > 1 {
                            return Some("Duplicate key".into());
                        }
                        None
                    })
                };

                let key_input = html! {
                    <div class="flex flex-col w-full">
                        <Input
                            id={format!("key-{}", id)}
                            label="Key"
                            placeholder="Enter key"
                            input_type={InputType::Text}
                            default_value={key_val.clone()}
                            required=true
                            validate={validate_cb.clone()}
                            class={classes!(
                                is_empty.then_some("border-red-500"),
                                is_duplicate.then_some("border-yellow-400"),
                            )}
                            on_change={update_key_cb.clone()}
                            disabled={props.disable_keys}
                        />
                        { if is_empty {
                            html! { <span class="text-xs text-red-500">{ "Key required" }</span> }
                        } else if is_duplicate {
                            html! { <span class="text-xs text-yellow-600">{ "Duplicate key" }</span> }
                        } else { html!{} }}
                    </div>
                };

                html! {
                    <div
                        key={id.to_string()}
                        class="relative grid grid-cols-1 sm:grid-cols-2 gap-x-6 gap-y-2 p-3 border border-gray-300 dark:border-gray-700 rounded-md bg-gray-50 dark:bg-gray-800 items-start"
                    >

                        // --- ACCORDION FOR NESTED (full row) ---
                        {
                            if let Value::Object(_) = &val_clone {
                                html! {
                                    <div class="col-span-full w-full">
                                        <Accordion
                                            title={key_input.clone()}
                                            default_open={false}
                                            compact=true
                                        >
                                            <JsonInput
                                                id={format!("nested-{}", id)}
                                                label=""
                                                initial_value={Some(val_clone.clone())}
                                                display_buttons={props.display_buttons}
                                                require_at_least_one={props.require_at_least_one}
                                                disable_keys={props.disable_keys}
                                                disable_values={props.disable_values}
                                                on_json_change={Callback::from({
                                                    let update_entries = update_entries.clone();
                                                    move |v| {
                                                        update_entries.emit(Box::new(move |list: &mut Vec<JsonEntry>| {
                                                            if let Some((_, _, val)) = list.iter_mut().find(|(uid, _, _)| *uid == id) {
                                                                *val = v;
                                                            }
                                                        }))
                                                    }
                                                })}
                                            />
                                        </Accordion>
                                    </div>
                                }
                            } else {
                                html! {
                                    <>
                                    {key_input}

                                    // --- VALUE CELL (only if NOT nested) ---
                                    {
                                        if !matches!(val_clone, Value::Object(_)) {
                                            html! {
                                                <div>
                                                    <Input
                                                        id={format!("val-{}", id)}
                                                        label="Value"
                                                        placeholder="Enter value"
                                                        input_type={InputType::Text}
                                                        default_value={val_clone.as_str().unwrap_or(&val_clone.to_string()).to_string()}
                                                        required=true
                                                        on_change={update_value_cb.clone()}
                                                        disabled={props.disable_values}
                                                    />
                                                </div>
                                            }
                                        } else {
                                            html! {}
                                        }
                                    }
                                    </>
                                }
                            }
                        }

                        // --- DELETE BUTTON ---
                        { if props.display_buttons && (!props.require_at_least_one || total > 1) {
                            html! {
                                <Button
                                    button_type={ButtonType::Danger}
                                    on_click={remove_cb.clone()}
                                    class="absolute top-2 right-2 p-1"
                                >
                                    <DeleteIcon />
                                </Button>
                            }
                        } else { html!{} }}
                    </div>
                }

            })}
            // Add button
            { if props.display_buttons {
                html! {
                    <div class="mt-4 flex justify-end">
                        <Button button_type={ButtonType::Primary} on_click={on_add.clone()} class="px-4 py-2">
                            <AddIcon />
                        </Button>
                    </div>
                }
            } else { html!{} }}
            <input type="hidden" id={props.id.clone()} name={props.id.clone()} value={json_state.to_string()} />
        </div>
    }
}
