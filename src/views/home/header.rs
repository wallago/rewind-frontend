use dioxus::prelude::*;

use crate::components::Label;

#[component]
pub fn Header() -> Element {
    rsx! {
        Label { class: "px-2 py-1.5", "Boards" }
    }
}
