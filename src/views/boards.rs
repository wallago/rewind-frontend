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
pub struct Board {
    pub uuid: String,
    name: String,
    description: Option<String>,
}

#[component]
pub fn Boards() -> Element {
    let refetch_signal = use_signal(|| 0);
    let boards = fetch::boards::get_boards(refetch_signal);

    rsx! (
         Body {
            p {
                class: "text-xl font-bold pb-6",
                "Welcom to your task manager!"
            }
            Table {
                TableCaption { "List of your boards." }
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
                            DialogAdd { refetch_signal }
                        }
                    }
                }

                TableBody {
                    match boards() {
                        Some(Some(boards)) => {
                                if boards.is_empty() {
                                    rsx!(TableRow { TableCell {
                                        colspan: Some(4),
                                        "No boards found"
                                    } })
                                } else {
                                    let boards = boards.clone();
                                    rsx!(
                                        {boards.iter().map(|board| {
                                            let uuid = board.uuid.clone();
                                            let name = board.name.clone();
                                            let desc = board.description.clone().unwrap_or_default();
                                            rsx!(TableRow {
                                                class: "cursor-pointer hover:bg-muted-light dark:hover:bg-muted-dark",
                                                onclick: move |e: MouseEvent| {
                                                    e.stop_propagation();
                                                    navigator().push(Route::Lists { uuid: uuid.clone() });
                                                },

                                                TableCell {
                                                    class: Some("font-medium".to_string()),
                                                    {board.uuid.clone()}
                                                }
                                                TableCell { {name} }
                                                TableCell { {desc} }
                                                TableCell {
                                                    class: Some("flex justify-end gap-2".to_string()),
                                                    DialogUpdate { refetch_signal, board: board.clone() }
                                                    DialogDelete { refetch_signal,  board: board.clone() }
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
pub fn DialogAdd(refetch_signal: Signal<u32>) -> Element {
    let name = use_signal(|| "X".to_string());
    let description = use_signal(|| "".to_string());

    rsx!(DialogForm {
        title: String::from("Add Board"),
        description: Some(String::from("Create a new board.")),
        submit: Some(EventHandler::new(move |_| {
            spawn_local(async move {
                let board = fetch::boards::NewBoard {
                    name: name(),
                    description: Some(description()),
                };

                match fetch::boards::add_board(board).await {
                    Ok(_) => refetch_signal.set(refetch_signal() + 1),
                    Err(err) => tracing::error!("Add board failed with: {err}"),
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
pub fn DialogUpdate(refetch_signal: Signal<u32>, board: Board) -> Element {
    let name = use_signal(|| board.name);
    let description = use_signal(|| board.description.unwrap_or_default());

    rsx!(DialogForm {
        title: String::from("Update Board"),
        description: Some(String::from("Edit your board.")),
        submit: Some(EventHandler::new(move |_| {
            let uuid = board.uuid.clone();
            spawn_local(async move {
                let board = fetch::boards::UpdateBoard {
                    name: Some(name()),
                    description: Some(description()),
                };

                match fetch::boards::update_board(&uuid, board).await {
                    Ok(_) => refetch_signal.set(refetch_signal() + 1),
                    Err(err) => tracing::error!("Update board failed with: {err}"),
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
pub fn DialogDelete(refetch_signal: Signal<u32>, board: Board) -> Element {
    rsx!(DialogSimple {
        title: String::from("Delete Board"),
        description: Some(String::from("Erase your board.")),
        submit: Some(EventHandler::new(move |_| {
            let uuid = board.uuid.clone();
            spawn_local(async move {
                match fetch::boards::delete_board(&uuid).await {
                    Ok(_) => refetch_signal.set(refetch_signal() + 1),
                    Err(err) => tracing::error!("Update board failed with: {err}"),
                }
            });
        })),
        trigger: rsx!(Button {
            Cross { size: "14px"}
        }),
    })
}
