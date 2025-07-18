use crate::{
    components::{Button, Card, HoverCard, HoverCardContent, Input, Label},
    hooks::{use_click_outside, use_list_drag_switch, use_list_update},
    models::List,
    views::board::{modals::DeleteList, tasks_card::TasksCard},
};
use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::fa_solid_icons::{FaCheck, FaXmark},
};

#[derive(Props, Clone, PartialEq)]
pub struct ListCardProps {
    list: List,
    dragging_from: Signal<Option<String>>,
}

#[component]
pub fn ListCard(mut props: ListCardProps) -> Element {
    let mut is_delete_open = use_signal(|| false);
    let mut is_update_open = use_signal(|| false);
    let mut dragging_to = use_signal(|| None::<String>);
    let name = use_signal(|| props.list.name.clone());

    let uuid_from = props.list.uuid.clone();
    let uuid_to = props.list.uuid.clone();

    use_click_outside(
        "update-list-area".to_string(),
        move || is_update_open(),
        EventHandler::new(move |_| is_update_open.set(false)),
    );

    let mut trigger_update = use_signal(|| false);
    use_list_update(name, props.list.uuid.clone(), trigger_update);

    let mut trigger_switch = use_signal(|| false);
    use_list_drag_switch(props.dragging_from, dragging_to, trigger_switch);

    rsx! {
        div {
            id: props.list.uuid,
            draggable: true,
            ondragstart: move |_| {
                props.dragging_from.set(Some(uuid_from.clone()));
            },
            ondragover: move |evt| {
                evt.prevent_default();
            },
            ondrop: move |_| {
                if (props.dragging_from)().is_some() {
                    dragging_to.set(Some(uuid_to.clone()));
                    trigger_switch.set(true);
                }
            },
            Card { class: "h-fit flex flex-col gap-2 mx-auto", width: "w-96",
                div { class: "flex flex-col justify-center text-sm font-medium gap-2 w-full",
                    div { class: "flex gap-3 justify-between h-full items-center pb-1",
                        if !is_update_open() {
                            Button {
                                variant: "ghost",
                                class: "text-base",
                                width: "w-fit",
                                onclick: move |_| is_update_open.set(true),
                                div { class: "break-all", "{props.list.name}" }
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
                                    is_update_open.set(false);
                                }),
                            }
                            Button {
                                class: "px-1 h-fit py-1 my-1",
                                onclick: move |_| {
                                    trigger_update.set(true);
                                    is_update_open.set(false)
                                },
                                Icon { height: 16, width: 16, icon: FaCheck }
                            }
                            Button {
                                class: "px-1 h-fit py-1 my-1",
                                onclick: move |_| is_update_open.set(false),
                                Icon { height: 16, width: 16, icon: FaXmark }
                            }
                        }
                    }
                    HoverCard {
                        Label {
                            variant: "outline",
                            class: "p-2",
                            width: "w-full",
                            div { class: "truncate", "UUID: {props.list.uuid}" }
                        }
                        HoverCardContent { {props.list.uuid.clone()} }
                    }
                }
                TasksCard { uuid: props.list.uuid.clone() }
                        // TasksCard { tasks: tasks(), is_settings_open: is_task_settings_open }
            // div { class: "w-full flex justify-end px-2 pb-2",
            //     if adding_task() {
            //         div { class: "flex w-full gap-4 items-center",
            //             Input {
            //                 class: "flex-1 text-base bg-primary-2",
            //                 value: name,
            //                 onenter: EventHandler::new(move |_e: KeyboardEvent| {
            //                     trigger_add_task.set(true);
            //                     adding_task.set(false);
            //                 }),

            //             }
            //             Button {
            //                 class: "px-1 h-full",
            //                 onclick: move |_| {
            //                     trigger_add_task.set(true);
            //                     adding_task.set(false);
            //                 },
            //                 Icon {
            //                     class: "text-primary",
            //                     height: 12,
            //                     icon: FaCheck,
            //                 }
            //             }
            //             Button {
            //                 class: "px-1 h-full",
            //                 onclick: move |_| adding_task.set(false),
            //                 Icon {
            //                     class: "text-primary",
            //                     height: 12,
            //                     icon: FaXmark,
            //                 }
            //             }
            //         }
            //     } else {
            //         Button {
            //             onclick: move |_| adding_task.set(true),
            //             class: "text-base justify-between pl-2 pr-1",
            //             width: "w-20",
            //             "Task"
            //             Icon {
            //                 class: "text-primary",
            //                 height: 12,
            //                 icon: FaPlus,
            //             }
            //         }
            //     }
            // }
            }
            DeleteList { list: props.list.clone(), is_open: is_delete_open }
        }
    }
}
