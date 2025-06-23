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
                        onclick: move |event: MouseEvent| {
                            event.stop_propagation();
                            if let Some(handler) = &props.onclick {
                                handler.call(event);
                            }
                            props.is_open.set(false)
                        },
                        class:"w-24",
                        "send"
                    }
                    Button {
                        onclick: move |event: MouseEvent| {
                            event.stop_propagation();
                            props.is_open.set(false)
                        },
                        class:"w-24",
                        "close"
                    }
                }
            }
        }
    }
}
