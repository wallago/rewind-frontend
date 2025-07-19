use crate::{
    components::{
        Button, Dialog, DialogClose, DialogContent, DialogFooter, DialogHeader, DialogTitle, Input,
    },
    hooks::{use_click_outside, use_task_add},
};
use dioxus::prelude::*;

#[component]
pub fn AddTask(is_open: Signal<bool>, list_uuid: String) -> Element {
    let name = use_signal(|| "".to_string());
    let mut trigger = use_signal(|| false);

    use_task_add(name, list_uuid, trigger);

    use_click_outside(
        "add-task-area".to_string(),
        move || is_open(),
        EventHandler::new(move |_| is_open.set(false)),
    );

    rsx! {
        Dialog { is_open,
            DialogContent { id: "add-task-area", class: "sm:max-w-[425px]",
                DialogHeader {
                    DialogTitle { "Add Task" }
                }
                Input {
                    width: "w-full",
                    placeholder: "Enter task name",
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
