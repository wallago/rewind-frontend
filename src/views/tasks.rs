use dioxus::prelude::*;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;

use crate::{
    Route,
    components::{
        Button, Dialog, InputProps, fetch,
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
    let mut refetch_signal = use_signal(|| 0);
    let tasks = fetch::tasks::get_tasks(uuid.clone(), refetch_signal);
    let mut is_open_add = use_signal(|| false);
    let new_name = use_signal(|| "".to_string());
    let new_desc = use_signal(|| "".to_string());
    rsx! {
        Body {
            p {
                class: "text-sm font-semibold pb-6",
                "Board: {uuid}"
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
                        TableHead {
                            class: Some("text-right".to_string()),
                            Button {
                                class: "hover:bg-muted-light dark:hover:bg-muted-dark",
                                onclick: move |_| is_open_add.set(true),
                                Add { size: "14px"}
                            }
                        }
                    }
                }
                TableBody {
                    match tasks() {
                        Some(Some(tasks)) => {
                                if tasks.is_empty() {
                                    rsx!(TableRow { TableCell { "No tasks found" } })
                                } else {
                                    let tasks = tasks.clone();
                                    rsx!(
                                        {tasks.iter().map(|task| {
                                            let uuid = task.uuid.clone();
                                            let name = task.name.clone();
                                            let desc = task.description.clone().unwrap_or_default();
                                            let current_name = use_signal(|| name.clone());
                                            let current_desc = use_signal(|| desc.clone());
                                            let mut is_open_delete = use_signal(|| false);
                                            let mut is_open_update = use_signal(|| false);

                                            let update_uuid = uuid.clone();
                                            let delete_uuid = uuid.clone();
                                            rsx!(TableRow {
                                                class: "cursor-pointer",
                                                onclick: move |_| {
                                                    if !(is_open_delete)() && !(is_open_update)() {
                                                        navigator().push(Route::Tasks { uuid: uuid.clone() });
                                                    }
                                                },

                                                TableCell {
                                                    class: Some("font-medium".to_string()),
                                                    {task.uuid.clone()}
                                                }
                                                TableCell { {name} }
                                                TableCell { {desc} }
                                                TableCell {
                                                    class: Some("text-right".to_string()),
                                                    Button {
                                                        onclick: move |evt: MouseEvent| {
                                                            evt.stop_propagation();
                                                            is_open_update.set(true)
                                                        },
                                                        class: "mr-2",
                                                        Settings { size: "14px".to_string() }
                                                    }
                                                    Button {
                                                        onclick: move |evt: MouseEvent| {
                                                            evt.stop_propagation();
                                                            is_open_delete.set(true)
                                                        },
                                                        Cross { size: "14px".to_string() }
                                                    }
                                                    if (is_open_update)() {
                                                        Dialog {
                                                            is_open:is_open_update,
                                                            title: format!("Update Task {}", task.uuid),
                                                            inputs: Some([
                                                                InputProps { name: String::from("name"), value: current_name},
                                                                InputProps { name: String::from("desc"), value: current_desc},
                                                            ].to_vec()),
                                                            // onclick: Some(EventHandler::new(move |_| {
                                                            //     let uuid = update_uuid.clone();
                                                            //     spawn_local(async move {
                                                            //         let task = fetch::tasks::UpdateBoard {
                                                            //             name: Some(current_name()),
                                                            //             description: Some(current_desc()),
                                                            //         };

                                                            //         match fetch::boards::update_board(&uuid ,board).await {
                                                            //             Ok(true) => refetch_signal.set( refetch_signal() + 1),
                                                            //             Ok(false) => tracing::error!("Update board failed"),
                                                            //             Err(err) => tracing::error!("Update board failed with: {err}")
                                                            //         }
                                                            //     });
                                                            // })),
                                                            div {
                                                                class:"
                                                                    h-1 w-full
                                                                    bg-border-light dark:bg-border-dark
                                                                    " 
                                                            }

                                                        }
                                                    }
                                                    if (is_open_delete)() {
                                                        Dialog {
                                                            is_open:is_open_delete,
                                                            title: format!("Delete Task {}", task.uuid),
                                                            // onclick: Some(EventHandler::new(move |_| {
                                                            //     let uuid = delete_uuid.clone();
                                                            //     spawn_local(async move {
                                                            //         match fetch::boards::delete_board(&uuid).await {
                                                            //             Ok(true) => refetch_signal.set( refetch_signal() + 1),
                                                            //             Ok(false) => tracing::error!("Delete board failed"),
                                                            //             Err(err) => tracing::error!("Delete board failed with: {err}")
                                                            //         }
                                                            //     });
                                                            // })),
                                                            div {
                                                                class:"
                                                                    h-1 w-full
                                                                    bg-border-light dark:bg-border-dark
                                                                    " 
                                                            }

                                                        }
                                                    }

                                                }
                                            })
                                        })}
                                )
                            }
                        },
                        _ => rsx!(TableRow { TableCell { "Loading..." } }),
                    }
                }
            }
        }
        if (is_open_add)() {
            Dialog {
                is_open:is_open_add,
                title: "Add Task",
                inputs: Some([
                    InputProps { name: String::from("name"), value: new_name},
                    InputProps { name: String::from("desc"), value: new_desc},
                ].to_vec()),
                onclick: Some(EventHandler::new(move |_| {
                    let uuid = uuid.clone();
                    spawn_local(async move {
                        let task = fetch::tasks::NewTask {
                            name: new_name(),
                            description: Some(new_desc()),
                            list_uuid: uuid,
                            position: 1,
                            status: String::from("Todo"),
                        };

                        match fetch::tasks::add_task(task).await {
                            Ok(_) => refetch_signal.set( refetch_signal() + 1),
                            Err(err) => tracing::error!("Add task failed with: {err}"),
                        }
                    });
                })),
                div {
                    class:"
                        h-1 w-full
                        bg-border-light dark:bg-border-dark
                    " 
                }

            }
        }
    }
}
