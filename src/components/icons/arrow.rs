use dioxus::prelude::*;

#[component]
pub fn Bottom() -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            height: "24px",
            view_box: "0 -960 960 960",
            width: "24px",
            class: "fill-text-light dark:fill-text-dark",
            path {
                d: "M480-344 240-584l56-56 184 184 184-184 56 56-240 240Z"
            }
        }
    }
}
