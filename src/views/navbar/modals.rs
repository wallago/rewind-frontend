use dioxus::prelude::*;

use crate::Route;
use crate::api::add_board;
use crate::components::{
    Button, Dialog, DialogClose, DialogContent, DialogFooter, DialogHeader, DialogTitle, Input,
};
use crate::context::BoardsContext;
use crate::models::NewBoard;

#[component]
pub fn AddBoard(is_open: Signal<bool>) -> Element {
    let mut ctx_boards = use_context::<BoardsContext>();
    let name = use_signal(|| "".to_string());
    let mut trigger = use_signal(|| false);
    let mut in_progress = use_signal(|| false);

    let _ = use_resource(move || async move {
        if trigger() {
            in_progress.set(true);
            match add_board(NewBoard {
                name: name(),
                position: None,
            })
            .await
            {
                Ok(_) => ctx_boards.refresh.set(()),
                Err(err) => tracing::error!("{err}"),
            };
            in_progress.set(false);
        }
    });

    use_effect(move || {
        if !in_progress() {
            trigger.set(false);
        }
    });

    rsx! {
        Dialog { is_open,
            DialogContent { id: "add-board-area", class: "sm:max-w-[425px]",
                DialogHeader {
                    DialogTitle { "Add Board" }
                }
                Input {
                    width: "w-full",
                    placeholder: "Enter board name",
                    value: name,
                    onenter: EventHandler::new(move |_e: KeyboardEvent| {
                        trigger.set(true);
                        navigator().push(Route::Home {});
                        is_open.set(false);
                    }),
                }
                DialogFooter {
                    DialogClose {}
                    Button {
                        onclick: move |_| {
                            trigger.set(true);
                            navigator().push(Route::Home {});
                            is_open.set(false);
                        },
                        r#type: "submit",
                        variant: "outline",
                        class: "font-semibold px-2 text-sm",
                        "Save"
                    }
                }
            }
        }
    }
}
