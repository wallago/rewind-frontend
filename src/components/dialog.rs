use super::{Button, Input, InputProps};
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct DialogProps {
    is_open: Signal<bool>,
    title: String,
    #[props(optional)]
    inputs: Option<Vec<InputProps>>,
    #[props(optional)]
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
}

#[component]
pub fn Dialog(mut props: DialogProps) -> Element {
    if !(props.is_open)() {
        return rsx!();
    }

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
                p {
                    class: "font-semibold truncate",
                    {props.title}
                }
                if let Some(inputs) = props.inputs {
                    for input in inputs {
                        Input {
                            name: input.name,
                            value: input.value
                        }
                    }
                }
                {props.children}
                div {
                    class:"flex justify-between",
                    Button {
                        onclick: move |event| {
                            if let Some(handler) = &props.onclick {
                                handler.call(event);
                            }
                            props.is_open.set(false)
                        },
                        class:"w-24",
                        "send"
                    }
                    Button {
                        onclick: move |_| props.is_open.set(false),
                        class:"w-24",
                        "close"
                    }
                }
            }
        }
    }
}

// #[component]
// pub fn Board(is_open: Signal<bool>, refetch_signal: Signal<u32>) -> Element {
//     if !(is_open)() {
//         return rsx!();
//     }

//     let name = use_signal(|| "X".to_string());
//     let desc = use_signal(|| "".to_string());

//     rsx! {
//         div {
//             class: "
//                 fixed top-0 left-0 w-full h-full
//                 bg-black/80 dark:bg-blakc/80
//                 flex items-center justify-center z-50
//             ",
//             div {
//                 class: "
//                     p-4
//                     w-80
//                     bg-muted-light dark:bg-muted-dark
//                     grid grid-cols-1 gap-4
//                 ",
//                 "This is a modal!"
//                 Input {
//                     name: "Board name",
//                     value: name,
//                 }
//                 Input {
//                     name: "Board desc",
//                     value: desc,
//                 }
//                 div {
//                     class:"
//                         h-1 w-full
//                         bg-border-light dark:bg-border-dark
//                     " }
//                 div {
//                     class:"flex justify-between",
//                     Button {
//                         onclick: Some(EventHandler::new(move |_| {
//                             spawn_local(async move {
//                                 let board = fetch::NewBoard {
//                                     name: name(),
//                                     description: Some(desc()),
//                                 };

//                                 if let Some(board) = fetch::add_board(board).await {
//                                     tracing::info!("New board has been added: {:#?}", board);
//                                     refetch_signal.set( refetch_signal() + 1);
//                                 }
//                                 is_open.set(false)
//                             });
//                         })),
//                         class:"w-24",
//                         "send"
//                     }
//                     Button {
//                         onclick: move |_| is_open.set(false),
//                         class:"w-24",
//                         "close"
//                     }
//                 }

//             }
//         }
//     }
// }
