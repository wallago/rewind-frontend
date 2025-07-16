use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_solid_icons::{FaCheck, FaXmark};

use crate::api::{switch_boards, update_board};
use crate::models::UpdateBoard;
use crate::{
    Route,
    components::{Button, Card, HoverCard, HoverCardContent, Input, Label},
    context::BoardsContext,
    hooks::use_click_outside,
    models::Board,
};

mod modals;

#[component]
pub fn Home() -> Element {
    let ctx_boards = use_context::<BoardsContext>();
    let dragging_index = use_signal(|| None::<String>);
    let boards: Vec<Element> = (ctx_boards.boards)()
        .iter()
        .map(|board| {
            rsx!(BoardCard {
                board: board.clone(),
                dragging_from: dragging_index
            })
        })
        .collect();

    rsx! {
        div { class: "p-4 h-full bg-primary border-2 border-secondary flex flex-col gap-4",
            Header {}
            div { class: "grid gap-4 grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5 overflow-y-auto",
                {boards.iter()}
            }
        }
    }
}

#[component]
fn Header() -> Element {
    rsx! {
        Label { class: "px-2 py-1.5", "Boards" }
    }
}

#[component]
fn BoardCard(board: Board, dragging_from: Signal<Option<String>>) -> Element {
    let name = use_signal(|| board.name.clone());
    let board_copy = board.clone();
    let board_uuid = board.uuid.clone();
    let board_uuid_from = board.uuid.clone();
    let board_uuid_to = board.uuid.clone();
    let id = board.uuid.clone();
    let mut is_delete_open = use_signal(|| false);
    let mut is_update = use_signal(|| false);
    let mut dragging_to = use_signal(|| None::<String>);

    use_click_outside(
        "delete-board-area".to_string(),
        move || is_delete_open(),
        EventHandler::new(move |_| is_delete_open.set(false)),
    );

    use_click_outside(
        "update-board-area".to_string(),
        move || is_update(),
        EventHandler::new(move |_| is_update.set(false)),
    );

    let mut ctx_boards = use_context::<BoardsContext>();
    let mut trigger_update = use_signal(|| false);
    let mut in_progress_update = use_signal(|| false);
    let mut trigger_switch = use_signal(|| false);
    let mut in_progress_switch = use_signal(|| false);

    let _ = use_resource(move || {
        let board_uuid = board_uuid.clone();
        async move {
            if trigger_update() {
                in_progress_update.set(true);
                match update_board(
                    &board_uuid,
                    UpdateBoard {
                        name: Some(name()),
                        position: None,
                    },
                )
                .await
                {
                    Ok(_) => ctx_boards.refresh.set(()),
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
            match switch_boards(&uuid_from, &uuid_to).await {
                Ok(_) => ctx_boards.refresh.set(()),
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

    rsx! {
        div {
            id,
            draggable: true,
            ondragstart: move |_| {
                dragging_from.set(Some(board_uuid_from.clone()));
            },
            ondragover: move |evt| {
                evt.prevent_default();
            },
            ondrop: move |_| {
                if dragging_from().is_some() {
                    dragging_to.set(Some(board_uuid_to.clone()));
                    trigger_switch.set(true);
                }
            },
            Card {
                class: "h-fit p-2 flex flex-col gap-4 mx-auto drag:scale-60",
                width: "w-72",
                div { class: "flex flex-col justify-center text-sm font-medium gap-2 w-full",
                    div { class: "flex gap-3 justify-between h-full items-center pb-1",
                        if !is_update() {
                            Button {
                                variant: "ghost",
                                class: "text-base",
                                width: "w-fit",
                                onclick: move |_| is_update.set(true),
                                div { class: "break-all", "{board.name}" }
                            }
                            Button {
                                class: "px-1 h-fit py-1 my-1",
                                onclick: move |_| is_delete_open.set(true),
                                Icon { height: 16, width: 16, icon: FaXmark }
                            }
                        } else {
                            Input {
                                id: "update-board-area",
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
                        Label {
                            variant: "outline",
                            class: "p-2 text-sm",
                            width: "w-full",
                            div { class: "truncate", "UUID: {board.uuid}" }
                        }
                        HoverCardContent { {board.uuid.clone()} }
                    }
                }
                div { class: "flex justify-end",
                    Button {
                        onclick: move |_| {
                            navigator()
                                .push(Route::Board {
                                    uuid: board.uuid.clone(),
                                });
                        },
                        class: "px-2 text-base",
                        "Details"
                    }
                }
                modals::DeleteBoard { board: board_copy, is_open: is_delete_open }
            }
        }
    }
}
