use super::{Button, Input, fetch};
use dioxus::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[component]
pub fn Board(is_open: Signal<bool>, refetch_signal: Signal<u32>) -> Element {
    if !(is_open)() {
        return rsx!();
    }

    let name = use_signal(|| "bob".to_string());

    rsx! {
        div {
            class: "
                fixed top-0 left-0 w-full h-full 
                bg-black/80 dark:bg-blakc/80 
                flex items-center justify-center z-50
            ",
            div {
                class: "
                    p-4 
                    w-80
                    bg-muted-light dark:bg-muted-dark
                    grid grid-cols-1 gap-4
                ",
                "This is a modal!"
                Input {
                    name: "Board name",
                    value: name,
                }
                div {
                    class:"
                        h-1 w-full 
                        bg-border-light dark:bg-border-dark
                    " }
                div {
                    class:"flex justify-between",
                    Button {
                        onclick: Some(EventHandler::new(move |_| {
                            // Spawn async task to call add_board
                            spawn_local(async move {
                                let board = fetch::NewBoard {
                                    name: name(),
                                    description: None,
                                };

                                if let Some(board) = fetch::add_board(board).await {
                                    tracing::info!("New board has been added: {:#?}", board);
                                    refetch_signal.set( refetch_signal() + 1);
                                }
                                is_open.set(false)
                            });
                        })),
                        class:"w-24",
                        "send"
                    }
                    Button {
                        onclick: move |_| is_open.set(false),
                        class:"w-24",
                        "close"
                    }
                }

            }
        }
    }
}
