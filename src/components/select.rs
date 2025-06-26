use dioxus::prelude::*;

use super::{Button, icons::ArrowBottom};

#[derive(PartialEq, Clone, Props)]
pub struct SelectProps {
    pub selected: Signal<Option<String>>,
    pub options: Vec<(String, String)>,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub name: Option<String>,
    #[props(optional)]
    pub id: Option<String>,
}

#[component]
pub fn Select(mut props: SelectProps) -> Element {
    let mut is_open = use_signal(|| false);
    rsx!(
        div { class: " w-full font-semibold",
            Button {
                variant: "outline",
                class: "w-full font-semibold",
                onclick: move |_| is_open.set(!is_open()),
                div {
                    class: "flex justify-between",
                    {(props.selected)().unwrap_or("--Please choose an option--".to_string())}
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
                    for (value, label) in props.options {
                        div {
                            onclick: move |_| {
                                props.selected.set(Some(value.clone()));
                                is_open.set(false);
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
