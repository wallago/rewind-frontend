use dioxus::prelude::*;

use crate::{
    Route,
    components::{Button, Label},
    models::Board,
};

#[component]
pub fn Home() -> Element {
    let boards = [
        Board {
            uuid: "3276121d-e160-4a17-a4ed-250cd028a177".to_string(),
            name: "Project X".to_string(),
        },
        Board {
            uuid: "3206121d-e160-4a17-a4ed-250cd028a177".to_string(),
            name: "Project Y".to_string(),
        },
        Board {
            uuid: "3286121d-e160-4a17-a4ed-250cd028a177".to_string(),
            name: "Project Z".to_string(),
        },
    ]
    .to_vec();
    rsx! {
        div {
            class: "p-4 h-full bg-primary border-2 border-secondary flex flex-col gap-4",
            Header {  }
            div {
                class: "grid gap-4 grid-cols-[repeat(auto-fit,_minmax(12rem,_1fr))]",
                {boards.iter().map(|board| {
                   rsx!(
                        BoardCard { board: board.clone() }
                    )
                })}
            }
        }
    }
}

#[component]
fn Header() -> Element {
    rsx! {
        Label {
            "Boards"
        }
    }
}

#[component]
fn BoardCard(board: Board) -> Element {
    rsx! {
        div {
            class: "h-fit w-48 p-2 bg-primary-2 border-2 border-secondary flex flex-col gap-4",
            div {
                class: "flex flex-col justify-center text-sm font-medium gap-2 w-full",
                Label {
                    variant: "outline",
                    class: "p-2",
                    width: "w-full",
                    div {
                        class: "truncate",
                        "Name: {board.name}"
                    }
                }
                Label {
                    variant: "outline",
                    class: "p-2",
                    width: "w-full",
                    div {
                        class: "truncate",
                        "UUID: {board.uuid}"
                    }
                }
            }
            div {
                class: "flex justify-center",
                Button {
                    onclick: move |_| {
                        navigator().push(Route::Board { uuid: board.uuid.clone() });
                    },
                    class: "px-4",
                    "Details"
                }
            }
        }
    }
}
