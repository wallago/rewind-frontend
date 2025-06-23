use dioxus::prelude::*;

#[component]
pub fn Tasks(uuid: String) -> Element {
    rsx! { h2 { "List: {uuid}" } }
}
