use dioxus::prelude::*;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;

use crate::{
    Route,
    components::{
        Button, Input, Label,
        dialog::*,
        fetch,
        icons::{Add, Cross, Settings},
        layout::Body,
        table::*,
    },
};

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Task {
    pub uuid: String,
    list_uuid: String,
    name: String,
    description: Option<String>,
    position: i32,
    status: String,
}

#[component]
pub fn Tasks(uuid: String) -> Element {
    let refetch_signal = use_signal(|| 0);
    let tasks = fetch::tasks::get_tasks(uuid.clone(), refetch_signal);

    rsx! (
         Body {
            p {
                class: "text-sm font-semibold pb-6",
                "List: {uuid}"
            }
            Table {
                TableCaption { "List of your tasks." }
                TableHeader {
                    TableRow {
                        TableHead {
                            class: Some("w-96".to_string()),
                            "UUID"
                        }
                        TableHead { "Name" }
                        TableHead { "Description" }
                        TableHead { "Status" }
                        TableHead {
                            class: Some("text-right".to_string()),
                            DialogAdd { uuid, refetch_signal }
                        }
                    }
                }

                TableBody {
                    match tasks() {
                        Some(Some(tasks)) => {
                                if tasks.is_empty() {
                                    rsx!(TableRow { TableCell {
                                        colspan: Some(5),
                                        "No tasks found"
                                    } })
                                } else {
                                    let tasks = tasks.clone();
                                    rsx!(
                                        {tasks.iter().map(|task| {
                                            let name = task.name.clone();
                                            let desc = task.description.clone().unwrap_or_default();
                                            let status = task.status.clone();
                                            // select and move <3
                                            rsx!(TableRow {
                                                class: "hover:bg-muted-light dark:hover:bg-muted-dark",
                                                TableCell {
                                                    class: Some("font-medium".to_string()),
                                                    {task.uuid.clone()}
                                                }
                                                TableCell { {name} }
                                                TableCell { {desc} }
                                                TableCell { {status} }
                                                TableCell {
                                                    class: Some("flex justify-end gap-2".to_string()),
                                                    DialogUpdate { refetch_signal, task: task.clone() }
                                                    DialogDelete { refetch_signal,  task: task.clone() }
                                                }
                                            })
                                        })}
                                )
                            }
                        },
                        _ => rsx!(TableRow { TableCell {
                            colspan: Some(5),
                            "Loading..."
                        } }),
                    }
                }
            }
        }
    )
}

#[component]
pub fn DialogAdd(uuid: String, refetch_signal: Signal<u32>) -> Element {
    let name = use_signal(|| "X".to_string());
    let description = use_signal(|| "".to_string());

    rsx!(DialogForm {
        title: String::from("Add Task"),
        description: Some(String::from("Create a new task.")),
        submit: Some(EventHandler::new(move |_| {
            let uuid = uuid.clone();
            spawn_local(async move {
                let task = fetch::tasks::NewTask {
                    name: name(),
                    description: Some(description()),
                    list_uuid: uuid,
                    position: 1,
                    status: String::from("Todo"),
                };

                match fetch::tasks::add_task(task).await {
                    Ok(_) => refetch_signal.set(refetch_signal() + 1),
                    Err(err) => tracing::error!("Add task failed with: {err}"),
                }
            });
        })),
        trigger: rsx!(Button {
            Add { size: "14px"}
        }),
        form: rsx!(
            div {
                class: "grid gap-3",
                Label { "Name" }
                Input {
                    name: "name",
                    value: name
                }
            }
            div {
                class: "grid gap-3",
                Label { "Description" }
                Input {
                    class: "",
                    name: "description",
                    value:description
                }
            }
        )
    })
}

#[component]
pub fn DialogUpdate(refetch_signal: Signal<u32>, task: Task) -> Element {
    let name = use_signal(|| task.name);
    let description = use_signal(|| task.description.unwrap_or_default());

    rsx!(DialogForm {
        title: String::from("Update Task"),
        description: Some(String::from("Edit your task.")),
        submit: Some(EventHandler::new(move |_| {
            let uuid = task.uuid.clone();
            spawn_local(async move {
                let task = fetch::tasks::UpdateTask {
                    name: Some(name()),
                    description: Some(description()),
                    position: None,
                    status: None,
                };

                match fetch::tasks::update_task(&uuid, task).await {
                    Ok(_) => refetch_signal.set(refetch_signal() + 1),
                    Err(err) => tracing::error!("Update task failed with: {err}"),
                }
            });
        })),
        trigger: rsx!(Button {
            Settings { size: "14px"}
        }),
        form: rsx!(
            div {
                class: "grid gap-3",
                Label { "Name" }
                Input {
                    name: "name",
                    value: name
                }
            }
            div {
                class: "grid gap-3",
                Label { "Description" }
                Input {
                    class: "",
                    name: "description",
                    value:description
                }
            }
        )
    })
}

#[component]
pub fn DialogDelete(refetch_signal: Signal<u32>, task: Task) -> Element {
    rsx!(DialogSimple {
        title: String::from("Delete Task"),
        description: Some(String::from("Erase your task.")),
        submit: Some(EventHandler::new(move |_| {
            let uuid = task.uuid.clone();
            spawn_local(async move {
                match fetch::tasks::delete_task(&uuid).await {
                    Ok(_) => refetch_signal.set(refetch_signal() + 1),
                    Err(err) => tracing::error!("Update task failed with: {err}"),
                }
            });
        })),
        trigger: rsx!(Button {
            Cross { size: "14px"}
        }),
    })
}
