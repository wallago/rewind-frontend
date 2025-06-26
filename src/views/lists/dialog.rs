use dioxus::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::components::{
    Button, Input, Label, Textarea,
    dialog::*,
    fetch,
    icons::{Add, Cross, Settings},
    models::*,
};

#[component]
pub fn DialogAdd(
    uuid: String,
    refetch_signal: Signal<u32>,
    lists: Resource<Option<Vec<List>>>,
) -> Element {
    let name = use_signal(|| "".to_string());
    let description = use_signal(|| "".to_string());

    let position = match lists() {
        Some(Some(lists)) => {
            let mut used_positions: Vec<i32> = lists.iter().map(|list| list.position).collect();
            used_positions.sort_unstable();
            used_positions.dedup();

            let mut next_position = 0;

            for &pos in &used_positions {
                if pos == next_position {
                    next_position += 1;
                } else if pos > next_position {
                    break;
                }
            }

            next_position
        }
        Some(None) => 0,
        None => {
            return rsx!(Button {
                disabled: true,
                Add { size: "14px"}
            });
        }
    };

    rsx!(DialogForm {
        title: String::from("Add List"),
        description: Some(String::from("Create a new list.")),
        submit: Some(EventHandler::new(move |_| {
            let uuid = uuid.clone();
            spawn_local(async move {
                let list = NewList {
                    name: name(),
                    description: Some(description()),
                    board_uuid: uuid,
                    position: position,
                };

                match fetch::lists::add_list(list).await {
                    Ok(_) => refetch_signal.set(refetch_signal() + 1),
                    Err(err) => tracing::error!("Add list failed with: {err}"),
                }
            });
        })),
        trigger: rsx!(Button {
            Button {
                class: "flex gap-2 items-center p-0",
                p {
                    class: "pl-1",
                    "list"
                }
                Add { size: "14px"}
            }
        }),
        form: rsx!(
            div {
                class: "grid gap-3",
                Label { "Name" }
                Input {
                    name: "name",
                    value: name
                }
            }
            div {
                class: "grid gap-3",
                Label { "Description" }
                Textarea {
                    name: "description",
                    value: description
                }
            }
        )
    })
}

#[component]
pub fn DialogUpdate(refetch_signal: Signal<u32>, list: List) -> Element {
    let name = use_signal(|| list.name);
    let description = use_signal(|| list.description.unwrap_or_default());

    rsx!(DialogForm {
        title: String::from("Update List"),
        description: Some(String::from("Edit your list.")),
        submit: Some(EventHandler::new(move |_| {
            let uuid = list.uuid.clone();
            spawn_local(async move {
                let list = UpdateList {
                    name: Some(name()),
                    description: Some(description()),
                    position: None,
                };

                match fetch::lists::update_list(&uuid, list).await {
                    Ok(_) => refetch_signal.set(refetch_signal() + 1),
                    Err(err) => tracing::error!("Update list failed with: {err}"),
                }
            });
        })),
        trigger: rsx!(Button {
            Settings { size: "14px"}
        }),
        form: rsx!(
            div {
                class: "grid gap-3",
                Label { "Name" }
                Input {
                    name: "name",
                    value: name
                }
            }
            div {
                class: "grid gap-3",
                Label { "Description" }
                Textarea {
                    name: "description",
                    value: description
                }
            }
        )
    })
}

#[component]
pub fn DialogDelete(refetch_signal: Signal<u32>, list: List) -> Element {
    rsx!(DialogSimple {
        title: String::from("Delete List"),
        description: Some(String::from("Erase your list.")),
        submit: Some(EventHandler::new(move |_| {
            let uuid = list.uuid.clone();
            spawn_local(async move {
                match fetch::lists::delete_list(&uuid).await {
                    Ok(_) => refetch_signal.set(refetch_signal() + 1),
                    Err(err) => tracing::error!("Update list failed with: {err}"),
                }
            });
        })),
        trigger: rsx!(Button {
            Cross { size: "14px"}
        }),
    })
}
