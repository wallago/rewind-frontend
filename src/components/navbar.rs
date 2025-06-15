use super::ThemeToggle;
use crate::{DarkMode, Route};
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    let mut dark_mode = use_context::<Signal<DarkMode>>();
    rsx! {
        div {
            class: "flex justify-between pt-2 px-4",
            id: "navbar",
            div {
                class: "font-semibold text-sm hover:bg-amber-100 dark:hover:bg-slate-800 p-1 rounded",
                Link {
                    to: Route::Home {},
                    "Home"
                }
            }
            ThemeToggle {}
        }
        Outlet::<Route> {}
    }
}
