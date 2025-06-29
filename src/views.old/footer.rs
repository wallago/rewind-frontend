use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        div {
            class: "h-12 py-2 bg-primary flex items-center border-t-2 border-secondary",
            div {
                class: "px-6 text-[12px]",
                "@ Copyrihgt-2025 - 2046"
            }
        }
    }
}
