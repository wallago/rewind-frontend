use crate::{
    components::TableRow,
    models::{Priority, Status, Task},
};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TaskCardProps {
    task: Task,
}

#[component]
pub fn TaskCard(props: TaskCardProps) -> Element {
    rsx! {
        div {
            TableRow {
                class: "cursor-pointer",
                div { class: "flex items-center gap-2",
                    div { class: "w-full flex items-center h-full gap-4",
                        {<Priority as Into<Element>>::into(props.task.priority.clone())}
                        {<Status as Into<Element>>::into(props.task.status.clone())}
                        div { class: "flex-grow flex-shrink inline-block truncate", {props.task.name.clone()} }
                    }
                }
            }
        }
    }
}
