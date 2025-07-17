use dioxus::prelude::*;

use crate::Route;
use crate::components::{
    Button, Dialog, DialogClose, DialogContent, DialogFooter, DialogHeader, DialogTitle, Input,
};
use crate::hooks::use_board_add;

#[component]
pub fn AddBoard(is_open: Signal<bool>) -> Element {
    let name = use_signal(|| "".to_string());
    let mut trigger = use_signal(|| false);

    use_board_add(name, trigger);

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
