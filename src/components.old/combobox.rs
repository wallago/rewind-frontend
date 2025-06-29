use dioxus::prelude::*;

use super::{
    Button, Input,
    icons::{ArrowBottom, Search},
};

#[derive(PartialEq, Clone, Props)]
pub struct ComboboxProps {
    pub title: String,
    pub options: Vec<(String, String)>,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub name: Option<String>,
    #[props(optional)]
    pub id: Option<String>,
}

#[component]
pub fn Combobox(mut props: ComboboxProps) -> Element {
    let mut is_open = use_signal(|| false);
    let search = use_signal(|| "".to_string());
    rsx!(
        div { class: " w-full font-semibold",
            Button {
                class: "w-full font-semibold",
                onclick: move |_| is_open.set(!is_open()),
                div {
                    class: "w-full flex justify-between",
                    {props.title}
                    ArrowBottom { size: "20px "}
                }
            }
            if is_open() {
                div {
                    class: "
                        mt-0.5 w-full
                        border-2 border-border-light dark:border-border-dark
                        bg-surface-light dark:bg-surface-dark
                    ",
                    // div {
                    //     onclick: move |_| {
                    //         // props.selected.set(Some(value.clone()));
                    //         // is_open.set(false);
                    //     },
                    //     class: "
                    //         w-full p-1
                    //         hover:bg-surface-variant-light dark:hover:bg-surface-variant-dark
                    //         cursor-pointer
                    //     ",
                    //     "{label}"
                    // }
                    div {
                        class: "
                            flex items-center px-1
                            bg-surface-variant-light dark:bg-surface-variant-dark 
                            fill-border-light dark:fill-muted-dark 
                            border-b-2 border-border-light dark:border-border-dark",
                        Search { size: "18px" }
                        Input {
                            variant: "variant",
                            class: "w-full border-0",
                            name: "search",
                            value: search
                        }
                    }
                    for (value, label) in props.options {
                        div {
                            onclick: move |_| {
                                // props.selected.set(Some(value.clone()));
                                // is_open.set(false);
                            },
                            class: "
                                w-full p-1
                                hover:bg-surface-variant-light dark:hover:bg-surface-variant-dark
                                cursor-pointer
                            ",
                            "{label}"
                        }
                    }
                }
            }
        }
    )
}
