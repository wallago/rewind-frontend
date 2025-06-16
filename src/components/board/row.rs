use dioxus::prelude::*;

use crate::components::{
    board::model::Board,
    icons::{More, arrow},
    list::{
        model::List,
        row::{ListRow, ListRowSkeleton, ListRowTitle},
    },
};

static API: &str = "http://0.0.0.0:8081/api";

#[component]
pub fn BoardRow(board: Board) -> Element {
    let mut is_open = use_signal(|| false);
    let board_clone = board.clone();
    let lists = use_resource(move || {
        let uuid = board_clone.uuid.clone();
        async move {
            let response = reqwest::get(format!("{API}/boards/{}/lists", uuid))
                .await
                .ok()?;
            response.json::<Vec<List>>().await.ok()
        }
    });

    rsx!(
        li {
            key: "{board.uuid}",
            class: "text-sm border-t-2 border-border-light dark:border-border-dark pb-2 pt-2",
            div {
                class: "grid grid-cols-4 gap-4",
                p { "{board.name}" }
                p { "{board.description.as_deref().unwrap_or(\"-\")}" }
                match lists.read().as_ref() {
                    Some(Some(board_list_items)) => rsx!(
                        p {
                            class: "flex justify-between",
                            "{board_list_items.len()}"
                            if !board_list_items.is_empty() {
                                p {
                                    class: "flex font-bold",
                                    "lists"
                                    button {
                                        class: "ml-1 transition-transform duration-200",
                                        class: if *is_open.read() { "rotate-180" } else { "rotate-0" },
                                        onclick: move |_| is_open.toggle(),
                                        arrow::Bottom {}
                                    }
                                }
                            }
                        }
                        button {
                            class: "justify-self-end mr-4 hover:bg-border-light dark:hover:bg-border-dark",
                            More {}
                        }
                        if *is_open.read() {
                            ul {
                                class: "py-2 px-4 mb-2 border-2 border-border-light dark:border-border-dark bg-surface-light dark:bg-surface-dark w-full col-span-4 grid grid-cols-1 gap-2",
                                ListRowTitle {}
                                Fragment {
                                    for list in board_list_items {
                                        ListRow { list: list.clone()  }
                                    }
                                }
                            }
                        }
                    ),
                    Some(None) => rsx!(p {
                        class: "text-error-light dark:text-error-dark",
                        "failed to fetch"
                    }),
                    None => rsx!(p {
                        ul {
                            class: "space-y-2 animate-pulse",
                            for i in 0..3 {
                                BoardRowSkeleton { i }
                            }
                        }
                    })
                }
            }
        }
    )
}

#[component]
pub fn BoardRowTitle() -> Element {
    rsx!(
        div {
            class: "grid grid-cols-4 gap-4 font-bold",
            p { "name" },
            p { "description" },
            p { "lists" },
            p { "" },
        }
    )
}

#[component]
pub fn BoardRowSkeleton(i: i32) -> Element {
    rsx!(
        li {
            key: "{i}",
            class: "p-2 rounded",
            div {
                class: "grid grid-cols-3 gap-4",
                div { class: "h-4 bg-muted-light dark:bg-muted-dark rounded" }
                div { class: "h-4 bg-muted-light dark:bg-muted-dark rounded" }
                div { class: "h-4 bg-muted-light dark:bg-muted-dark rounded" }
            }
        }
    )
}
