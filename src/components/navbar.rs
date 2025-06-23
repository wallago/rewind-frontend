use super::ThemeToggle;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        div {
            class: "sticky top-0",
            div {
                class:"flex justify-between pt-6 px-4",
                id: "navbar",
                div {
                    class: "font-semibold text-sm hover:bg-muted-light dark:hover:bg-muted-dark p-1",
                    Link {
                        to: Route::Boards {},
                        "Boards"
                    }
                }
                ThemeToggle {}
            }
        }
    }
}
