use dioxus::prelude::*;

#[component]
pub fn Lists(uuid: String) -> Element {
    rsx! { h2 { "Board: {uuid}" } }
}
