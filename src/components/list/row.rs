use super::model::List;
use dioxus::prelude::*;

#[component]
pub fn ListRow(list: List) -> Element {
    rsx!(li {
        class: "pt-1 border-t-2 border-border-light dark:border-border-dark",
        key: "{list.uuid}",
        "{list.name}"
    })
}

#[component]
pub fn ListRowTitle() -> Element {
    rsx!(
        div {
            class: "grid grid-cols-4 gap-4 font-bold",
            p { "name" },
            p { "description" },
            p { "tasks" },
            p { "" },
        }
    )
}

#[component]
pub fn ListRowSkeleton(i: i32) -> Element {
    rsx!(
        li {
            key: "{i}",
            class: "p-2 rounded",
            div {
                class: "grid grid-cols-3 gap-4",
                div { class: "h-4 bg-muted-light dark:bg-muted-dark rounded" }
                div { class: "h-4 bg-muted-light dark:bg-muted-dark rounded" }
                div { class: "h-4 bg-muted-light dark:bg-muted-dark rounded" }
            }
        }
    )
}
