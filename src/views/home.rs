use dioxus::prelude::*;

use crate::components::{
    board::{
        Board,
        row::{BoardRow, BoardRowSkeleton, BoardRowTitle},
    },
    icons::arrow,
};

static API: &str = "http://0.0.0.0:8081/api";

#[component]
pub fn Home() -> Element {
    let boards = use_resource(move || async move {
        let response = reqwest::get(format!("{API}/boards",)).await.ok()?;
        response.json::<Vec<Board>>().await.ok()
    });

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
