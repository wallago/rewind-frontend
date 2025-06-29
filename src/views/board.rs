use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::{
        fa_regular_icons::FaCircle as FaCircleEmpty,
        fa_solid_icons::{
            FaCheck, FaChevronRight, FaCircle, FaCircleHalfStroke, FaPlus, FaTag, FaXmark,
        },
    },
};

use crate::components::{
    Button, Input, Label, Table, TableBody, TableCaption, TableCell, TableHead, TableHeader,
    TableRow,
};

#[derive(Clone, PartialEq)]
enum Status {
    Todo,
    InProgress,
    Done,
}

impl Into<Element> for Status {
    fn into(self) -> Element {
        match self {
            Status::Todo => rsx!(Icon {
                class: "text-secondary",
                height: 16,
                icon: FaCircleEmpty,
            }),
            Status::InProgress => rsx!(Icon {
                class: "text-secondary",
                height: 16,
                icon: FaCircleHalfStroke,
            }),
            Status::Done => rsx!(Icon {
                class: "text-secondary",
                height: 16,
                icon: FaCircle,
            }),
        }
    }
}

#[derive(Clone, PartialEq)]
enum Priority {
    Low,
    Medium,
    High,
}

impl Into<Element> for Priority {
    fn into(self) -> Element {
        match self {
            Priority::Low => rsx!(Icon {
                class: "text-priority-low",
                height: 16,
                icon: FaTag,
            }),
            Priority::Medium => rsx!(Icon {
                class: "text-priority-medium",
                height: 16,
                icon: FaTag,
            }),
            Priority::High => rsx!(Icon {
                class: "text-priority-high",
                height: 16,
                icon: FaTag,
            }),
        }
    }
}

#[derive(Clone, PartialEq)]
struct Task {
    text: String,
    priority: Priority,
    status: Status,
}

#[component]
pub fn Board(uuid: String) -> Element {
    rsx! {
        div {
            class: "p-4 h-full bg-primary border-2 border-secondary flex flex-col gap-4",
            Header { uuid }
            div {
                class: "grid gap-4 grid-cols-[repeat(auto-fit,_minmax(26rem,_1fr))] h-full",
                List {}
                List {}
                List {}
            }
        }
    }
}

#[component]
fn Header(uuid: String) -> Element {
    rsx! {
        div {
            class: "flex gap-4 items-center",
            Label {
                "Boards"
            }
            Icon {
                class: "text-secondary",
                height: 20,
                icon: FaChevronRight,
            }
            Label {
                div {
                    class: "w-fit truncate",
                    {uuid}
                }
            }
        }
    }
}

#[component]
fn List() -> Element {
    let mut adding_task = use_signal(|| false);
    let input_text = use_signal(|| "".to_string());
    let tasks = use_signal(|| {
        [
            Task {
                text: String::from("Websocket with Actix"),
                priority: Priority::Low,
                status: Status::Todo,
            },
            Task {
                text: String::from("Websocket with Actix"),
                priority: Priority::Medium,
                status: Status::InProgress,
            },
            Task {
                text: String::from("Websocket with Actix"),
                priority: Priority::High,
                status: Status::Done,
            },
        ]
        .to_vec()
    });

    let on_submit = {
        let mut tasks = tasks.clone();
        let mut input_text = input_text.clone();
        let mut adding_task = adding_task.clone();

        move |_: KeyboardEvent| {
            tracing::info!("ENTER !!!! with {}", input_text());
            if !input_text().is_empty() {
                tasks.with_mut(|list| {
                    list.push(Task {
                        text: input_text(),
                        priority: Priority::Low,
                        status: Status::Todo,
                    })
                });
                input_text.set("".to_string());
                adding_task.set(false);
            }
        }
    };

    rsx! {
        div {
            class: "h-fit w-96 p-2 bg-primary-2 border-2 border-secondary flex flex-col gap-2",
            div {
                class: "flex text-sm font-medium gap-2 w-full",
                Label {
                    variant: "outline",
                    class: "p-2",
                    width: "w-1/2",
                    div {
                        class: "truncate",
                        "Name: Backend"
                    }
                }
                Label {
                    variant: "outline",
                    class: "p-2",
                    width: "w-1/2",
                    div {
                        class: "truncate",
                        "UUID: 571a9fa0-1bb4-4545-bdd3-b7315dcb6615"
                    }
                }
            }
            Tasks { tasks: tasks() }
            div {
                class: "w-full flex justify-end px-2 pb-2",
                if adding_task() {
                    div {
                        class: "flex w-full gap-4 items-center",
                        Input {
                            class: "flex-1",
                            value: input_text,
                            onenter: on_submit

                        }
                        Button {
                            class: "px-1 h-fit py-1.5",
                            onclick: move |_| adding_task.set(false),
                            Icon {
                                class: "text-primary",
                                height: 12,
                                icon: FaCheck,
                            }
                        }
                        Button {
                            class: "px-1 h-fit py-1.5",
                            onclick: move |_| adding_task.set(false),
                            Icon {
                                class: "text-primary",
                                height: 12,
                                icon: FaXmark,
                            }
                        }
                    }
                    // div {
                    //     class: "px-2",
                    //     input {
                    //         class: "w-full text-sm px-2 py-1 border rounded bg-primary",
                    //         value: "{input_text}",
                    //         oninput: move |e| input_text.set(e.value.clone()),
                    //         onkeydown: move |e| {
                    //             if e.key() == "Enter" {
                    //                 on_submit(());
                    //             }
                    //         }
                    //     }
                    // }
                } else {
                    Button {
                        onclick: move |_| adding_task.set(true),
                        class: "text-sm justify-between pl-2 pr-1 mt-0.5 mb-0.5",
                        width: "w-20",
                        "Task"
                        Icon {
                            class: "text-primary",
                            height: 12,
                            icon: FaPlus,
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Tasks(tasks: Vec<Task>) -> Element {
    rsx! {
        div {
            Table {
                TableCaption {
                    class: "text-xs",
                    "Tasks"
                }
                TableHeader {
                    class: "font-semibold text-sm text-secondary",
                    TableRow {
                        TableHead {
                            "Tasks"
                        }
                    }
                }
                TableBody {
                    class: "w-full font-medium text-sm text-secondary max-h-64",
                    {tasks.into_iter().map(|task| rsx!(
                            TableRow {
                                TableCell {
                                    class: "w-fit pr-2",
                                    {<Priority as Into<Element>>::into(task.priority)}
                                }
                                TableCell {
                                    class: "w-full truncate",
                                    {task.text.clone()}
                                }
                                TableCell {
                                    class: "w-fit pl-2",
                                    {<Status as Into<Element>>::into(task.status)}
                                }
                            }
                    ))}
                }
            }
        }
    }
}
