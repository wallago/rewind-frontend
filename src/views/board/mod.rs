use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::fa_solid_icons::{FaCheck, FaChevronRight, FaPlus, FaXmark},
};

use crate::{
    Route,
    api::{add_task, get_lists_by_board_uuid, get_tags, get_tasks_by_list_uuid},
    components::{
        Button, Card, HoverCard, HoverCardContent, Input, Label, Table, TableBody, TableCaption,
        TableHead, TableHeader, TableRow,
    },
    hooks::use_click_outside,
    models::{List, NewTask, Priority, Status, Tag, Task},
};

mod modals;

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
        div { class: "p-4 h-full bg-primary border-2 border-secondary flex flex-col gap-4",
            div { class: "flex justify-between",
                Header { uuid: uuid.clone() }
                Button {
                    class: "px-2 justify-between gap-2 font-semibold text-base",
                    width: "w-24",
                    onclick: move |_| is_add_list_open.set(true),
                    "List"
                    Icon { height: 14, width: 14, icon: FaPlus }
                }
            }
            div { class: "overflow-y-auto  h-full grid gap-4 grid-cols-1 sm:grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-3",
                if let Some(lists) = lists() {
                    {
                        lists
                            .iter()
                            .map(|list| {
                                rsx! {
                                    ListCard { list: list.clone(), is_task_settings_open }
                                }
                            })
                    }
                }
            }
        }
        modals::AddList { is_open: is_add_list_open, board_uuid: uuid.clone() }
        modals::TaskSettings {
            is_open: is_task_settings_open,
            tags: tags().unwrap_or(Vec::new()),
        }
    }
}

#[component]
fn Header(uuid: String) -> Element {
    rsx! {
        div { class: "flex gap-4 items-center",
            Button {
                class: "px-2 text-base",
                onclick: move |_| {
                    navigator().push(Route::Home {});
                },
                "Boards"
            }
            Icon { class: "text-secondary", height: 20, icon: FaChevronRight }
            Label { class: "w-fit px-2 truncate text-base py-1.5", {uuid} }
        }
    }
}

#[component]
fn ListCard(list: List, is_task_settings_open: Signal<bool>) -> Element {
    let mut adding_task = use_signal(|| false);
    let name = use_signal(|| "".to_string());

    let mut tasks = use_signal(|| None::<Vec<Task>>);
    let list_uuid = list.uuid.clone();
    let list_uuid_copy = list.uuid.clone();
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

    let mut add = use_signal(|| false);
    use_future(move || {
        let uuid = list_uuid_copy.clone();
        async move {
            if !add() {
                return ();
            } else {
                add.set(false);
            }
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

    rsx! {
        Card { class: "h-fit flex flex-col gap-2", width: "w-96",
            div { class: "flex flex-col text-sm font-medium gap-2 w-full",
                Label {
                    variant: "title_1",
                    class: "px-2 pb-2 text-base",
                    width: "w-full",
                    div { class: "break-all", "{list.name}" }
                }
                HoverCard {
                    Label { variant: "outline", class: "p-2", width: "w-full",
                        div { class: "truncate", "UUID: {list.uuid}" }
                    }
                    HoverCardContent { {list.uuid.clone()} }
                }
            }
            Tasks { tasks: tasks(), is_settings_open: is_task_settings_open }
            div { class: "w-full flex justify-end px-2 pb-2",
                if adding_task() {
                    div { class: "flex w-full gap-4 items-center",
                        Input {
                            class: "flex-1 text-base bg-primary-2",
                            value: name,
                            onenter: EventHandler::new(move |_e: KeyboardEvent| {
                                add.set(true);
                                adding_task.set(false);
                            }),
                        
                        }
                        Button {
                            class: "px-1 h-full",
                            onclick: move |_| {
                                add.set(true);
                                adding_task.set(false);
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
                        Icon { class: "text-primary", height: 12, icon: FaPlus }
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
                TableCaption { class: "text-sm", "Tasks" }
                TableHeader { class: "font-semibold text-base text-secondary",
                    TableRow { draggable: false,
                        TableHead { "Tasks" }
                    }
                }
                TableBody { class: "w-full font-medium text-base text-secondary max-h-64",

                    if let Some(tasks) = tasks {
                        {
                            tasks
                                .into_iter()
                                .map(|task| {
                                    rsx! {
                                        div {
                                            TableRow {
                                                class: "cursor-pointer",
                                                onclick: move |_| { is_settings_open.set(true) },
                                                div { class: "flex items-center gap-2",
                                                    div { class: "w-full flex items-center h-full gap-4",
                                                        {<Priority as Into<Element>>::into(task.priority.clone())}
                                                        {<Status as Into<Element>>::into(task.status.clone())}
                                                        div { class: "flex-grow flex-shrink inline-block truncate", {task.name.clone()} }
                                                    
                                                    }
                                                }
                                            }
                                        }
                                    }
                                })
                        }
                    }
                }
            }
        }
    }
}
