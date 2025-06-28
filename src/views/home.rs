use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "h-full bg-primary border-2 border-secondary",
            div {
                class: "px-6 text-[12px]",
                "@ Copyrihgt-2025 - 2046"
            }
        }
    }
}
