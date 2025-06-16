use super::ThemeToggle;
use crate::{DarkMode, Route};
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    let mut dark_mode = use_context::<Signal<DarkMode>>();
    rsx! {
        div {
            class: "flex justify-between pt-6 px-4",
            id: "navbar",
            div {
                class: "font-semibold text-sm hover:bg-muted-light dark:hover:bg-muted-dark p-1",
                Link {
                    to: Route::Home {},
                    "Home"
                }
            }
            ThemeToggle {}
        }
    }
}
