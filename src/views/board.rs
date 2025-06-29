use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::fa_solid_icons::{FaCheck, FaChevronRight, FaCircle, FaPenToSquare, FaPlus, FaXmark},
};

use crate::{
    components::{
        Button, Dialog, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader,
        DialogState, DialogTitle, DialogTrigger, Input, Label, Table, TableBody, TableCaption,
        TableHead, TableHeader, TableRow,
    },
    hooks::use_click_outside,
    models::{Priority, Status, Task},
};

#[component]
pub fn Board(uuid: String) -> Element {
    let lists = ["Backend", "Frontend", "Miscellaneous"].to_vec();
    let mut adding_task = use_signal(|| false);
    let mut is_task_settings_open = use_signal(|| false);

    use_click_outside(
        "add-task-area".to_string(),
        adding_task,
        EventHandler::new(move |_| adding_task.set(false)),
    );

    use_click_outside(
        "settings-task-area".to_string(),
        is_task_settings_open,
        EventHandler::new(move |_| is_task_settings_open.set(false)),
    );

    rsx! {
        div {
            class: "p-4 h-full bg-primary border-2 border-secondary flex flex-col gap-4",
            Header { uuid }
            div {
                class: "grid gap-4 grid-cols-[repeat(auto-fit,_minmax(26rem,_1fr))] h-full",
                {lists.into_iter().enumerate().map(|(id, list)| rsx!(
                    List {
                        name: "{list}",
                        r#key: "{id}",
                        adding_task,
                        is_task_settings_open,
                    }
                ))}
                TaskSettings { is_open: is_task_settings_open}
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
fn List(
    name: String,
    r#key: String,
    adding_task: Signal<bool>,
    is_task_settings_open: Signal<bool>,
) -> Element {
    let mut input_text = use_signal(|| "".to_string());
    let mut tasks = use_signal(|| {
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

    let mut on_submit = move || {
        if !input_text().is_empty() {
            tasks.with_mut(|list| {
                list.push(Task {
                    text: input_text(),
                    priority: Priority::Low,
                    status: Status::Todo,
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
            key: r#key,
            class: "h-fit w-96 p-2 bg-primary-2 border-2 border-secondary flex flex-col gap-2",
            div {
                class: "flex text-sm font-medium gap-2 w-full",
                Label {
                    variant: "outline",
                    class: "p-2",
                    width: "w-1/2",
                    div {
                        class: "truncate",
                        "Name: {name}"
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
                            class: "w-full flex items-center h-full gap-4",
                            {<Priority as Into<Element>>::into(task.priority.clone())}
                            {<Status as Into<Element>>::into(task.status.clone())}
                            div {
                                class: "flex-grow",
                                {task.text.clone()}
                            }
                            div {
                                class: "w-fit flex flex-wrap justify-end gap-0.5",
                                Icon { class: "text-red-800", height: 6, width: 6, icon: FaCircle },
                                Icon { class: "text-red-800", height: 6, width: 6, icon: FaCircle },
                            }
                            Icon {
                                class: "text-secondary hidden group-hover:block",
                                height: 14,
                                icon: FaPenToSquare
                            },
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
