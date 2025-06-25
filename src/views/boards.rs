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
    position: i32,
}

#[component]
pub fn Boards() -> Element {
    let mut refetch_signal = use_signal(|| 0);
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
                            DialogAdd { refetch_signal, boards }
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
                                    let mut dragging_board_uuid = use_signal(|| None::<String>);
                                    rsx!(
                                        {boards.iter().map(|board| {
                                            let uuid = board.uuid.clone();
                                            let name = board.name.clone();
                                            let desc = board.description.clone().unwrap_or_default();

                                            let dragged_uuid_from = board.uuid.clone();
                                            let dragged_uuid_to = board.uuid.clone();
                                            rsx!(TableRow {
                                                class: "cursor-pointer hover:bg-muted-light dark:hover:bg-muted-dark",
                                                onclick: move |e: MouseEvent| {
                                                    e.stop_propagation();
                                                    navigator().push(Route::Lists { uuid: uuid.clone() });
                                                },
                                                draggable: Some(true),
                                                ondragstart: Some(EventHandler::new(move |_: DragEvent| {
                                                    dragging_board_uuid.set(Some(dragged_uuid_from.clone()));
                                                })),
                                                ondrop: Some(EventHandler::new(move |_: DragEvent| {
                                                    let uuid_to = dragged_uuid_to.clone();
                                                    if let Some(uuid_from) = dragging_board_uuid() {
                                                        spawn_local(async move {
                                                            match fetch::boards::switch_boards(&uuid_from, &uuid_to).await {
                                                                Ok(_) => refetch_signal.set(refetch_signal() + 1),
                                                                Err(err) => tracing::error!("Update board failed with: {err}"),
                                                            }
                                                        })
                                                    }
                                                })),


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
pub fn DialogAdd(refetch_signal: Signal<u32>, boards: Resource<Option<Vec<Board>>>) -> Element {
    let name = use_signal(|| "X".to_string());
    let description = use_signal(|| "".to_string());

    let position = match boards() {
        Some(Some(boards)) => {
            let mut used_positions: Vec<i32> = boards.iter().map(|board| board.position).collect();
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
        title: String::from("Add Board"),
        description: Some(String::from("Create a new board.")),
        submit: Some(EventHandler::new(move |_| {
            spawn_local(async move {
                let board = fetch::boards::NewBoard {
                    name: name(),
                    description: Some(description()),
                    position: position,
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
                    position: None,
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
