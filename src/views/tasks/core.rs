use dioxus::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::{
    components::{
        Label, fetch,
        icons::{Cross, Empty, Full, Half, Label as LabelIcon},
        layout::Body,
        table::*,
    },
    views::{tags::DialogAdd as DialogAddTag, tasks::dialog::*},
};

#[component]
pub fn Tasks(uuid: String) -> Element {
    let mut refetch_signal = use_signal(|| 0);
    let tasks = fetch::tasks::get_tasks(uuid.clone(), refetch_signal);

    rsx! (
         Body {
            div {
                class: "pt-2",
                p {
                    class: "text-sm font-semibold pb-4",
                    "List: {uuid}"
                }
                DialogAddTag {}
            }
            // div { class: "h-0.5 w-full bg-border-light mb-4" }
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

                                            let dragged_uuid_from = task.uuid.clone();
                                            let dragged_uuid_to = task.uuid.clone();
                                            rsx!(TableRow {

                                                class: "hover:bg-surface-variant-light dark:hover:bg-surface-variant-dark",
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
