use dioxus::prelude::*;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;

use crate::{
    Route,
    components::{
        Button, Input, Label,
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
}

#[component]
pub fn Lists(uuid: String) -> Element {
    let refetch_signal = use_signal(|| 0);
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
                            DialogAdd { uuid, refetch_signal }
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
                                    rsx!(
                                        {lists.iter().map(|list| {
                                            let uuid = list.uuid.clone();
                                            let name = list.name.clone();
                                            let desc = list.description.clone().unwrap_or_default();
                                            rsx!(TableRow {
                                                class: "cursor-pointer hover:bg-muted-light dark:hover:bg-muted-dark",
                                                onclick: move |e: MouseEvent| {
                                                    e.stop_propagation();
                                                    navigator().push(Route::Tasks { uuid: uuid.clone() });
                                                },

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
pub fn DialogAdd(uuid: String, refetch_signal: Signal<u32>) -> Element {
    let name = use_signal(|| "X".to_string());
    let description = use_signal(|| "".to_string());

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
                Input {
                    class: "",
                    name: "description",
                    value:description
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
                Input {
                    class: "",
                    name: "description",
                    value:description
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
