use crate::{
    components::{
        Button, Dialog, DialogClose, DialogContent, DialogFooter, DialogHeader, DialogTitle, Input,
    },
    hooks::{use_click_outside, use_list_add},
};
use dioxus::prelude::*;

#[component]
pub fn AddList(is_open: Signal<bool>, board_uuid: String) -> Element {
    let name = use_signal(|| "".to_string());
    let mut trigger = use_signal(|| false);

    use_list_add(name, board_uuid, trigger);

    use_click_outside(
        "delete-list-area".to_string(),
        move || is_open(),
        EventHandler::new(move |_| is_open.set(false)),
    );

    rsx! {
        Dialog { is_open,
            DialogContent { id: "delete-list-area", class: "sm:max-w-[425px]",
                DialogHeader {
                    DialogTitle { "Add List" }
                }
                Input {
                    label: "name:",
                    width: "w-full",
                    placeholder: "Enter list name",
                    value: name,
                    onenter: EventHandler::new(move |_e: KeyboardEvent| {
                        trigger.set(true);
                        is_open.set(false);
                    }),
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
                        "Save"
                    }
                }
            }
        }
    }
}
