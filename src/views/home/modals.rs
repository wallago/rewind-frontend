use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_solid_icons::FaXmark;

use crate::{
    Route,
    api::{delete_board, get_boards},
    components::{
        Button, Card, Dialog, DialogClose, DialogContent, DialogDescription, DialogFooter,
        DialogHeader, DialogTitle, HoverCard, HoverCardContent, Label,
    },
    hooks::use_click_outside,
    models::Board,
};

#[component]
pub fn DeleteBoard(board: Board, is_open: Signal<bool>) -> Element {
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
                            let uuid = board.uuid.clone();
                            use_future(move || {
                                let board_uuid = uuid.clone();
                                async move {
                                    match delete_board(board_uuid).await {
                                        Ok(_) => {}
                                        Err(err) => tracing::error!("{err}"),
                                    }
                                }
                            });
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
