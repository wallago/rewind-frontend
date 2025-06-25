use dioxus::prelude::*;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;

use crate::components::{
    Button, Input, Label,
    dialog::*,
    fetch,
    icons::{Add, Cross, Empty, Full, Half, Label as LabelIcon, Settings},
    layout::Body,
    table::*,
};

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Task {
    pub uuid: String,
    list_uuid: String,
    name: String,
    description: Option<String>,
    position: i32,
    status: String,
    priority: String,
}

#[component]
pub fn Tasks(uuid: String) -> Element {
    let mut refetch_signal = use_signal(|| 0);
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
                            class: Some("w-4".to_string()),
                        }
                        TableHead {
                            class: Some("w-4".to_string()),
                        }
                        TableHead {
                            class: Some("w-96".to_string()),
                            "UUID"
                        }
                        TableHead { "Name" }
                        TableHead { "Description" }
                        TableHead {
                            class: Some("text-right".to_string()),
                            DialogAdd { uuid, refetch_signal, tasks }
                        }
                    }
                }

                TableBody {
                    match tasks() {
                        Some(Some(tasks)) => {
                                if tasks.is_empty() {
                                    rsx!(TableRow { TableCell {
                                        colspan: Some(6),
                                        "No tasks found"
                                    } })
                                } else {
                                    let tasks = tasks.clone();
                                    let mut dragging_task_uuid = use_signal(|| None::<String>);
                                    rsx!(
                                        {tasks.iter().map(|task| {
                                            let name = task.name.clone();
                                            let desc = task.description.clone().unwrap_or_default();
                                            let status = task.status.clone();

                                            let dragged_uuid_from = task.uuid.clone();
                                            let dragged_uuid_to = task.uuid.clone();
                                            rsx!(TableRow {
                                                class: "hover:bg-muted-light dark:hover:bg-muted-dark",
                                                draggable: Some(true),
                                                ondragstart: Some(EventHandler::new(move |_: DragEvent| {
                                                    dragging_task_uuid.set(Some(dragged_uuid_from.clone()));
                                                })),
                                                ondrop: Some(EventHandler::new(move |_: DragEvent| {
                                                    let uuid_to = dragged_uuid_to.clone();
                                                    if let Some(uuid_from) = dragging_task_uuid() {
                                                        spawn_local(async move {
                                                            match fetch::tasks::switch_tasks(&uuid_from, &uuid_to).await {
                                                                Ok(_) => refetch_signal.set(refetch_signal() + 1),
                                                                Err(err) => tracing::error!("Update task failed with: {err}"),
                                                            }
                                                        })
                                                    }
                                                })),

                                                TableCell {
                                                    match task.priority.as_ref() {
                                                        "Low" => rsx!(
                                                            div {
                                                                class: "text-low-light dark:text-low-dark",
                                                                LabelIcon {
                                                                    size: "24px"
                                                                }
                                                            }
                                                        ),
                                                        "Medium" => rsx!(
                                                            div {
                                                                class: "text-medium-light dark:text-medium-dark",
                                                                LabelIcon {
                                                                    size: "24px"
                                                                }
                                                            }
                                                        ),
                                                        "High" => rsx!(
                                                            div {
                                                                class: "text-high-light dark:text-high-dark",
                                                                LabelIcon {
                                                                    size: "24px"
                                                                }
                                                            }
                                                        ),
                                                        _ =>  rsx!(Cross { size: "24px" })
                                                    }
                                                }
                                                TableCell {
                                                    match task.status.as_ref() {
                                                        "Todo" => rsx!(Empty { size: "18px" }),
                                                        "InProgress" => rsx!(Half { size: "18px"}),
                                                        "Done" => rsx!(Full { size: "18px" }),
                                                        _ =>  rsx!(Cross { size: "24px" })
                                                    }
                                                }
                                                TableCell { {task.uuid.clone()} }
                                                TableCell { {name} }
                                                TableCell { {desc} }
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
                            colspan: Some(6),
                            "Loading..."
                        } }),
                    }
                }
            }
        }
    )
}

#[component]
pub fn DialogAdd(
    uuid: String,
    refetch_signal: Signal<u32>,
    tasks: Resource<Option<Vec<Task>>>,
) -> Element {
    let name = use_signal(|| "X".to_string());
    let description = use_signal(|| "".to_string());

    let position = match tasks() {
        Some(Some(tasks)) => {
            let mut used_positions: Vec<i32> = tasks.iter().map(|task| task.position).collect();
            used_positions.sort_unstable();
            used_positions.dedup();

            let mut next_position = 0;

            for &pos in &used_positions {
                if pos == next_position {
                    next_position += 1;
                } else if pos > next_position {
                    break;
                }
            }

            next_position
        }
        Some(None) => 0,
        None => {
            return rsx!(Button {
                disabled: true,
                Add { size: "14px"}
            });
        }
    };

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
                    position: position,
                    status: String::from("Todo"),
                    priority: String::from("Medium"),
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
                    priority: None,
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
