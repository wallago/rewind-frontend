use dioxus::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::{
    Route,
    components::{fetch, layout::Body, table::*},
    views::lists::dialog::*,
};

#[component]
pub fn Lists(uuid: String) -> Element {
    let mut refetch_signal = use_signal(|| 0);
    let lists = fetch::lists::get_lists(uuid.clone(), refetch_signal);

    rsx! (
         Body {
            p {
                class: "text-sm font-semibold pb-6",
                "Board: {uuid}"
            }
            Table {
                TableCaption { "List of your lists." }
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
                            DialogAdd { uuid, refetch_signal, lists }
                        }
                    }
                }

                TableBody {
                    match lists() {
                        Some(Some(lists)) => {
                                if lists.is_empty() {
                                    rsx!(TableRow { TableCell {
                                        colspan: Some(4),
                                        "No lists found"
                                    } })
                                } else {
                                    let lists = lists.clone();
                                    let mut dragging_list_uuid = use_signal(|| None::<String>);
                                    rsx!(
                                        {lists.iter().map(|list| {
                                            let uuid = list.uuid.clone();
                                            let name = list.name.clone();
                                            let desc = list.description.clone().unwrap_or_default();

                                            let dragged_uuid_from = list.uuid.clone();
                                            let dragged_uuid_to = list.uuid.clone();
                                            rsx!(TableRow {
                                                class: "cursor-pointer hover:bg-surface-variant-light dark:hover:bg-surface-variant-dark",
                                                onclick: move |e: MouseEvent| {
                                                    e.stop_propagation();
                                                    navigator().push(Route::Tasks { uuid: uuid.clone() });
                                                },
                                                draggable: Some(true),
                                                ondragstart: Some(EventHandler::new(move |_: DragEvent| {
                                                    dragging_list_uuid.set(Some(dragged_uuid_from.clone()));
                                                })),
                                                ondrop: Some(EventHandler::new(move |_: DragEvent| {
                                                    let uuid_to = dragged_uuid_to.clone();
                                                    if let Some(uuid_from) = dragging_list_uuid() {
                                                        spawn_local(async move {
                                                            match fetch::lists::switch_lists(&uuid_from, &uuid_to).await {
                                                                Ok(_) => refetch_signal.set(refetch_signal() + 1),
                                                                Err(err) => tracing::error!("Update list failed with: {err}"),
                                                            }
                                                        })
                                                    }
                                                })),

                                                TableCell {
                                                    class: Some("font-medium".to_string()),
                                                    {list.uuid.clone()}
                                                }
                                                TableCell { {name} }
                                                TableCell { {desc} }
                                                TableCell {
                                                    class: Some("flex justify-end gap-2".to_string()),
                                                    DialogUpdate { refetch_signal, list: list.clone() }
                                                    DialogDelete { refetch_signal,  list: list.clone() }
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
