use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::fa_solid_icons::{FaCheck, FaChevronDown, FaCircle, FaPlus, FaXmark},
};
use regex::Regex;

use crate::{
    api::add_list,
    components::{
        Button, Dialog, DialogClose, DialogContent, DialogFooter, DialogHeader, DialogTitle,
        Dropdown, DropdownContent, DropdownTrigger, Input, SearchDropdown, SearchDropdownContent,
        SearchDropdownInput, Textarea,
    },
    hooks::use_click_outside,
    models::{NewList, Priority, Status, Tag},
};

#[component]
pub fn AddList(is_open: Signal<bool>, board_uuid: String) -> Element {
    let name = use_signal(|| "".to_string());

    let mut add = use_signal(|| false);
    use_future(move || {
        let uuid = board_uuid.clone();
        async move {
            if !add() {
                return ();
            } else {
                add.set(false);
            }
            match add_list(NewList {
                name: name(),
                position: None,
                board_uuid: uuid,
            })
            .await
            {
                Ok(_) => {}
                Err(err) => tracing::error!("{err}"),
            }
        }
    });
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
                }
                DialogFooter {
                    DialogClose {}
                    Button {
                        onclick: move |_| {
                            add.set(true);
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
pub fn TaskSettings(is_open: Signal<bool>, tags: Vec<Tag>) -> Element {
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
        Dialog { is_open,
            DialogContent { id: "settings-task-area", class: "sm:max-w-[425px]",
                DialogHeader {
                    DialogTitle { "Task settings" }
                }
                div { class: "flex flex-col gap-4",
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
                        options: Signal::new(
                            [
                                (
                                    Status::Todo.to_string(),
                                    Some(EventHandler::new(move |_| status.set(Status::Todo))),
                                ),
                                (
                                    Status::InProgress.to_string(),
                                    Some(EventHandler::new(move |_| status.set(Status::InProgress))),
                                ),
                                (
                                    Status::Done.to_string(),
                                    Some(EventHandler::new(move |_| status.set(Status::Done))),
                                ),
                            ]
                                .to_vec(),
                        ),
                        DropdownTrigger { width: "w-full", name: status().to_string(),
                            Icon { height: 14, width: 14, icon: FaChevronDown }
                        }
                        DropdownContent {}
                    }
                    Dropdown {
                        id: "priority-task-area",
                        is_open: is_priority_open,
                        class: "text-base",
                        options: Signal::new(
                            [
                                (
                                    Priority::Low.to_string(),
                                    Some(EventHandler::new(move |_| priority.set(Priority::Low))),
                                ),
                                (
                                    Priority::Medium.to_string(),
                                    Some(EventHandler::new(move |_| priority.set(Priority::Medium))),
                                ),
                                (
                                    Priority::High.to_string(),
                                    Some(EventHandler::new(move |_| priority.set(Priority::High))),
                                ),
                            ]
                                .to_vec(),
                        ),
                        DropdownTrigger { width: "w-full", name: priority().to_string(),
                            Icon { height: 14, width: 14, icon: FaChevronDown }
                        }
                        DropdownContent {}
                    }
                    div { class: "flex items-center gap-2 h-full",
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
                                },
                            }
                            Button {
                                class: "px-1 h-7 w-7",
                                onclick: move |_| {
                                    adding_tag.set(false);
                                    task_tags
                                        .with_mut(|tag_list| {
                                            tag_list
                                                .push(Tag {
                                                    uuid: "WTF".to_string(),
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
                            p { class: "font-semibold pb-2", "Tags" }
                            div { class: "flex flex-col gap-2 max-h-32 overflow-y-auto w-full border-2 border-secondary p-2",
                                {tags.iter().map(|tag| rsx! {
                                    div { class: "flex gap-2 justify-between items-center w-full px-1",
                                        div { class: "flex gap-4 items-center w-fit",
                                            Icon {
                                                style: format!("--tag-color: {};", tag.color),
                                                class: format!("text-[var(--tag-color)]"),
                                                height: 10,
                                                width: 10,
                                                icon: FaCircle,
                                            }
                                            "{tag.name}"
                                        }
                                        Button { class: "px-1 h-6 w-6",
                                            // onclick: move |_| adding_task.set(false),
                                            Icon { class: "text-primary", height: 12, icon: FaXmark }
                                        }
                                    }
                                })}
                            }
                        }
                    }
                }
                DialogFooter {
                    DialogClose {}
                    Button {
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
