use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
        class: "mx-4 py-2 text-sm",
        p { "© 2024 Your Task Manager" }
    }
    }
}
