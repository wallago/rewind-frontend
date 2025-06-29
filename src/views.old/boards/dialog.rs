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
pub fn DialogAdd(refetch_signal: Signal<u32>, boards: Resource<Option<Vec<Board>>>) -> Element {
    let name = use_signal(|| "".to_string());
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
                let board = NewBoard {
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
            Button {
                class: "flex gap-2 items-center p-0",
                p {
                    class: "pl-1",
                    "board"
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
pub fn DialogUpdate(refetch_signal: Signal<u32>, board: Board) -> Element {
    let name = use_signal(|| board.name);
    let description = use_signal(|| board.description.unwrap_or_default());

    rsx!(DialogForm {
        title: String::from("Update Board"),
        description: Some(String::from("Edit your board.")),
        submit: Some(EventHandler::new(move |_| {
            let uuid = board.uuid.clone();
            spawn_local(async move {
                let board = UpdateBoard {
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
                Textarea {
                    name: "description",
                    value: description
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
