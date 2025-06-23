use crate::components::{
    icons::{ArrowBottom, More},
    list::List,
    task::{
        Task,
        row::{TaskRow, TaskRowTitle},
    },
};
use dioxus::prelude::*;

static API: &str = "http://0.0.0.0:8081/api";

#[component]
pub fn ListRow(list: List) -> Element {
    let mut is_open = use_signal(|| false);
    let list_clone = list.clone();
    let tasks = use_resource(move || {
        let uuid = list_clone.uuid.clone();
        async move {
            let response = reqwest::get(format!("{API}/lists/{}/tasks", uuid))
                .await
                .ok()?;
            response.json::<Vec<Task>>().await.ok()
        }
    });

    rsx!(li {
        key: "{list.uuid}",
        class: "text-sm border-t-2 border-border-light dark:border-border-dark pt-2",
        div {
            class: "grid grid-cols-4 gap-4",
            p {"{list.name}"}
            p { "{list.description.as_deref().unwrap_or(\"-\")}" }
            match tasks.read().as_ref() {
                Some(Some(list_task_items)) => rsx!(
                    p {
                        class: "flex justify-between",
                        "{list_task_items.len()}"
                        if !list_task_items.is_empty() {
                            p {
                                class: "flex font-bold",
                                "lists"
                                button {
                                    class: "ml-1 transition-transform duration-200",
                                    class: if *is_open.read() { "rotate-180" } else { "rotate-0" },
                                    onclick: move |_| is_open.toggle(),
                                    ArrowBottom {}
                                }
                            }
                        }
                    }
                    button {
                        class: "justify-self-end mr-4 h-6 w-8 border-border-light dark:border-border-dark hover:border-2 hover:bg-muted-light dark:hover:bg-muted-dark",
                        div {
                            class: "flex justify-center items-center w-full h-full",
                            More {}
                        }
                    }
                    if *is_open.read() {
                        ul {
                            class: "py-2 px-4 mb-2 border-2 border-border-light dark:border-border-dark bg-muted-light dark:bg-muted-dark w-full col-span-4 grid grid-cols-1 gap-2",
                            TaskRowTitle {}
                            Fragment {
                                for task in list_task_items {
                                    TaskRow { task: task.clone()  }
                                }
                            }
                        }
                    }
                ),
                Some(None) => rsx!(p {
                    class: "text-error-light dark:text-error-dark",
                    "failed to fetch"
                }),
                None => rsx!(p {
                    ul {
                        class: "space-y-2 animate-pulse",
                        for i in 0..3 {
                            ListRowSkeleton { i }
                        }
                    }
                })
            }
        }
    })
}

#[component]
pub fn ListRowTitle() -> Element {
    rsx!(
        div {
            class: "grid grid-cols-4 gap-4 font-bold",
            p { "name" },
            p { "description" },
            p { "tasks" },
            p { "" },
        }
    )
}

#[component]
pub fn ListRowSkeleton(i: i32) -> Element {
    rsx!(
        li {
            key: "{i}",
            class: "p-2 rounded",
            div {
                class: "grid grid-cols-3 gap-4",
                div { class: "h-4 bg-muted-light dark:bg-muted-dark rounded" }
                div { class: "h-4 bg-muted-light dark:bg-muted-dark rounded" }
                div { class: "h-4 bg-muted-light dark:bg-muted-dark rounded" }
            }
        }
    )
}
