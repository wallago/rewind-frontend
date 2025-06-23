use dioxus::prelude::*;

#[derive(Clone)]
pub struct ModalState {
    pub is_open: Signal<bool>,
}

#[component]
pub fn Modal() -> Element {
    rsx! {
        div {
            // class: "fixed top-0 left-0 w-full h-full bg-gray-800/50 flex items-center justify-center z-50",
            class: "fixed top-0 left-0 w-full h-full bg-gray-800/50 flex items-center justify-center z-50",
            div {
                class: "p-8 bg-white dark:bg-gray-900 rounded-lg w-96 h-64 border-2 border-border-light dark:border-border-dark",
                "This is a modal!"

            }
        }
    }
}
