use crate::hooks::{use_board_drag_switch, use_board_update, use_click_outside};
use crate::views::home::delete_board_modal::DeleteBoard;
use crate::{
    Route,
    components::{Accordion, AccordionContent, AccordionTrigger, Button, Card, Input},
    models::Board,
};
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_solid_icons::{FaCheck, FaXmark};

#[derive(Props, Clone, PartialEq)]
pub struct BoardCardProps {
    board: Board,
    dragging_from: Signal<Option<String>>,
}

#[component]
pub fn BoardCard(mut props: BoardCardProps) -> Element {
    let name = use_signal(|| props.board.name.clone());
    let mut is_update_open = use_signal(|| false);
    let mut is_delete_open = use_signal(|| false);
    let mut is_desc_open = use_signal(|| false);
    let mut dragging_to = use_signal(|| None::<String>);

    let uuid_from = props.board.uuid.clone();
    let uuid_to = props.board.uuid.clone();

    use_click_outside(
        "update-board-area".to_string(),
        move || is_update_open(),
        EventHandler::new(move |_| is_update_open.set(false)),
    );

    let mut trigger_update = use_signal(|| false);
    use_board_update(name, props.board.uuid.clone(), trigger_update);

    let mut trigger_switch = use_signal(|| false);
    use_board_drag_switch(props.dragging_from, dragging_to, trigger_switch);

    rsx! {
        div {
            id: props.board.uuid,
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
            Card {
                class: "h-fit flex flex-col gap-2 mx-auto pt-2 px-2",
                width: "w-72",
                div { class: "flex flex-col justify-center text-sm font-medium  w-full",
                    div { class: "flex gap-1 justify-between h-full items-center pb-1",
                        if !is_update_open() {
                            Button {
                                variant: "ghost",
                                class: "text-base",
                                width: "w-fit",
                                onclick: move |_| is_update_open.set(true),
                                div { class: "break-all", "{props.board.name}" }
                            }
                            Button {
                                class: "h-fit p-0.5",
                                onclick: move |_| is_delete_open.set(true),
                                Icon { height: 14, width: 14, icon: FaXmark }
                            }
                        } else {
                            Input {
                                id: "update-board-area",
                                width: "w-full",
                                placeholder: "Enter board name",
                                value: name,
                                onenter: EventHandler::new(move |_e: KeyboardEvent| {
                                    trigger_update.set(true);
                                    is_update_open.set(false);
                                }),
                            }
                            div { class: "flex gap-2",
                                Button {
                                    class: "h-fit p-0.5",
                                    onclick: move |_| {
                                        trigger_update.set(true);
                                        is_update_open.set(false)
                                    },
                                    Icon {
                                        height: 14,
                                        width: 14,
                                        icon: FaCheck,
                                    }
                                }
                                Button {
                                    class: "h-fit p-0.5",
                                    onclick: move |_| is_update_open.set(false),
                                    Icon {
                                        height: 14,
                                        width: 14,
                                        icon: FaXmark,
                                    }
                                }
                            }
                        }
                    }
                    Accordion { class: "py-1", is_open: is_desc_open.clone(),
                        AccordionTrigger { class: "text-sm", label: "description" }
                        AccordionContent { class: "text-sm", "la tu coco faudrais une desc sdlfjsdlkfjsd" }
                    }
                }
                div { class: "flex justify-end",
                    Button {
                        onclick: {
                            let uuid = props.board.uuid.clone();
                            move |_| {
                                navigator().push(Route::Board { uuid: uuid.clone() });
                            }
                        },
                        class: "px-2 text-sm",
                        "Details"
                    }
                }
                DeleteBoard { board: props.board.clone(), is_open: is_delete_open }
            }
        }
    }
}
