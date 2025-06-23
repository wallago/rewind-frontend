use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            class: "fixed bottom-0 mx-4 py-2 text-sm",
            p { "© 2024 Your Task Manager" }
        }
    }
}
