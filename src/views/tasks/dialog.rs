use dioxus::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::components::{
    Button, Combobox, Input, Label, Select, Textarea,
    dialog::*,
    fetch,
    icons::{Add, Cross, Settings},
    models::{NewTask, Task, UpdateTask},
};

#[component]
pub fn DialogAdd(
    uuid: String,
    refetch_signal: Signal<u32>,
    tasks: Resource<Option<Vec<Task>>>,
) -> Element {
    let name = use_signal(|| "".to_string());
    let description = use_signal(|| "".to_string());
    let status = use_signal(|| Some("Todo".to_string()));
    let priority = use_signal(|| Some("Medium".to_string()));

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
                let task = NewTask {
                    name: name(),
                    description: Some(description()),
                    list_uuid: uuid,
                    position: position,
                    status: status().unwrap_or("Todo".to_string()),
                    priority: priority().unwrap_or("Medium".to_string()),
                };

                match fetch::tasks::add_task(task).await {
                    Ok(_) => refetch_signal.set(refetch_signal() + 1),
                    Err(err) => tracing::error!("Add task failed with: {err}"),
                }
            });
        })),
        trigger: rsx!(Button {
            Button {
                class: "flex gap-2 items-center p-0",
                p {
                    class: "pl-1",
                    "task"
                }
                Add { size: "14px"}
            }
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
                Textarea {
                    name: "description",
                    value: description
                }
            }
            div {
                class: "grid gap-3",
                Label { "Status" }
                Select {
                    name: "status",
                    options: [
                        ("Todo".to_string(),"Todo".to_string()),
                        ("InProgress".to_string(),"In Progress".to_string()),
                        ("Done".to_string(),"Done".to_string())
                    ].to_vec(),
                    selected: status
                }
            }
            div {
                class: "grid gap-3",
                Label { "Priority" }
                Select {
                    name: "priority",
                    options: [
                        ("Low".to_string(),"Low".to_string()),
                        ("Medium".to_string(),"Medium".to_string()),
                        ("High".to_string(),"High".to_string())
                    ].to_vec(),
                    selected: priority
                }
            }
            div {
                class: "grid gap-3",
                Label { "Tags" }
                Combobox { title: "Select", options: [
                    ("a".to_string(),"A".to_string())
                ].to_vec() }
                // Select {
                //     name: "priority",
                //     options: [
                //         ("Low".to_string(),"Low".to_string()),
                //         ("Medium".to_string(),"Medium".to_string()),
                //         ("High".to_string(),"High".to_string())
                //     ].to_vec(),
                //     selected: priority
                // }
            }
        )
    })
}

#[component]
pub fn DialogUpdate(refetch_signal: Signal<u32>, task: Task) -> Element {
    let name = use_signal(|| task.name);
    let description = use_signal(|| task.description.unwrap_or_default());
    let status = use_signal(|| Some(task.status));
    let priority = use_signal(|| Some(task.priority));

    rsx!(DialogForm {
        title: String::from("Update Task"),
        description: Some(String::from("Edit your task.")),
        submit: Some(EventHandler::new(move |_| {
            let uuid = task.uuid.clone();
            spawn_local(async move {
                let task = UpdateTask {
                    name: Some(name()),
                    description: Some(description()),
                    position: None,
                    status: status(),
                    priority: priority(),
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
                Textarea {
                    name: "description",
                    value: description
                }
            }
            div {
                class: "grid gap-3",
                Label { "Status" }
                Select {
                    name: "status",
                    options: [
                        ("Todo".to_string(),"Todo".to_string()),
                        ("InProgress".to_string(),"In Progress".to_string()),
                        ("Done".to_string(),"Done".to_string())
                    ].to_vec(),
                    selected: status
                }
            }
            div {
                class: "grid gap-3",
                Label { "Priority" }
                Select {
                    name: "priority",
                    options: [
                        ("Low".to_string(),"Low".to_string()),
                        ("Medium".to_string(),"Medium".to_string()),
                        ("High".to_string(),"High".to_string())
                    ].to_vec(),
                    selected: priority
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
