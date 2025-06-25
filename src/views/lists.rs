use dioxus::prelude::*;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;

use crate::{
    Route,
    components::{
        Button, Input, Label, Textarea,
        dialog::*,
        fetch,
        icons::{Add, Cross, Settings},
        layout::Body,
        table::*,
    },
};

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct List {
    pub uuid: String,
    board_uuid: String,
    name: String,
    description: Option<String>,
    position: i32,
}

#[component]
pub fn Lists(uuid: String) -> Element {
    let mut refetch_signal = use_signal(|| 0);
    let lists = fetch::lists::get_lists(uuid.clone(), refetch_signal);

    rsx! (
         Body {
            p {
                class: "text-sm font-semibold pb-6",
                "Board: {uuid}"
            }
            Table {
                TableCaption { "List of your lists." }
                TableHeader {
                    TableRow {
                        TableHead {
                            class: Some("w-96".to_string()),
                            "UUID"
                        }
                        TableHead { "Name" }
                        TableHead { "Description" }
                        TableHead {
                            class: Some("text-right".to_string()),
                            DialogAdd { uuid, refetch_signal, lists }
                        }
                    }
                }

                TableBody {
                    match lists() {
                        Some(Some(lists)) => {
                                if lists.is_empty() {
                                    rsx!(TableRow { TableCell {
                                        colspan: Some(4),
                                        "No lists found"
                                    } })
                                } else {
                                    let lists = lists.clone();
                                    let mut dragging_list_uuid = use_signal(|| None::<String>);
                                    rsx!(
                                        {lists.iter().map(|list| {
                                            let uuid = list.uuid.clone();
                                            let name = list.name.clone();
                                            let desc = list.description.clone().unwrap_or_default();

                                            let dragged_uuid_from = list.uuid.clone();
                                            let dragged_uuid_to = list.uuid.clone();
                                            rsx!(TableRow {
                                                class: "cursor-pointer hover:bg-surface-variant-light dark:hover:bg-surface-variant-dark",
                                                onclick: move |e: MouseEvent| {
                                                    e.stop_propagation();
                                                    navigator().push(Route::Tasks { uuid: uuid.clone() });
                                                },
                                                draggable: Some(true),
                                                ondragstart: Some(EventHandler::new(move |_: DragEvent| {
                                                    dragging_list_uuid.set(Some(dragged_uuid_from.clone()));
                                                })),
                                                ondrop: Some(EventHandler::new(move |_: DragEvent| {
                                                    let uuid_to = dragged_uuid_to.clone();
                                                    if let Some(uuid_from) = dragging_list_uuid() {
                                                        spawn_local(async move {
                                                            match fetch::lists::switch_lists(&uuid_from, &uuid_to).await {
                                                                Ok(_) => refetch_signal.set(refetch_signal() + 1),
                                                                Err(err) => tracing::error!("Update list failed with: {err}"),
                                                            }
                                                        })
                                                    }
                                                })),

                                                TableCell {
                                                    class: Some("font-medium".to_string()),
                                                    {list.uuid.clone()}
                                                }
                                                TableCell { {name} }
                                                TableCell { {desc} }
                                                TableCell {
                                                    class: Some("flex justify-end gap-2".to_string()),
                                                    DialogUpdate { refetch_signal, list: list.clone() }
                                                    DialogDelete { refetch_signal,  list: list.clone() }
                                                }
                                            })
                                        })}
                                )
                            }
                        },
                        _ => rsx!(TableRow { TableCell {
                            colspan: Some(4),
                            "Loading..."
                        } }),
                    }
                }
            }
        }
    )
}

#[component]
pub fn DialogAdd(
    uuid: String,
    refetch_signal: Signal<u32>,
    lists: Resource<Option<Vec<List>>>,
) -> Element {
    let name = use_signal(|| "X".to_string());
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
                let list = fetch::lists::NewList {
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
            Add { size: "14px"}
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
                let list = fetch::lists::UpdateList {
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
