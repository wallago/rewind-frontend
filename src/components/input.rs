use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct InputProps {
    name: String,
    value: Signal<String>,
}

#[component]
pub fn Input(mut props: InputProps) -> Element {
    rsx!(
        div {
            class: "flex items-center",
            p {
                class: "text-xs font-semi-bold pr-4",
                "{props.name}:"
            }
            input {
                class: "
                    text-xs font-medium
                    bg-surface-light dark:bg-surface-dark
                    border-2 border-border-light dark:border-border-dark 
                    p-1
                    focus:outline-none focus:ring-0
                ",
                value: "{props.value}",
                oninput: move |event| props.value.set(event.value())
            }
        }
    )
}
