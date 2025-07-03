use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::fa_solid_icons::{FaCheck, FaChevronDown, FaChevronRight, FaCircle, FaPlus, FaXmark},
};
use regex::Regex;

use crate::{
    Route,
    api::{add_list, add_task, get_lists_by_board_uuid, get_tags, get_tasks_by_list_uuid},
    components::{
        Button, Card, Dialog, DialogClose, DialogContent, DialogFooter, DialogHeader, DialogTitle,
        Dropdown, DropdownContent, DropdownTrigger, HoverCard, HoverCardContent, Input, Label,
        SearchDropdown, SearchDropdownContent, SearchDropdownInput, Table, TableBody, TableCaption,
        TableHead, TableHeader, TableRow, Textarea,
    },
    hooks::use_click_outside,
    models::{List, NewList, NewTask, Priority, Status, Tag, Task},
};

#[component]
pub fn Board(uuid: String) -> Element {
    let mut lists = use_signal(|| None::<Vec<List>>);
    let mut tags = use_signal(|| None::<Vec<Tag>>);
    use_future(move || async move {
        match get_tags().await {
            Ok(res) => tags.set(Some(res)),
            Err(err) => tracing::error!("{err}"),
        }
    });

    let board_uuid = uuid.clone();
    use_future(move || {
        let uuid = board_uuid.clone();
        async move {
            match get_lists_by_board_uuid(uuid).await {
                Ok(res) => lists.set(Some(res)),
                Err(err) => tracing::error!("{err}"),
            }
        }
    });

    let mut is_task_settings_open = use_signal(|| false);
    let mut is_add_list_open = use_signal(|| false);

    use_click_outside(
        "add-list-area".to_string(),
        move || is_add_list_open(),
        EventHandler::new(move |_| is_add_list_open.set(false)),
    );

    use_click_outside(
        "settings-task-area".to_string(),
        move || is_task_settings_open(),
        EventHandler::new(move |_| is_task_settings_open.set(false)),
    );

    rsx! {
        div {
            class: "p-4 h-full bg-primary border-2 border-secondary flex flex-col gap-4",
            div {
                class: "flex justify-between",
                Header { uuid: uuid.clone() }
                Button {
                    class: "px-2 justify-between gap-2 font-semibold text-base",
                    width: "w-24",
                    onclick: move |_| is_add_list_open.set(true),
                    "List"
                    Icon { height: 14, width: 14,icon: FaPlus }
                }
            }
            div {
                class: "overflow-y-auto  h-full grid gap-4 grid-cols-1 sm:grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-3",
                if let Some(lists) = lists() {
                    {lists.iter().map(|list| {
                       rsx!(
                            ListCard { list: list.clone(), is_task_settings_open }
                        )
                    })}
                }
            }
        }
        AddList { is_open: is_add_list_open, board_uuid: uuid.clone() }
        TaskSettings { is_open: is_task_settings_open, tags: tags().unwrap_or(Vec::new())}
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
    let name = use_signal(|| "".to_string());

    let mut tasks = use_signal(|| None::<Vec<Task>>);
    let list_uuid = list.uuid.clone();
    let list_uuid_copy_1 = list.uuid.clone();
    let list_uuid_copy_2 = list.uuid.clone();
    use_future(move || {
        let uuid = list_uuid.clone();
        async move {
            match get_tasks_by_list_uuid(uuid).await {
                Ok(res) => tasks.set(Some(res)),
                Err(err) => tracing::error!("{err}"),
            }
        }
    });

    use_click_outside(
        "add-task-area".to_string(),
        move || adding_task(),
        EventHandler::new(move |_| adding_task.set(false)),
    );

    let on_submit = move |uuid: String| {
        use_future(move || {
            let uuid = uuid.clone();
            async move {
                match add_task(NewTask {
                    name: name(),
                    list_uuid: uuid.clone(),
                    position: None,
                    status: None,
                    priority: None,
                })
                .await
                {
                    Ok(_) => (),
                    Err(err) => tracing::error!("{err}"),
                }
            }
        });
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
                            value: name,
                            onenter: EventHandler::new(move |_e: KeyboardEvent| {
                                let uuid = list_uuid_copy_1.clone();
                                on_submit(uuid);
                            })

                        }
                        Button {
                            class: "px-1 h-full",
                            onclick: move |_| {
                                let uuid = list_uuid_copy_2.clone();
                                adding_task.set(false);
                                on_submit(uuid);
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
fn Tasks(tasks: Option<Vec<Task>>, is_settings_open: Signal<bool>) -> Element {
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

                    if let Some(tasks) = tasks {
                        {tasks.into_iter().map(|task| {
                            rsx!(
                                div {
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
                                                // HoverCard {
                                                //     div {
                                                //         class: "w-8 flex justify-end gap-1 block group-hover:hidden",
                                                //         {task.tags.iter().enumerate().map(|(id, tag)| {
                                                //             if id < 2 {
                                                //                 rsx!(
                                                //                     Icon {
                                                //                         style: format!("--tag-color: {};", tag.color),
                                                //                         class: format!("text-[var(--tag-color)]"),
                                                //                         height: 10,
                                                //                         width: 10,
                                                //                         icon: FaCircle
                                                //                     }
                                                //                 )
                                                //             } else if id == 2 {
                                                //                 rsx!(
                                                //                     Icon {
                                                //                         class: format!("text-secondary"),
                                                //                         height: 14,
                                                //                         width: 14,
                                                //                         icon: FaPlus
                                                //                     }
                                                //                 )
                                                //             } else {
                                                //                 rsx!()
                                                //             }
                                                //         })}
                                                //     }
                                                //     HoverCardContent {
                                                //         {task.tags.iter().map(|tag| {
                                                //             rsx!(
                                                //                 div {
                                                //                     class: "flex items-center gap-1 p-0.5",
                                                //                     {tag.name.clone()}
                                                //                     Icon {
                                                //                         style: format!("--tag-color: {};", tag.color),
                                                //                         class: format!("text-[var(--tag-color)]"),
                                                //                         height: 10,
                                                //                         width: 10,
                                                //                         icon: FaCircle
                                                //                     },

                                                //                 }
                                                //             )
                                                //         })}
                                                //     }
                                                // }

                                            }
                                        }
                                    }
                                }
                            )
                        })}
                    }
                }
            }
        }
    }
}

#[component]
fn TaskSettings(is_open: Signal<bool>, tags: Vec<Tag>) -> Element {
    let re = Regex::new(r"^#[0-9a-fA-F]{6}$")?;
    let name = use_signal(|| "".to_string());
    let desc = use_signal(|| "".to_string());
    let mut search = use_signal(|| "".to_string());
    let mut is_status_open = use_signal(|| false);
    let mut status = use_signal(|| Status::Todo);
    let mut is_priority_open = use_signal(|| false);
    let mut priority = use_signal(|| Priority::Low);
    let mut adding_tag = use_signal(|| false);
    let new_tag_name = use_signal(|| "".to_string());
    let new_tag_color = use_signal(|| "".to_string());
    let mut is_valid_hex = use_signal(|| false);
    let mut task_tags = use_signal(|| Vec::<Tag>::new());
    let is_search_active = use_memo(move || !search().is_empty());

    use_click_outside(
        "status-task-area".to_string(),
        move || is_status_open(),
        EventHandler::new(move |_| is_status_open.set(false)),
    );

    use_click_outside(
        "priority-task-area".to_string(),
        move || is_priority_open(),
        EventHandler::new(move |_| is_priority_open.set(false)),
    );

    use_click_outside(
        "search-tags-area".to_string(),
        move || is_search_active(),
        EventHandler::new(move |_| search.set("".to_string())),
    );

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
                        id: "status-task-area",
                        is_open: is_status_open,
                        class: "text-base",
                        options: Signal::new([
                            (Status::Todo.to_string(), Some(EventHandler::new(move |_| status.set(Status::Todo)))),
                            (Status::InProgress.to_string(), Some(EventHandler::new(move |_| status.set(Status::InProgress)))),
                            (Status::Done.to_string(), Some(EventHandler::new(move |_| status.set(Status::Done))))
                        ].to_vec()),
                        DropdownTrigger {
                            width: "w-full",
                            name: status().to_string(),
                            Icon { height: 14, width: 14,icon: FaChevronDown }
                        }
                        DropdownContent {}
                    }
                    Dropdown {
                        id: "priority-task-area",
                        is_open: is_priority_open,
                        class: "text-base",
                        options: Signal::new([
                            (Priority::Low.to_string(), Some(EventHandler::new(move |_| priority.set(Priority::Low)))),
                            (Priority::Medium.to_string(), Some(EventHandler::new(move |_| priority.set(Priority::Medium)))),
                            (Priority::High.to_string(), Some(EventHandler::new(move |_| priority.set(Priority::High))))
                        ].to_vec()),
                        DropdownTrigger {
                            width: "w-full",
                            name: priority().to_string(),
                            Icon { height: 14, width: 14,icon: FaChevronDown }
                        }
                        DropdownContent {}
                    }
                    div {
                        class: "flex items-center gap-2 h-full",
                        if !adding_tag() {
                            SearchDropdown {
                                id: "search-tags-area",
                                options: Signal::new(tags.iter().map(|tag| tag.name.clone()).collect()),
                                value: search,
                                class: "text-base",
                                SearchDropdownInput {
                                    width: "w-full",
                                    placeholder: " Search tags",
                                }
                                SearchDropdownContent {}
                            }
                            Button {
                                class: "px-1 h-7 w-7",
                                onclick: move |_| {
                                    adding_tag.set(true);
                                },
                                Icon {
                                    class: "text-primary",
                                    height: 12,
                                    icon: FaPlus,
                                }
                            }
                        } else {
                            Input {
                                width: "w-full",
                                placeholder: "New tag name",
                                value: new_tag_name,
                            }
                            Input {
                                width: "w-full",
                                placeholder: "New tag color",
                                value: new_tag_color,
                                oninput: move |_| {
                                    if !re.is_match(&new_tag_color()) {
                                        is_valid_hex.set(false)
                                    } else {
                                        is_valid_hex.set(true)
                                    }
                                }
                            }
                            Button {
                                class: "px-1 h-7 w-7",
                                onclick: move |_| {
                                    adding_tag.set(false);
                                    task_tags.with_mut(|tag_list| {
                                        tag_list.push(Tag {
                                            name: new_tag_name(),
                                            color: new_tag_color(),
                                        });
                                    });
                                },
                                disabled: !is_valid_hex(),
                                Icon {
                                    class: "text-primary",
                                    height: 12,
                                    icon: FaCheck,
                                }
                            }
                        }
                    }
                    if !tags.is_empty() {
                        div {
                            p {
                                class: "font-semibold pb-2",
                                "Tags"
                            }
                            div {
                                class: "flex flex-col gap-2 max-h-32 overflow-y-auto w-full border-2 border-secondary p-2",
                                {tags.iter().map(|tag| rsx!(
                                    div {
                                        class: "flex gap-2 justify-between items-center w-full px-1",
                                        div {
                                            class: "flex gap-4 items-center w-fit",
                                            Icon {
                                                style: format!("--tag-color: {};", tag.color),
                                                class: format!("text-[var(--tag-color)]"),
                                                height: 10,
                                                width: 10,
                                                icon: FaCircle
                                            },
                                            "{tag.name}"
                                        }
                                        Button {
                                            class: "px-1 h-6 w-6",
                                            // onclick: move |_| adding_task.set(false),
                                            Icon {
                                                class: "text-primary",
                                                height: 12,
                                                icon: FaXmark,
                                            }
                                        }
                                    }
                                ))}
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
            }
        }
    }
}

#[component]
fn AddList(is_open: Signal<bool>, board_uuid: String) -> Element {
    let name = use_signal(|| "".to_string());
    let uuid = board_uuid.clone();
    rsx! {
        Dialog {
            is_open: is_open,
            DialogContent {
                id: "add-list-area",
                class: "sm:max-w-[425px]",
                DialogHeader {
                    DialogTitle { "Add List" }
                }
                Input {
                    width: "w-full",
                    placeholder: "Enter list name",
                    value: name,
                }
                DialogFooter {
                    DialogClose {}
                    Button {
                        onclick: move |_| {
                            let board_uuid = uuid.clone();
                            use_future(move || {
                                let uuid = board_uuid.clone();
                                async move {
                                match add_list(NewList {
                                    name: name(),
                                    position: None,
                                    board_uuid: uuid
                                }).await {
                                    Ok(_) => (),
                                    Err(err) => tracing::error!("{err}"),
                                }
                            }});
                            is_open.set(false);
                        },
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
