use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::fa_solid_icons::{FaCheck, FaChevronRight, FaPlus, FaXmark},
};

use crate::{
    Route,
    api::{
        add_task, get_board_by_uuid, get_lists_by_board_uuid, get_tags, get_tasks_by_list_uuid,
        switch_lists, update_list,
    },
    components::{
        Button, Card, HoverCard, HoverCardContent, Input, Label, Table, TableBody, TableCaption,
        TableHead, TableHeader, TableRow,
    },
    context::ListsContext,
    hooks::use_click_outside,
    models::{List, NewTask, Priority, Status, Tag, Task, UpdateList},
};

mod modals;

#[component]
pub fn Board(uuid: String) -> Element {
    use_context_provider(|| ListsContext {
        lists: Signal::new(Vec::new()),
        refresh: Signal::new(()),
    });
    let ctx_lists = use_context::<ListsContext>();
    let _ = use_resource({
        let mut lists = ctx_lists.lists.clone();
        let refresh = ctx_lists.refresh.clone();
        let board_uuid = uuid.clone();
        move || {
            let board_uuid = board_uuid.clone();
            async move {
                refresh();
                match get_lists_by_board_uuid(board_uuid).await {
                    Ok(fetched) => lists.set(fetched),
                    Err(err) => tracing::error!("{err}"),
                }
            }
        }
    });

    // let mut lists = use_signal(|| None::<Vec<List>>);
    // let mut tags = use_signal(|| None::<Vec<Tag>>);
    // use_future(move || async move {
    //     match get_tags().await {
    //         Ok(res) => tags.set(Some(res)),
    //         Err(err) => tracing::error!("{err}"),
    //     }
    // });

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

    let dragging_index = use_signal(|| None::<String>);
    let lists: Vec<Element> = (ctx_lists.lists)()
        .iter()
        .map(|list| {
            rsx!(ListCard {
                list: list.clone(),
                is_task_settings_open,
                dragging_from: dragging_index
            })
        })
        .collect();

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
                {lists.iter()}
            }
        }
        modals::AddList { is_open: is_add_list_open, board_uuid: uuid.clone() }
        // modals::TaskSettings {
        //     is_open: is_task_settings_open,
        //     tags: tags().unwrap_or(Vec::new()),
        // }
    }
}

#[component]
fn Header(uuid: String) -> Element {
    let board = use_resource(move || {
        let uuid = uuid.clone();
        async move {
            match get_board_by_uuid(uuid.clone()).await {
                Ok(fetched) => Some(fetched),
                Err(err) => {
                    tracing::error!("{err}");
                    None
                }
            }
        }
    });

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
            {
                match board() {
                Some(Some(board)) => rsx!(
                    Label {
                        class: "w-fit px-2 truncate text-base py-1.5",
                        {board.name}
                    }
                ),
                _ => rsx!(
                    Label {
                        class: "w-fit px-2 truncate text-base py-1.5",
                        "Loading"
                    }
                )
            }
            }
            // if let Some(board) = board() {
            //     Label { class: "w-fit px-2 truncate text-base py-1.5", {board.name} }
            // } else {
            //     Label { class: "w-fit px-2 truncate text-base py-1.5", "Loading" }
            // }
        }
    }
}

#[component]
fn ListCard(
    list: List,
    is_task_settings_open: Signal<bool>,
    dragging_from: Signal<Option<String>>,
) -> Element {
    let mut adding_task = use_signal(|| false);
    let name = use_signal(|| list.name.clone());

    let mut tasks = use_signal(|| None::<Vec<Task>>);
    let list_copy = list.clone();
    let list_uuid = list.uuid.clone();
    let list_uuid_copy = list.uuid.clone();
    let list_uuid_from = list.uuid.clone();
    let list_uuid_to = list.uuid.clone();
    let id = list.uuid.clone();
    let mut is_delete_open = use_signal(|| false);
    let mut is_update = use_signal(|| false);
    let mut dragging_to = use_signal(|| None::<String>);

    // use_future(move || {
    //     let uuid = list_uuid.clone();
    //     async move {
    //         match get_tasks_by_list_uuid(uuid).await {
    //             Ok(res) => tasks.set(Some(res)),
    //             Err(err) => tracing::error!("{err}"),
    //         }
    //     }
    // });

    use_click_outside(
        "add-task-area".to_string(),
        move || adding_task(),
        EventHandler::new(move |_| adding_task.set(false)),
    );

    use_click_outside(
        "delete-list-area".to_string(),
        move || is_delete_open(),
        EventHandler::new(move |_| is_delete_open.set(false)),
    );

    use_click_outside(
        "update-list-area".to_string(),
        move || is_update(),
        EventHandler::new(move |_| is_update.set(false)),
    );

    let mut ctx_lists = use_context::<ListsContext>();
    let mut trigger_update = use_signal(|| false);
    let mut in_progress_update = use_signal(|| false);
    let mut trigger_switch = use_signal(|| false);
    let mut in_progress_switch = use_signal(|| false);
    let mut trigger_add_task = use_signal(|| false);
    let mut in_progress_add_task = use_signal(|| false);

    let _ = use_resource(move || {
        let list_uuid = list_uuid.clone();
        async move {
            if trigger_update() {
                in_progress_update.set(true);
                match update_list(
                    &list_uuid,
                    UpdateList {
                        name: Some(name()),
                        position: None,
                    },
                )
                .await
                {
                    Ok(_) => ctx_lists.refresh.set(()),
                    Err(err) => tracing::error!("{err}"),
                };
                in_progress_update.set(false);
            }
        }
    });

    use_effect(move || {
        if !in_progress_update() {
            trigger_update.set(false);
        }
    });

    let _ = use_resource(move || async move {
        if trigger_switch()
            && let Some(uuid_from) = dragging_from()
            && let Some(uuid_to) = dragging_to()
        {
            in_progress_switch.set(true);
            match switch_lists(&uuid_from, &uuid_to).await {
                Ok(_) => ctx_lists.refresh.set(()),
                Err(err) => tracing::error!("{err}"),
            };
            in_progress_switch.set(false);
        }
    });

    use_effect(move || {
        if !in_progress_switch() {
            dragging_from.set(None);
            dragging_to.set(None);
            trigger_switch.set(false);
        }
    });

    // let mut add = use_signal(|| false);
    // use_future(move || {
    //     let uuid = list_uuid_copy.clone();
    //     async move {
    //         if !add() {
    //             return ();
    //         } else {
    //             add.set(false);
    //         }
    //         match add_task(NewTask {
    //             name: name(),
    //             list_uuid: uuid.clone(),
    //             position: None,
    //             status: None,
    //             priority: None,
    //         })
    //         .await
    //         {
    //             Ok(_) => (),
    //             Err(err) => tracing::error!("{err}"),
    //         }
    //     }
    // });

    rsx! {
        div {
        id,
            draggable: true,
            ondragstart: move |_| {
                dragging_from.set(Some(list_uuid_from.clone()));
            },
            ondragover: move |evt| {
                evt.prevent_default();
            },
            ondrop: move |_| {
                if dragging_from().is_some() {
                    dragging_to.set(Some(list_uuid_to.clone()));
                    trigger_switch.set(true);
                }
            },
        Card { class: "h-fit flex flex-col gap-2 mx-auto", width: "w-96",
            div { class: "flex flex-col justify-center text-sm font-medium gap-2 w-full",
                div { class: "flex gap-3 justify-between h-full items-center pb-1",
                    if !is_update() {
                        Button {
                            variant: "ghost",
                            class: "text-base",
                            width: "w-fit",
                            onclick: move |_| is_update.set(true),
                            div { class: "break-all", "{list.name}" }
                        }
                        Button {
                            class: "px-1 h-fit py-1 my-1",
                            onclick: move |_| is_delete_open.set(true),
                            Icon { height: 16, width: 16, icon: FaXmark }
                        }
                    } else {
                        Input {
                            id: "update-list-area",
                            width: "w-full",
                            placeholder: "Enter board name",
                            value: name,
                            onenter: EventHandler::new(move |_e: KeyboardEvent| {
                                trigger_update.set(true);
                                is_update.set(false);
                            }),
                        }
                        Button {
                            class: "px-1 h-fit py-1 my-1",
                            onclick: move |_| {
                                trigger_update.set(true);
                                is_update.set(false)
                            },
                            Icon { height: 16, width: 16, icon: FaCheck }
                        }
                        Button {
                            class: "px-1 h-fit py-1 my-1",
                            onclick: move |_| is_update.set(false),
                            Icon { height: 16, width: 16, icon: FaXmark }
                        }
                    }
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
                                trigger_add_task.set(true);
                                adding_task.set(false);
                            }),

                        }
                        Button {
                            class: "px-1 h-full",
                            onclick: move |_| {
                                trigger_add_task.set(true);
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
            modals::DeleteList { list: list_copy, is_open: is_delete_open }
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
