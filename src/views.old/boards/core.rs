use dioxus::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::{
    Route,
    components::{fetch, layout::Body, table::*},
    views::boards::dialog::*,
};

#[component]
pub fn Boards() -> Element {
    let mut refetch_signal = use_signal(|| 0);
    let boards = fetch::boards::get_boards(refetch_signal);

    rsx! (
         Body {
            p {
                class: "text-xl font-bold pb-6",
                "Welcom to your task manager!"
            }
            Table {
                TableCaption { "List of your boards." }
                TableHeader {
                    TableRow {
                        TableHead {
                            class: Some("w-96".to_string()),
                            "UUID"
                        }
                        TableHead { "Name" }
                        TableHead { "Description" }
                        TableHead {
                            class: Some("text-right".to_string()),
                            DialogAdd { refetch_signal, boards }
                        }
                    }
                }

                TableBody {
                    match boards() {
                        Some(Some(boards)) => {
                                if boards.is_empty() {
                                    rsx!(TableRow { TableCell {
                                        colspan: Some(4),
                                        "No boards found"
                                    } })
                                } else {
                                    let boards = boards.clone();
                                    let mut dragging_board_uuid = use_signal(|| None::<String>);
                                    rsx!(
                                        {boards.iter().map(|board| {
                                            let uuid = board.uuid.clone();
                                            let name = board.name.clone();
                                            let desc = board.description.clone().unwrap_or_default();

                                            let dragged_uuid_from = board.uuid.clone();
                                            let dragged_uuid_to = board.uuid.clone();
                                            rsx!(TableRow {
                                                class: "cursor-pointer hover:bg-surface-variant-light dark:hover:bg-surface-variant-dark",
                                                onclick: move |e: MouseEvent| {
                                                    e.stop_propagation();
                                                    navigator().push(Route::Lists { uuid: uuid.clone() });
                                                },
                                                draggable: Some(true),
                                                ondragstart: Some(EventHandler::new(move |_: DragEvent| {
                                                    dragging_board_uuid.set(Some(dragged_uuid_from.clone()));
                                                })),
                                                ondrop: Some(EventHandler::new(move |_: DragEvent| {
                                                    let uuid_to = dragged_uuid_to.clone();
                                                    if let Some(uuid_from) = dragging_board_uuid() {
                                                        spawn_local(async move {
                                                            match fetch::boards::switch_boards(&uuid_from, &uuid_to).await {
                                                                Ok(_) => refetch_signal.set(refetch_signal() + 1),
                                                                Err(err) => tracing::error!("Update board failed with: {err}"),
                                                            }
                                                        })
                                                    }
                                                })),
                                                TableCell {
                                                    class: Some("font-medium".to_string()),
                                                    {board.uuid.clone()}
                                                }
                                                TableCell { {name} }
                                                TableCell { {desc} }
                                                TableCell {
                                                    class: Some("flex justify-end gap-2".to_string()),
                                                    DialogUpdate { refetch_signal, board: board.clone() }
                                                    DialogDelete { refetch_signal,  board: board.clone() }
                                                }
                                            })
                                        })}
                                )
                            }
                        },
                        _ => rsx!(TableRow { TableCell {
                            colspan: Some(4),
                            "Loading..."
                        } }),
                    }
                }
            }
        }
    )
}
