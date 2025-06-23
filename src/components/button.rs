use dioxus::prelude::*;

use super::modal;

#[derive(PartialEq, Clone, Props)]
pub struct ClickableProps {
    children: Element,
}

#[component]
pub fn Button(props: ClickableProps) -> Element {
    let mut modal_state = use_context::<modal::ModalState>();

    // let mut dark_mode = use_context::<Signal<DarkMode>>();
    rsx!(button {
        onclick: move |_| modal_state.is_open.toggle(),
        class: "p-1 border-2 border-border-light dark:border-border-dark  hover:bg-border-light dark:hover:bg-border-dark",
        {props.children}
    })
}
