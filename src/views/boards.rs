use dioxus::prelude::*;
use serde::Deserialize;

use crate::components::{
    board::{
        Board,
        row::{BoardRow, BoardRowSkeleton, BoardRowTitle},
    },
    fetch,
    icons::arrow,
};

#[component]
pub fn Boards() -> Element {
    let boards = fetch::get_boards()?;
    rsx! {
        div {
            class: "px-4 py-3 mx-52 my-8 border-2 border-border-light dark:border-border-dark",
            p {
                class: "text-xl font-bold",
                "Welcom to your task manager!"
            }
            p {
                class: "mb-8",
                "Here's a list of your tasks for this month."
            }
            div {
                class: "bg-muted-light dark:bg-muted-dark px-4 py-2 mx-8 text-sm my-6",
                BoardRowTitle {}
                match boards.read().as_ref() {
                    Some(Some(board_items)) => rsx!(
                        ul {
                            class: "mt-1",
                            if board_items.is_empty() {
                                li { "no boards found." }
                            } else {
                                Fragment {
                                    for board in board_items {
                                        BoardRow { board: board.clone() }
                                    }
                                }
                            }
                        }
                    ),
                    Some(None) => rsx!(p {
                        class: "text-error-light dark:text-error-dark",
                        "failed to fetch"
                    }),
                    None => rsx!(
                        ul {
                            class: "space-y-2 animate-pulse",
                            for i in 0..3 {
                                BoardRowSkeleton { i }
                            }
                        }
                    ),
                }
            }
        }
    }
}
