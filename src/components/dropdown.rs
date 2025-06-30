use dioxus::prelude::*;

use crate::components::Button;

#[derive(PartialEq, Clone, Props)]
pub struct SelectProps {
    pub is_open: Signal<bool>,
    pub options: Vec<(String, Option<EventHandler>)>,
    #[props(optional)]
    pub name: Option<String>,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub id: Option<String>,
    #[props(optional)]
    pub width: Option<String>,
    pub children: Element,
}

#[component]
pub fn Dropdown(mut props: SelectProps) -> Element {
    let base_class = "relative w-full";
    rsx!(
        div { class: format!("{} {}", base_class, props.class.clone().unwrap_or_default()),
        id: props.id,
        Button {
            onclick: move |_| {
                props.is_open.toggle();
            },
            div {
                class: format!("px-2 flex gap-2 justify-between items-center {}", props.width.unwrap_or("w-fit".to_string())),
                span {
                    class: "truncate",
                    {props.name.clone().unwrap_or_else(|| String::from("Select an option"))}
                }
                {props.children}

            }
        }
        if (props.is_open)() {
            div {
                class: "
                    absolute z-50 mt-1 w-full max-h-60 overflow-y-auto
                    border-2 border-secondary shadow-lg
                    bg-primary text-secondary
                    ",
                    for (label, handler) in &props.options {
                        div {
                            key: "{label}",
                            onclick: {
                                let handler = handler.clone();
                                move |_| {
                                    if let Some(cb) = handler {
                                        cb.call(());
                                    }
                                    props.is_open.set(false);
                                }
                            },
                            class: "px-2 py-1 hover:bg-primary cursor-pointer hover:bg-primary-2 hover:text-secondary-2",
                            "{label}"
                        }
                    }
            }
        }
        }
    )
}
