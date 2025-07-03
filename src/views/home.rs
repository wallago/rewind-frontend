use dioxus::prelude::*;

use crate::{
    Route,
    api::get_boards,
    components::{Button, Card, HoverCard, HoverCardContent, Label},
    models::Board,
};

#[component]
pub fn Home() -> Element {
    let mut boards = use_signal(|| None::<Vec<Board>>);
    use_future(move || async move {
        match get_boards().await {
            Ok(res) => boards.set(Some(res)),
            Err(err) => tracing::error!("{err}"),
        }
    });

    rsx! {
        div {
            class: "p-4 h-full bg-primary border-2 border-secondary flex flex-col gap-4",
            Header {  }
            div {
                class: "grid gap-4 grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5",
                if let Some(boards) = boards() {
                    {boards.iter().map(|board| {
                       rsx!(
                            BoardCard { board: board.clone() }
                        )
                    })}
                }
            }
        }
    }
}

#[component]
fn Header() -> Element {
    rsx! {
        Label {
            class: "px-2 py-1.5",
            "Boards"
        }
    }
}

#[component]
fn BoardCard(board: Board) -> Element {
    rsx! {
        Card {
            class: "h-fit p-2 flex flex-col gap-4",
            width: "w-72",
            div {
                class: "flex flex-col justify-center text-sm font-medium gap-2 w-full",
                Label {
                    variant: "title_1",
                    class: "px-2 pb-2 text-base",
                    width: "w-full",
                    div {
                        class: "break-all",
                        "{board.name}"
                    }
                }
                HoverCard {
                    Label {
                        variant: "outline",
                        class: "p-2 text-sm",
                        width: "w-full",
                        div {
                            class: "truncate",
                            "UUID: {board.uuid}"
                        }
                    }
                    HoverCardContent {
                        {board.uuid.clone()}
                    }
                }
            }
            div {
                class: "flex justify-end",
                Button {
                    onclick: move |_| {
                        navigator().push(Route::Board { uuid: board.uuid.clone() });
                    },
                    class: "px-2 text-base",
                    "Details"
                }
            }
        }
    }
}
