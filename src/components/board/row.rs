use dioxus::prelude::*;

use crate::components::{
    Button,
    board::Board,
    icons::{More, arrow},
    list::{
        List,
        row::{ListRow, ListRowSkeleton, ListRowTitle},
    },
};

#[component]
pub fn BoardRow(board: Board) -> Element {
    rsx!(
        li {
            key: "{board.uuid}",
            class: "text-sm border-t-2 border-border-light dark:border-border-dark py-2",
            div {
                class: "grid grid-cols-4 gap-2",
                p { "{board.name}" }
                p { "{board.description.as_deref().unwrap_or(\"-\")}" }
            }
        }
    )
}

#[component]
pub fn BoardRowTitle() -> Element {
    rsx!(
        div {
            class: "grid grid-cols-4 gap-4 font-bold",
            p { "name" },
            p { "description" },
            p { "lists" },
            div {
                class: "flex justify-between",
                div {}
                Button {
                    "add board"
                }
            },
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
