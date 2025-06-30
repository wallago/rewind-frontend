use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::fa_solid_icons::{FaCheck, FaChevronRight, FaCircle, FaPenToSquare, FaPlus, FaXmark},
};

use crate::{
    components::{
        Button, Dialog, DialogClose, DialogContent, DialogFooter, DialogHeader, DialogState,
        DialogTitle, HoverCard, HoverCardContent, Input, Label, Table, TableBody, TableCaption,
        TableHead, TableHeader, TableRow,
    },
    hooks::use_click_outside,
    models::{List, Priority, Status, Tag, Task},
};

#[component]
pub fn Board(uuid: String) -> Element {
    let lists = [
        List {
            name: String::from("Backend"),
            uuid: String::from("23221150-a181-4592-99e7-fbbb76ad3772"),
        },
        List {
            name: String::from("Backend"),
            uuid: String::from("23221199-a181-4592-99e7-fbbb76ad3772"),
        },
        List {
            name: String::from("Backend"),
            uuid: String::from("23221149-a181-4592-99e7-fbbb76ad3772"),
        },
        List {
            name: String::from("Backend"),
            uuid: String::from("23221149-d181-4592-99e7-fbbb76ad3772"),
        },
    ]
    .to_vec();
    let mut is_task_settings_open = use_signal(|| false);

    use_click_outside(
        "settings-task-area".to_string(),
        move || is_task_settings_open(),
        EventHandler::new(move |_| is_task_settings_open.set(false)),
    );

    rsx! {
        div {
            class: "p-4 h-full bg-primary border-2 border-secondary flex flex-col gap-4",
            Header { uuid }
            div {
                class: "grid gap-4 grid-cols-[repeat(auto-fit,_minmax(26rem,_1fr))] h-full",
                {lists.into_iter().map(|(list)| rsx!(
                    ListCard {
                        list,
                        is_task_settings_open,
                    }
                ))}
            }
        }
        TaskSettings { is_open: is_task_settings_open}
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
fn ListCard(list: List, is_task_settings_open: Signal<bool>) -> Element {
    let mut adding_task = use_signal(|| false);
    let mut input_text = use_signal(|| "".to_string());
    let mut tasks = use_signal(|| {
        [
            Task {
                name: String::from("Websocket with Actix"),
                priority: Priority::Low,
                status: Status::Todo,
                tags: [Tag {
                    name: String::from("tech"),
                    color: String::from("#FFAA00"),
                }]
                .to_vec(),
            },
            Task {
                name: String::from("Websocket with Actix"),
                priority: Priority::Medium,
                status: Status::InProgress,
                tags: [
                    Tag {
                        name: String::from("tech"),
                        color: String::from("#FFFF00"),
                    },
                    Tag {
                        name: String::from("tech"),
                        color: String::from("#AB0000"),
                    },
                    Tag {
                        name: String::from("tech"),
                        color: String::from("#FFAA00"),
                    },
                ]
                .to_vec(),
            },
            Task {
                name: String::from("Websocket with Actix"),
                priority: Priority::High,
                status: Status::Done,
                tags: [Tag {
                    name: String::from("tech"),
                    color: String::from("#FFAA00"),
                }]
                .to_vec(),
            },
        ]
        .to_vec()
    });

    use_click_outside(
        "add-task-area".to_string(),
        move || adding_task(),
        EventHandler::new(move |_| adding_task.set(false)),
    );

    let mut on_submit = move || {
        if !input_text().is_empty() {
            tasks.with_mut(|list| {
                list.push(Task {
                    name: input_text(),
                    priority: Priority::Low,
                    status: Status::Todo,
                    tags: [Tag {
                        name: String::from("tech"),
                        color: String::from("#FFAA00"),
                    }]
                    .to_vec(),
                })
            });
            input_text.set(String::new());
            adding_task.set(false);
        }
    };

    let on_submit_by_key = {
        move |_: KeyboardEvent| {
            on_submit();
        }
    };

    rsx! {
        div {
            key: format!("{}",list.uuid),
            class: "h-fit w-96 p-2 bg-primary-2 border-2 border-secondary flex flex-col gap-2",
            div {
                class: "flex text-sm font-medium gap-2 w-full",
                Label {
                    variant: "outline",
                    class: "p-2",
                    width: "w-1/2",
                    div {
                        class: "truncate",
                        "Name: {list.name}"
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
            Tasks { tasks: tasks(), is_settings_open: is_task_settings_open}
            div {
                class: "w-full flex justify-end px-2 pb-2",
                if adding_task() {
                    div {
                        id: "add-task-area",
                        class: "flex w-full gap-4 items-center",
                        Input {
                            class: "flex-1 text-sm",
                            value: input_text,
                            onenter: on_submit_by_key

                        }
                        Button {
                            class: "px-1 h-full",
                            onclick: move |_| {
                                adding_task.set(false);
                                on_submit();
                            },
                            Icon {
                                class: "text-primary",
                                height: 12,
                                icon: FaCheck,
                            }
                        }
                        Button {
                            class: "px-1 h-full",
                            onclick: move |_| adding_task.set(false),
                            Icon {
                                class: "text-primary",
                                height: 12,
                                icon: FaXmark,
                            }
                        }
                    }
                } else {
                    Button {
                        onclick: move |_| adding_task.set(true),
                        class: "text-sm justify-between pl-2 pr-1",
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
fn Tasks(tasks: Vec<Task>, is_settings_open: Signal<bool>) -> Element {
    let tasks_element = tasks.into_iter().enumerate().map(|(id, task)| {
        rsx!(
            div {
                key: "{id}",
                TableRow {
                    class: "group cursor-pointer",
                    onclick: move |_| { is_settings_open.set(true) },
                    div {
                        class: "flex items-center gap-2",
                        div {
                            class: "w-full flex items-center h-full gap-4",
                            {<Priority as Into<Element>>::into(task.priority.clone())}
                            {<Status as Into<Element>>::into(task.status.clone())}
                            div {
                                class: "flex-grow flex-shrink inline-block truncate",
                                {task.name.clone()}
                            }
                            div {
                                class: "w-fit flex justify-end gap-1",
                                {task.tags.iter().map(|tag| {
                                    rsx!(
                                        HoverCard {
                                            Icon {
                                                style: format!("--tag-color: {};", tag.color),
                                                class: format!("text-[var(--tag-color)]"),
                                                height: 10,
                                                width: 10,
                                                icon: FaCircle
                                            },
                                            HoverCardContent {
                                                {tag.name.clone()}
                                            }
                                        }
                                    )
                                })}
                            }
                        }
                        Icon {
                            class: "text-secondary hidden group-hover:block",
                            height: 14,
                            icon: FaPenToSquare
                        }
                    }
                }
            }
        )
    });

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
                        draggable: false,
                        TableHead {
                            "Tasks"
                        }
                    }
                }
                TableBody {
                    class: "w-full font-medium text-sm text-secondary max-h-64",
                    {tasks_element}
                }
            }
        }
    }
}

#[component]
fn TaskSettings(is_open: Signal<bool>) -> Element {
    use_context_provider(|| DialogState(is_open));
    rsx! {
        Dialog {
            DialogContent {
                id: "settings-task-area",
                class: "sm:max-w-[425px]",
                DialogHeader {
                    DialogTitle { "Task settings" }
                }
                DialogFooter {
                    DialogClose {}
                    Button {
                        r#type:"submit",
                        variant: "outline",
                        class: "font-semibold px-2 text-sm",
                        "Save"
                    }
                }
            }
        }
    }
}
