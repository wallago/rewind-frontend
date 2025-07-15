use dioxus::prelude::*;

use crate::Route;
use crate::api::add_board;
use crate::components::{
    Button, Dialog, DialogClose, DialogContent, DialogFooter, DialogHeader, DialogTitle, Input,
};
use crate::models::NewBoard;

#[component]
pub fn AddBoard(is_open: Signal<bool>) -> Element {
    let name = use_signal(|| "".to_string());

    let mut add = use_signal(|| false);
    use_future(move || async move {
        if !add() {
            return ();
        } else {
            add.set(false);
        }
        match add_board(NewBoard {
            name: name(),
            position: None,
        })
        .await
        {
            Ok(_) => {}
            Err(err) => tracing::error!("{err}"),
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
                }
                DialogFooter {
                    DialogClose {}
                    Button {
                        onclick: move |_| {
                            add.set(true);
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
