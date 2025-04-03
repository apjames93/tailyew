use crate::atoms::{Button, ButtonType};
use serde_json::{json, Map, Value};
use std::cell::RefCell;
use std::iter::FromIterator;
use std::rc::Rc;
use uuid::Uuid;
use yew::prelude::*;

use super::{Input, InputType};

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
}

#[function_component(JsonInput)]
pub fn json_input(props: &JsonInputProps) -> Html {
    let json_state = use_state(|| props.initial_value.clone().unwrap_or(json!({})));

    let items = use_state(|| {
        Rc::new(RefCell::new(
            json_state
                .as_object()
                .map(|obj| {
                    obj.iter()
                        .map(|(key, value)| (Uuid::new_v4(), key.clone(), value.clone()))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default(),
        ))
    });

    // Update the JSON output anytime the items change
    let update_json = {
        let items = items.clone();
        let json_state = json_state.clone();
        let on_json_change = props.on_json_change.clone();

        Callback::from(move |_: ()| {
            let json_object: Map<String, Value> = Map::from_iter(
                items
                    .borrow()
                    .iter()
                    .map(|(_, key, value)| (key.clone(), value.clone())),
            );

            let new_json = Value::Object(json_object);
            if *json_state != new_json {
                json_state.set(new_json.clone());
                if let Some(cb) = &on_json_change {
                    cb.emit(new_json);
                }
            }
        })
    };

    // Trigger update on items change (Yew 0.21 workaround)
    {
        let update_json = update_json.clone();
        use_effect_with(items.clone(), move |_| {
            update_json.emit(());
            || ()
        });
    }

    let add_item = {
        let items = items.clone();
        let update_json = update_json.clone();
        let json_state = json_state.clone();

        Callback::from(move |_| {
            let mut new_items = items.borrow().clone();
            let existing_keys: Vec<String> =
                new_items.iter().map(|(_, key, _)| key.clone()).collect();

            let generate_unique_key = |base: &str| -> String {
                let mut counter = 1;
                let mut new_key = base.to_string();
                while existing_keys.contains(&new_key) {
                    new_key = format!("{}_{}", base, counter);
                    counter += 1;
                }
                new_key
            };

            if let Some(first_record) = new_items.first().cloned() {
                let (_, first_key, first_value) = first_record;
                let new_key = generate_unique_key(&first_key);
                new_items.push((Uuid::new_v4(), new_key, first_value.clone()));
            } else if let Some(existing_obj) = json_state.as_object() {
                let new_key = if existing_obj.values().all(|v| !v.is_object()) {
                    generate_unique_key("new_key")
                } else {
                    generate_unique_key("nested_entry")
                };
                new_items.push((Uuid::new_v4(), new_key, Value::String("".to_string())));
            } else {
                let new_key = generate_unique_key("key");
                new_items.push((Uuid::new_v4(), new_key, Value::String("".to_string())));
            }

            *items.borrow_mut() = new_items;
            update_json.emit(());
        })
    };

    let remove_item = {
        let items = items.clone();
        let update_json = update_json.clone();

        Callback::from(move |id: Uuid| {
            let mut new_items = items.borrow().clone();

            if new_items.len() == 1 {
                let (_, _, value) = &new_items[0];
                if value.is_object() {
                    return; // Prevent removing the only nested object
                }
            }

            new_items.retain(|(uuid, _, _)| *uuid != id);
            *items.borrow_mut() = new_items;
            update_json.emit(());
        })
    };

    let update_item = {
        let items = items.clone();
        let update_json = update_json.clone();

        Callback::from(move |(id, field, val): (Uuid, String, Value)| {
            let mut new_items = items.borrow().clone();

            if let Some(index) = new_items.iter().position(|(uuid, _, _)| *uuid == id) {
                if field == "key" {
                    new_items[index].1 = val.as_str().unwrap_or("").to_string();
                } else {
                    new_items[index].2 = val;
                }
            }

            *items.borrow_mut() = new_items;
            update_json.emit(());
        })
    };

    html! {
        <div class="p-4 bg-white dark:bg-gray-900 rounded-lg shadow-md border border-gray-200 dark:border-gray-700">
            <h3 class="text-lg font-semibold text-gray-800 dark:text-gray-200 mb-4">{ &props.label }</h3>

            { for items.borrow().iter().map(|(id, key, value)| {
                let id_clone = *id;
                let key_clone = key.clone();
                let value_clone = value.clone();
                let items_clone = items.clone();
                let update_json_clone = update_json.clone();

                html! {
                    <div class="flex flex-wrap items-center gap-3 p-3 border border-gray-300 dark:border-gray-700 rounded-md bg-gray-50 dark:bg-gray-800">
                        <Input
                            id={id.to_string()}
                            label="Key"
                            placeholder="Enter key name"
                            input_type={InputType::Text}
                            default_value={key_clone.clone()}
                            required={true}
                            on_change={update_item.reform(move |val| (id_clone, "key".into(), Value::String(val)))}
                        />

                        {
                            if let Value::Object(_) = value_clone {
                                html! {
                                    <JsonInput
                                        id={format!("nested-{}", id_clone)}
                                        label={format!("Nested JSON for {}", key_clone)}
                                        initial_value={Some(value_clone.clone())}
                                        display_buttons={true}
                                        on_json_change={Callback::from(move |updated_nested_json: Value| {
                                            let mut new_items = items_clone.borrow().clone();
                                            if let Some(index) = new_items.iter().position(|(uuid, _, _)| *uuid == id_clone) {
                                                new_items[index].2 = updated_nested_json;
                                            }
                                            *items_clone.borrow_mut() = new_items;
                                            update_json_clone.emit(());
                                        })}
                                    />
                                }
                            } else {
                                html! {
                                    <Input
                                        id={format!("value-{}", id_clone)}
                                        input_type={InputType::Text}
                                        label="Value"
                                        placeholder="Enter value"
                                        required={true}
                                        default_value={
                                            value_clone.as_str()
                                                .map(|s| s.to_string())
                                                .unwrap_or_else(|| value_clone.to_string())
                                        }
                                        on_change={update_item.reform(move |val| (id_clone, "value".into(), Value::String(val)))}
                                    />
                                }
                            }
                        }

                        { if props.display_buttons {
                            html! {
                                <Button
                                    button_type={ButtonType::Danger}
                                    class="ml-2"
                                    onclick={remove_item.reform(move |_| id_clone)}
                                >
                                    {"🗑 Remove"}
                                </Button>
                            }
                        } else { html! {} } }
                    </div>
                }
            })}

            { if props.display_buttons {
                html! {
                    <div class="mt-4 flex justify-end">
                        <Button button_type={ButtonType::Primary} class="px-4 py-2" onclick={add_item}>
                            {"➕ Add Entry"}
                        </Button>
                    </div>
                }
            } else { html! {} }}

            <input
                type="hidden"
                id={props.id.clone()}
                name={props.id.clone()}
                value={json_state.to_string()}
            />
        </div>
    }
}
