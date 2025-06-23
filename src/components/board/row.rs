use dioxus::prelude::*;

use crate::{
    Route,
    components::{
        Button, Dialog, InputProps,
        board::Board,
        fetch::{self},
        icons::{Add, Cross, Settings},
    },
};
use wasm_bindgen_futures::spawn_local;

#[component]
pub fn BoardRow(board: Board, refetch_signal: Signal<u32>) -> Element {
    let mut is_open_delete = use_signal(|| false);
    let mut is_open_update = use_signal(|| false);
    let board_copy = board.clone();
    let uuid = board.uuid.clone();
    let name = use_signal(|| board_copy.name.to_string());
    let desc = use_signal(|| board_copy.description.unwrap_or_default().to_string());

    rsx!(
        li {
            key: "{board.uuid}",
            class: "
                text-sm 
                border-t-2 border-border-light dark:border-border-dark 
                p-1
            ",
            div {
                class: "
                    grid grid-cols-4 gap-2 items-center
                    px-2 py-1
                    hover:bg-border-light dark:hover:bg-border-dark
                ",
                onclick: move |_| {
                    navigator().push(Route::Lists { uuid: uuid.clone() });
                },
                p { "{board.name}" }
                p { "{board.description.as_deref().unwrap_or(\"-\")}" }
                div {}
                div {
                    class: "justify-self-end",
                    Button {
                        onclick: move |evt: MouseEvent| {
                            evt.stop_propagation();
                            is_open_update.set(true)
                        },
                        class: "mr-2",
                        Settings { size: "14px".to_string() }
                    }
                    Button {
                        onclick: move |evt: MouseEvent| {
                            evt.stop_propagation();
                            is_open_delete.set(true)
                        },
                        Cross { size: "14px".to_string() }
                    }
                }

            }
            if (is_open_update)() {
                Dialog {
                    is_open:is_open_update,
                    title: format!("Update Board {}", board.uuid),
                    inputs: Some([
                        InputProps { name: String::from("name"), value: name},
                        InputProps { name: String::from("desc"), value: desc},
                    ].to_vec()),
                    onclick: Some(EventHandler::new(move |_| {
                        let uuid = board_copy.uuid.clone();
                        spawn_local(async move {
                            let board = fetch::UpdateBoard {
                                name: Some(name()),
                                description: Some(desc()),
                            };

                            match fetch::update_board(&uuid ,board).await {
                                Ok(true) => refetch_signal.set( refetch_signal() + 1),
                                Ok(false) => tracing::error!("Update board failed"),
                                Err(err) => tracing::error!("Update board failed with: {err}")
                            }
                        });
                    })),
                    div {
                        class:"
                            h-1 w-full
                            bg-border-light dark:bg-border-dark
                        " 
                    }

                }
            }
            if (is_open_delete)() {
                Dialog {
                    is_open:is_open_delete,
                    title: format!("Delete Board {}", board.uuid),
                    onclick: Some(EventHandler::new(move |_| {
                        let uuid = board.uuid.clone();
                        spawn_local(async move {
                            match fetch::delete_board(&uuid).await {
                                Ok(true) => refetch_signal.set( refetch_signal() + 1),
                                Ok(false) => tracing::error!("Delete board failed"),
                                Err(err) => tracing::error!("Delete board failed with: {err}")
                            }
                        });
                    })),
                    div {
                        class:"
                            h-1 w-full
                            bg-border-light dark:bg-border-dark
                        " 
                    }

                }
            }
       }
    )
}

#[component]
pub fn BoardRowTitle(refetch_signal: Signal<u32>) -> Element {
    let mut is_open = use_signal(|| false);
    let name = use_signal(|| "".to_string());
    let desc = use_signal(|| "".to_string());

    rsx!(
        div {
            class: "
                text-sm 
                p-1
            ",
            div {
                class: "
                    grid grid-cols-4 gap-4 items-center
                    font-bold 
                    px-2 py-1 h-full
                ",
                p { "name" },
                p { "description" },
                p { "lists" },
                div {
                    class: "flex justify-between",
                    div {}
                    Button {
                        onclick: move |_| is_open.set(true),
                        Add { size: "14px"}
                    }
                },
                if (is_open)() {
                    Dialog {
                        is_open:is_open,
                        title: "Add Board",
                        inputs: Some([
                            InputProps { name: String::from("name"), value: name},
                            InputProps { name: String::from("desc"), value: desc},
                        ].to_vec()),
                        onclick: Some(EventHandler::new(move |_| {
                            spawn_local(async move {
                                let board = fetch::NewBoard {
                                    name: name(),
                                    description: Some(desc()),
                                };

                                match fetch::add_board(board).await {
                                    Ok(_) => refetch_signal.set( refetch_signal() + 1),
                                    Err(err) => tracing::error!("Add board failed with: {err}"),
                                }
                            });
                        })),
                        div {
                            class:"
                                h-1 w-full
                                bg-border-light dark:bg-border-dark
                            " 
                        }

                    }
                }
           }
        }
    )
}

#[component]
pub fn BoardRowSkeleton(i: i32) -> Element {
    rsx!(
        li {
            key: "{i}",
            class: "p-2 rounded",
            div {
                class: "grid grid-cols-3 gap-4",
                div { class: "h-4 bg-muted-light dark:bg-muted-dark rounded" }
                div { class: "h-4 bg-muted-light dark:bg-muted-dark rounded" }
                div { class: "h-4 bg-muted-light dark:bg-muted-dark rounded" }
            }
        }
    )
}
