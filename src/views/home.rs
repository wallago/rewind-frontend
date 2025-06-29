use dioxus::prelude::*;

use crate::{
    Route,
    components::{Button, Label},
};

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "p-4 h-full bg-primary border-2 border-secondary flex flex-col gap-4",
            Header {  }
            div {
                class: "grid gap-4 grid-cols-[repeat(auto-fit,_minmax(12rem,_1fr))]",
                Board {}
                Board {}
                Board {}
                Board {}
                Board {}
                Board {}
            }
        }
    }
}

#[component]
fn Header() -> Element {
    rsx! {
        Label {
            "Boards"
        }
    }
}

#[component]
fn Board() -> Element {
    rsx! {
        div {
            class: "h-fit w-48 p-2 bg-primary-2 border-2 border-secondary flex flex-col gap-4",
            div {
                class: "flex flex-col justify-center text-sm font-medium gap-2 w-full",
                Label {
                    variant: "outline",
                    class: "p-2",
                    width: "w-full",
                    div {
                        class: "truncate",
                        "Name: Project XYZ"
                    }
                }
                Label {
                    variant: "outline",
                    class: "p-2",
                    width: "w-full",
                    div {
                        class: "truncate",
                        "UUID: 571a9fa0-1bb4-4545-bdd3-b7315dcb6615"
                    }
                }
            }
            div {
                class: "flex justify-center",
                Button {
                    onclick: |_| {
                        navigator().push(Route::Board { uuid: "571a9fa0-1bb4-4545-bdd3-b7315dcb6615".to_string() });
                    },
                    class: "px-4",
                    "Details"
                }
            }
        }
    }
}
