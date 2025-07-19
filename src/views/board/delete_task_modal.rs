use crate::{
    components::{
        Button, Dialog, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader,
        DialogTitle,
    },
    hooks::{use_click_outside, use_task_delete},
    models::Task,
};
use dioxus::prelude::*;

#[component]
pub fn DeleteTask(task: Task, is_open: Signal<bool>, parent_open: Signal<bool>) -> Element {
    let mut trigger = use_signal(|| false);

    use_task_delete(task.uuid, trigger);

    use_click_outside(
        "delete-task-area".to_string(),
        move || is_open(),
        EventHandler::new(move |_| is_open.set(false)),
    );

    rsx! {
        Dialog { is_open,
            DialogContent { id: "delete-task-area", class: "sm:max-w-[425px]",
                DialogHeader {
                    DialogTitle { "Delete {task.name}" }
                    DialogDescription { "Are you sure ?" }
                }
                DialogFooter {
                    DialogClose {}
                    Button {
                        onclick: move |_| {
                            trigger.set(true);
                            is_open.set(false);
                            parent_open.set(false);
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
