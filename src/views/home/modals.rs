use dioxus::prelude::*;

use crate::{
    api::{delete_board, update_board},
    components::{
        Button, Dialog, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader,
        DialogTitle, Input,
    },
    context::BoardsContext,
    models::{Board, UpdateBoard as UpdateBoardModel},
};

#[component]
pub fn DeleteBoard(board: Board, is_open: Signal<bool>) -> Element {
    let mut ctx_boards = use_context::<BoardsContext>();
    let mut trigger = use_signal(|| false);
    let mut in_progress = use_signal(|| false);

    let _ = use_resource(move || {
        let board_uuid = board.uuid.clone();
        async move {
            if trigger() {
                in_progress.set(true);
                match delete_board(board_uuid).await {
                    Ok(_) => ctx_boards.refresh.set(()),
                    Err(err) => tracing::error!("{err}"),
                };
                in_progress.set(false);
            }
        }
    });

    use_effect(move || {
        if !in_progress() {
            trigger.set(false);
        }
    });

    rsx! {
        Dialog { is_open,
            DialogContent { id: "delete-board-area", class: "sm:max-w-[425px]",
                DialogHeader {
                    DialogTitle { "Delete {board.name}" }
                    DialogDescription { "Are you sure ?" }
                }
                DialogFooter {
                    DialogClose {}
                    Button {
                        onclick: move |_| {
                            trigger.set(true);
                            is_open.set(false);
                        },
                        r#type: "submit",
                        variant: "outline",
                        class: "font-semibold px-2 text-sm",
                        "Delete"
                    }
                }
            }
        }
    }
}
