use crate::components::task::Task;
use dioxus::prelude::*;

#[component]
pub fn TaskRow(task: Task) -> Element {
    rsx!(li {
        class: "pt-1 border-t-2 border-border-light dark:border-border-dark",
        key: "{task.uuid}",
        div {
            class: "grid grid-cols-6 gap-4",
            p {"{task.name}"}
            p { "{task.description.as_deref().unwrap_or(\"-\")}" }
            p { "{task.status}" }
            p { "{task.deadline.as_deref().unwrap_or(\"-\")}" }
            p { "todo" }
        }
    })
}

#[component]
pub fn TaskRowTitle() -> Element {
    rsx!(
        div {
            class: "grid grid-cols-6 gap-4 font-bold",
            p { "name" },
            p { "description" },
            p { "status" },
            p { "deadline" },
            p { "priority" },
            p { "" },
        }
    )
}
