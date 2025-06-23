use dioxus::prelude::*;

use crate::components::{
    board::row::{BoardRow, BoardRowSkeleton, BoardRowTitle},
    fetch,
};

#[component]
pub fn Boards() -> Element {
    let refetch_signal = use_signal(|| 0);
    let boards = fetch::get_boards(refetch_signal);

    rsx! {
        div {
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
                    class: "
                        bg-muted-light dark:bg-muted-dark 
                        mx-8 my-6 
                        text-sm
                    ",
                    BoardRowTitle { refetch_signal }
                    match boards.read().as_ref() {
                        Some(Some(board_items)) => rsx!(
                            ul {
                                class: "mt-1",
                                if board_items.is_empty() {
                                    li { "no boards found." }
                                } else {
                                    Fragment {
                                        for board in board_items {
                                            BoardRow { board: board.clone(), refetch_signal }
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
}
