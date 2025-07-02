use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::fa_solid_icons::{FaCheck, FaChevronDown, FaChevronRight, FaCircle, FaPlus, FaXmark},
};

use crate::{
    Route,
    components::{
        Button, Card, Dialog, DialogClose, DialogContent, DialogFooter, DialogHeader, DialogTitle,
        Dropdown, DropdownContent, DropdownTrigger, HoverCard, HoverCardContent, Input, Label,
        SearchDropdown, SearchDropdownContent, SearchDropdownInput, Table, TableBody, TableCaption,
        TableHead, TableHeader, TableRow, Textarea,
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
                class: "h-full grid gap-4 grid-cols-4",
                // class: "grid gap-4 grid-cols-[repeat(auto-fit,_minmax(26rem,_1fr))] h-full",
                {lists.into_iter().map(|list| rsx!(
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
            Button {
                class: "px-2 text-base",
                onclick: move |_| { navigator().push(Route::Home { }); },
                "Boards"
            }
            Icon {
                class: "text-secondary",
                height: 20,
                icon: FaChevronRight,
            }
            Label {
                class: "w-fit px-2 truncate text-base py-1.5",
                {uuid}
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
        Card {
            class: "h-fit flex flex-col gap-2",
            width: "w-96",
            div {
                class: "flex flex-col text-sm font-medium gap-2 w-full",
                Label {
                    variant: "title_1",
                    class: "px-2 pb-2 text-base",
                    width: "w-full",
                    div {
                        class: "break-all",
                        "{list.name}"
                    }
                }
                HoverCard {
                    Label {
                        variant: "outline",
                        class: "p-2",
                        width: "w-full",
                        div {
                            class: "truncate",
                            "UUID: {list.uuid}"
                        }
                    }
                     HoverCardContent {
                        {list.uuid.clone()}
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
                            class: "flex-1 text-base bg-primary-2",
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
                        class: "text-base justify-between pl-2 pr-1",
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
                    class: "cursor-pointer",
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
                            HoverCard {
                                div {
                                    class: "w-8 flex justify-end gap-1 block group-hover:hidden",
                                    {task.tags.iter().enumerate().map(|(id, tag)| {
                                        if id < 2 {
                                            rsx!(
                                                Icon {
                                                    style: format!("--tag-color: {};", tag.color),
                                                    class: format!("text-[var(--tag-color)]"),
                                                    height: 10,
                                                    width: 10,
                                                    icon: FaCircle
                                                }
                                            )
                                        } else if id == 2 {
                                            rsx!(
                                                Icon {
                                                    class: format!("text-secondary"),
                                                    height: 14,
                                                    width: 14,
                                                    icon: FaPlus
                                                }
                                            )
                                        } else {
                                            rsx!()
                                        }
                                    })}
                                }
                                HoverCardContent {
                                    {task.tags.iter().map(|tag| {
                                        rsx!(
                                            div {
                                                class: "flex items-center gap-1 p-0.5",
                                                {tag.name.clone()}
                                                Icon {
                                                    style: format!("--tag-color: {};", tag.color),
                                                    class: format!("text-[var(--tag-color)]"),
                                                    height: 10,
                                                    width: 10,
                                                    icon: FaCircle
                                                },

                                            }
                                        )
                                    })}
                                }
                            }

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
                    class: "text-sm",
                    "Tasks"
                }
                TableHeader {
                    class: "font-semibold text-base text-secondary",
                    TableRow {
                        draggable: false,
                        TableHead {
                            "Tasks"
                        }
                    }
                }
                TableBody {
                    class: "w-full font-medium text-base text-secondary max-h-64",
                    {tasks_element}
                }
            }
        }
    }
}

#[component]
fn TaskSettings(is_open: Signal<bool>) -> Element {
    let name = use_signal(|| "T".to_string());
    let desc = use_signal(|| "".to_string());
    let search = use_signal(|| "".to_string());
    let is_status_open = use_signal(|| false);
    let mut status = use_signal(|| Status::Todo);
    let is_priority_open = use_signal(|| false);
    let mut priority = use_signal(|| Priority::Low);
    let mut adding_tag = use_signal(|| false);
    rsx! {
        Dialog {
            is_open: is_open,
            DialogContent {
                id: "settings-task-area",
                class: "sm:max-w-[425px]",
                DialogHeader {
                    DialogTitle { "Task settings" }
                }
                div {
                    class: "flex flex-col gap-4",
                    Input {
                        width: "w-full",
                        placeholder: "Enter board name",
                        value: name,
                    }
                    Textarea {
                        width: "w-full",
                        placeholder: "Enter board description",
                        value: desc,
                    }
                    Dropdown {
                        is_open: is_status_open,
                        class: "text-base",
                        options: [
                            (Status::Todo.to_string(), Some(EventHandler::new(move |_| status.set(Status::Todo)))),
                            (Status::InProgress.to_string(), Some(EventHandler::new(move |_| status.set(Status::InProgress)))),
                            (Status::Done.to_string(), Some(EventHandler::new(move |_| status.set(Status::Done))))
                        ].to_vec(),
                        DropdownTrigger {
                            width: "w-full",
                            name: status().to_string(),
                            Icon { height: 14, width: 14,icon: FaChevronDown }
                        }
                        DropdownContent {
                            id: "recent-boards-area",
                        }
                    }
                    Dropdown {
                        is_open: is_priority_open,
                        class: "text-base",
                        options: [
                            (Priority::Low.to_string(), Some(EventHandler::new(move |_| priority.set(Priority::Low)))),
                            (Priority::Medium.to_string(), Some(EventHandler::new(move |_| priority.set(Priority::Medium)))),
                            (Priority::High.to_string(), Some(EventHandler::new(move |_| priority.set(Priority::High))))
                        ].to_vec(),
                        DropdownTrigger {
                            width: "w-full",
                            name: priority().to_string(),
                            Icon { height: 14, width: 14,icon: FaChevronDown }
                        }
                        DropdownContent {
                            id: "recent-boards-area",
                        }
                    }
                    div {
                        class: "flex items-center gap-2",
                        SearchDropdown {
                            value: search,
                            class: "text-base",
                            SearchDropdownInput {
                                width: "w-full",
                                placeholder: " Search tags",
                            }
                            SearchDropdownContent {
                                id: "search-boards-area",
                            }
                        }
                        Button {
                            class: "px-1 h-full",
                            onclick: move |_| {
                                adding_tag.set(false);
                                // on_submit();
                            },
                            Icon {
                                class: "text-primary",
                                height: 12,
                                icon: FaCheck,
                            }
                        }
                    }

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
                if adding_tag() {
                } else {
                }
            }
        }
    }
}
