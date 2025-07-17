use crate::{
    Route,
    components::{
        Button, Dialog, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader,
        DialogTitle, Input, Label, SearchDropdown, SearchDropdownContent, SearchDropdownInput,
    },
    context::TagsContext,
    hooks::{
        use_click_outside, use_list_add, use_list_delete, use_tag_add, use_task_add,
        use_task_delete, use_task_update,
    },
    models::{List, Task},
};
use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::fa_solid_icons::FaPlus};
use regex::Regex;

#[component]
pub fn AddList(is_open: Signal<bool>, board_uuid: String) -> Element {
    let name = use_signal(|| "".to_string());
    let mut trigger = use_signal(|| false);

    use_list_add(name, board_uuid, trigger);

    use_click_outside(
        "delete-list-area".to_string(),
        move || is_open(),
        EventHandler::new(move |_| is_open.set(false)),
    );

    rsx! {
        Dialog { is_open,
            DialogContent { id: "add-list-area", class: "sm:max-w-[425px]",
                DialogHeader {
                    DialogTitle { "Add List" }
                }
                Input {
                    width: "w-full",
                    placeholder: "Enter list name",
                    value: name,
                    onenter: EventHandler::new(move |_e: KeyboardEvent| {
                        trigger.set(true);
                        is_open.set(false);
                    }),
                }
                DialogFooter {
                    DialogClose {}
                    Button {
                        onclick: move |_| {
                            trigger.set(true);
                            is_open.set(false);
                        },
                        r#type: "submit",
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
pub fn DeleteList(list: List, is_open: Signal<bool>) -> Element {
    let mut trigger = use_signal(|| false);

    use_list_delete(list.uuid, trigger);

    use_click_outside(
        "delete-list-area".to_string(),
        move || is_open(),
        EventHandler::new(move |_| is_open.set(false)),
    );

    rsx! {
        Dialog { is_open,
            DialogContent { id: "delete-list-area", class: "sm:max-w-[425px]",
                DialogHeader {
                    DialogTitle { "Delete {list.name}" }
                    DialogDescription { "Are you sure ?" }
                }
                DialogFooter {
                    DialogClose {}
                    Button {
                        onclick: move |_| {
                            trigger.set(true);
                            is_open.set(false);
                        },
                        r#type: "submit",
                        variant: "outline",
                        class: "font-semibold px-2 text-sm",
                        "Delete"
                    }
                }
            }
        }
    }
}

#[component]
pub fn AddTask(is_open: Signal<bool>, list_uuid: String) -> Element {
    let name = use_signal(|| "".to_string());
    let mut trigger = use_signal(|| false);

    use_task_add(name, list_uuid, trigger);

    use_click_outside(
        "add-task-area".to_string(),
        move || is_open(),
        EventHandler::new(move |_| is_open.set(false)),
    );

    rsx! {
        Dialog { is_open,
            DialogContent { id: "add-task-area", class: "sm:max-w-[425px]",
                DialogHeader {
                    DialogTitle { "Add Task" }
                }
                Input {
                    width: "w-full",
                    placeholder: "Enter task name",
                    value: name,
                    onenter: EventHandler::new(move |_e: KeyboardEvent| {
                        trigger.set(true);
                        is_open.set(false);
                    }),
                }
                DialogFooter {
                    DialogClose {}
                    Button {
                        onclick: move |_| {
                            trigger.set(true);
                            is_open.set(false);
                        },
                        r#type: "submit",
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
pub fn UpdateTask(is_open: Signal<bool>, task: Task) -> Element {
    let name = use_signal(|| task.name.clone());
    let tag_search = use_signal(|| "".to_string());

    let mut is_delete_open = use_signal(|| false);
    let mut is_add_tag_open = use_signal(|| false);

    let mut trigger_update = use_signal(|| false);
    use_task_update(name, task.uuid.clone(), trigger_update);

    let ctx_tags = use_context::<TagsContext>();

    use_click_outside(
        "update-task-area".to_string(),
        move || is_open(),
        EventHandler::new(move |_| {
            if !is_add_tag_open() && !is_delete_open() {
                is_open.set(false)
            }
        }),
    );

    let tags_name = use_memo(move || ctx_tags.tags.iter().map(|tag| tag.name.clone()).collect());

    rsx! {
        Dialog { is_open,
            DialogContent { id: "update-task-area", class: "sm:max-w-[425px]",
                DialogHeader {
                    div {
                        class: "flex justify-between items-center",
                        DialogTitle { "Update {task.name}" }
                        Button {
                            onclick: move |_| {
                                is_delete_open.set(true);
                            },
                            class: "px-2 text-xs h-6",
                            "Delete"
                        }
                    }
                }
                Input {
                    width: "w-full",
                    placeholder: "Enter list name",
                    value: name,
                    onenter: EventHandler::new(move |_e: KeyboardEvent| {
                        trigger_update.set(true);
                        is_open.set(false);
                    }),
                }
                Label {
                    variant: "title",
                    class: "p-2 bg-primary",
                    width: "w-full",
                    "Tags"
                }
                div {
                    class: "flex justify-between gap-2",
                    SearchDropdown {
                        id: "search-tags-area",
                        options: tags_name,
                        value: tag_search,
                        class: "text-base",
                        SearchDropdownInput {
                            width: "w-full",
                            placeholder: " Search tags",
                        }
                        SearchDropdownContent {}
                    }
                    Button {
                        class: "px-2 justify-between gap-2 font-semibold text-base",
                        onclick: move |_| is_add_tag_open.set(true),
                        Icon { height: 14, width: 14, icon: FaPlus }
                    }
                }
                DialogFooter {
                    DialogClose {}
                    Button {
                        onclick: move |_| {
                            trigger_update.set(true);
                            is_open.set(false);
                        },
                        r#type: "submit",
                        variant: "outline",
                        class: "font-semibold px-2 text-sm",
                        "Save"
                    }
                }
            }
        }
        DeleteTask { is_open: is_delete_open, task: task.clone(), parent_open: is_open }
        AddTag { is_open: is_add_tag_open }
    }
}

#[component]
pub fn DeleteTask(task: Task, is_open: Signal<bool>, parent_open: Signal<bool>) -> Element {
    let mut trigger = use_signal(|| false);

    use_task_delete(task.uuid, trigger);

    use_click_outside(
        "delete-task-area".to_string(),
        move || is_open(),
        EventHandler::new(move |_| is_open.set(false)),
    );

    rsx! {
        Dialog { is_open,
            DialogContent { id: "delete-task-area", class: "sm:max-w-[425px]",
                DialogHeader {
                    DialogTitle { "Delete {task.name}" }
                    DialogDescription { "Are you sure ?" }
                }
                DialogFooter {
                    DialogClose {}
                    Button {
                        onclick: move |_| {
                            trigger.set(true);
                            is_open.set(false);
                            parent_open.set(false);
                        },
                        r#type: "submit",
                        variant: "outline",
                        class: "font-semibold px-2 text-sm",
                        "Delete"
                    }
                }
            }
        }
    }
}

#[component]
pub fn AddTag(is_open: Signal<bool>) -> Element {
    let route = use_route::<Route>();
    let board_uuid = match route {
        Route::Board { uuid } => uuid.clone(),
        _ => return rsx!("Invalid route"),
    };
    let re = Regex::new(r"^#[0-9a-fA-F]{6}$")?;
    let name = use_signal(|| "".to_string());
    let color = use_signal(|| "".to_string());
    let mut is_valid_hex = use_signal(|| false);
    let mut trigger = use_signal(|| false);

    use_tag_add(name, color, board_uuid, trigger);

    use_click_outside(
        "add-tag-area".to_string(),
        move || is_open(),
        EventHandler::new(move |_| is_open.set(false)),
    );

    use_effect(move || {
        if !re.is_match(&color().trim()) {
            is_valid_hex.set(false)
        } else {
            is_valid_hex.set(true)
        }
    });

    rsx! {
        Dialog { is_open,
            DialogContent { id: "add-tag-area", class: "sm:max-w-[425px]",
                DialogHeader {
                    DialogTitle { "Add Tag" }
                }
                Input {
                    width: "w-full",
                    placeholder: "Enter tag name",
                    value: name,
                    onenter: EventHandler::new(move |_e: KeyboardEvent| {
                        trigger.set(true);
                        is_open.set(false);
                    }),
                }
                Input {
                    width: "w-full",
                    placeholder: "Enter tag color",
                    value: color,
                }
                DialogFooter {
                    DialogClose {}
                    Button {
                        disabled: !is_valid_hex(),
                        onclick: move |_| {
                            trigger.set(true);
                            is_open.set(false);
                        },
                        r#type: "submit",
                        variant: "outline",
                        class: "font-semibold px-2 text-sm",
                        "Save"
                    }
                }
            }
        }
    }
}

// #[component]
// pub fn TaskSettings(is_open: Signal<bool>, tags: Vec<Tag>) -> Element {
//     let re = Regex::new(r"^#[0-9a-fA-F]{6}$")?;
//     let name = use_signal(|| "".to_string());
//     let desc = use_signal(|| "".to_string());
//     let mut search = use_signal(|| "".to_string());
//     let mut is_status_open = use_signal(|| false);
//     let mut status = use_signal(|| Status::Todo);
//     let mut is_priority_open = use_signal(|| false);
//     let mut priority = use_signal(|| Priority::Low);
//     let mut adding_tag = use_signal(|| false);
//     let new_tag_name = use_signal(|| "".to_string());
//     let new_tag_color = use_signal(|| "".to_string());
//     let mut is_valid_hex = use_signal(|| false);
//     let mut task_tags = use_signal(|| Vec::<Tag>::new());
//     let is_search_active = use_memo(move || !search().is_empty());

//     use_click_outside(
//         "status-task-area".to_string(),
//         move || is_status_open(),
//         EventHandler::new(move |_| is_status_open.set(false)),
//     );

//     use_click_outside(
//         "priority-task-area".to_string(),
//         move || is_priority_open(),
//         EventHandler::new(move |_| is_priority_open.set(false)),
//     );

//     use_click_outside(
//         "search-tags-area".to_string(),
//         move || is_search_active(),
//         EventHandler::new(move |_| search.set("".to_string())),
//     );

//     rsx! {
//         Dialog { is_open,
//             DialogContent { id: "settings-task-area", class: "sm:max-w-[425px]",
//                 DialogHeader {
//                     DialogTitle { "Task settings" }
//                 }
//                 div { class: "flex flex-col gap-4",
//                     Input {
//                         width: "w-full",
//                         placeholder: "Enter board name",
//                         value: name,
//                     }
//                     Textarea {
//                         width: "w-full",
//                         placeholder: "Enter board description",
//                         value: desc,
//                     }
//                     Dropdown {
//                         id: "status-task-area",
//                         is_open: is_status_open,
//                         class: "text-base",
//                         options: Signal::new(
//                             [
//                                 (
//                                     Status::Todo.to_string(),
//                                     Some(EventHandler::new(move |_| status.set(Status::Todo))),
//                                 ),
//                                 (
//                                     Status::InProgress.to_string(),
//                                     Some(EventHandler::new(move |_| status.set(Status::InProgress))),
//                                 ),
//                                 (
//                                     Status::Done.to_string(),
//                                     Some(EventHandler::new(move |_| status.set(Status::Done))),
//                                 ),
//                             ]
//                                 .to_vec(),
//                         ),
//                         DropdownTrigger { width: "w-full", name: status().to_string(),
//                             Icon { height: 14, width: 14, icon: FaChevronDown }
//                         }
//                         DropdownContent {}
//                     }
//                     Dropdown {
//                         id: "priority-task-area",
//                         is_open: is_priority_open,
//                         class: "text-base",
//                         options: Signal::new(
//                             [
//                                 (
//                                     Priority::Low.to_string(),
//                                     Some(EventHandler::new(move |_| priority.set(Priority::Low))),
//                                 ),
//                                 (
//                                     Priority::Medium.to_string(),
//                                     Some(EventHandler::new(move |_| priority.set(Priority::Medium))),
//                                 ),
//                                 (
//                                     Priority::High.to_string(),
//                                     Some(EventHandler::new(move |_| priority.set(Priority::High))),
//                                 ),
//                             ]
//                                 .to_vec(),
//                         ),
//                         DropdownTrigger { width: "w-full", name: priority().to_string(),
//                             Icon { height: 14, width: 14, icon: FaChevronDown }
//                         }
//                         DropdownContent {}
//                     }
//                     div { class: "flex items-center gap-2 h-full",
//                         if !adding_tag() {
//                             SearchDropdown {
//                                 id: "search-tags-area",
//                                 options: Signal::new(tags.iter().map(|tag| tag.name.clone()).collect()),
//                                 value: search,
//                                 class: "text-base",
//                                 SearchDropdownInput {
//                                     width: "w-full",
//                                     placeholder: " Search tags",
//                                 }
//                                 SearchDropdownContent {}
//                             }
//                             Button {
//                                 class: "px-1 h-7 w-7",
//                                 onclick: move |_| {
//                                     adding_tag.set(true);
//                                 },
//                                 Icon {
//                                     class: "text-primary",
//                                     height: 12,
//                                     icon: FaPlus,
//                                 }
//                             }
//                         } else {
//                             Input {
//                                 width: "w-full",
//                                 placeholder: "New tag name",
//                                 value: new_tag_name,
//                             }
//                             Input {
//                                 width: "w-full",
//                                 placeholder: "New tag color",
//                                 value: new_tag_color,
//                                 oninput: move |_| {
//                                     if !re.is_match(&new_tag_color()) {
//                                         is_valid_hex.set(false)
//                                     } else {
//                                         is_valid_hex.set(true)
//                                     }
//                                 },
//                             }
//                             Button {
//                                 class: "px-1 h-7 w-7",
//                                 onclick: move |_| {
//                                     adding_tag.set(false);
//                                     task_tags
//                                         .with_mut(|tag_list| {
//                                             tag_list
//                                                 .push(Tag {
//                                                     uuid: "WTF".to_string(),
//                                                     name: new_tag_name(),
//                                                     color: new_tag_color(),
//                                                 });
//                                         });
//                                 },
//                                 disabled: !is_valid_hex(),
//                                 Icon {
//                                     class: "text-primary",
//                                     height: 12,
//                                     icon: FaCheck,
//                                 }
//                             }
//                         }
//                     }
//                     if !tags.is_empty() {
//                         div {
//                             p { class: "font-semibold pb-2", "Tags" }
//                             div { class: "flex flex-col gap-2 max-h-32 overflow-y-auto w-full border-2 border-secondary p-2",
//                                 {tags.iter().map(|tag| rsx! {
//                                     div { class: "flex gap-2 justify-between items-center w-full px-1",
//                                         div { class: "flex gap-4 items-center w-fit",
//                                             Icon {
//                                                 style: format!("--tag-color: {};", tag.color),
//                                                 class: format!("text-[var(--tag-color)]"),
//                                                 height: 10,
//                                                 width: 10,
//                                                 icon: FaCircle,
//                                             }
//                                             "{tag.name}"
//                                         }
//                                         Button { class: "px-1 h-6 w-6",
//                                             // onclick: move |_| adding_task.set(false),
//                                             Icon { class: "text-primary", height: 12, icon: FaXmark }
//                                         }
//                                     }
//                                 })}
//                             }
//                         }
//                     }
//                 }
//                 DialogFooter {
//                     DialogClose {}
//                     Button {
//                         r#type: "submit",
//                         variant: "outline",
//                         class: "font-semibold px-2 text-sm",
//                         "Save"
//                     }
//                 }
//             }
//         }
//     }
// }
