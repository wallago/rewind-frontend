use dioxus::prelude::*;

use crate::{
    components::{
        Button, Dialog, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader,
        DialogTitle,
    },
    hooks::{use_board_delete, use_click_outside},
    models::Board,
};

#[component]
pub fn DeleteBoard(board: Board, is_open: Signal<bool>) -> Element {
    let mut trigger = use_signal(|| false);

    use_board_delete(board.uuid, trigger);

    use_click_outside(
        "delete-board-area".to_string(),
        move || is_open(),
        EventHandler::new(move |_| is_open.set(false)),
    );

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
