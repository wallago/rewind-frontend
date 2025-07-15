use dioxus::prelude::*;

use crate::{
    api::delete_board,
    components::{
        Button, Dialog, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader,
        DialogTitle,
    },
    models::Board,
};

#[component]
pub fn DeleteBoard(board: Board, is_open: Signal<bool>) -> Element {
    let mut delete = use_signal(|| false);
    use_future(move || {
        let board_uuid = board.uuid.clone();
        async move {
            if !delete() {
                return ();
            } else {
                delete.set(false);
            }
            match delete_board(board_uuid).await {
                Ok(_) => {}
                Err(err) => tracing::error!("{err}"),
            }
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
                            delete.set(true);
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
