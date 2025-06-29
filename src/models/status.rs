use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::{
        fa_regular_icons::FaCircle as FaCircleEmpty,
        fa_solid_icons::{FaCircle, FaCircleHalfStroke},
    },
};

#[derive(Clone, PartialEq)]
pub enum Status {
    Todo,
    InProgress,
    Done,
}

impl Into<Element> for Status {
    fn into(self) -> Element {
        match self {
            Status::Todo => rsx!(Icon {
                class: "text-secondary",
                height: 16,
                icon: FaCircleEmpty,
            }),
            Status::InProgress => rsx!(Icon {
                class: "text-secondary",
                height: 16,
                icon: FaCircleHalfStroke,
            }),
            Status::Done => rsx!(Icon {
                class: "text-secondary",
                height: 16,
                icon: FaCircle,
            }),
        }
    }
}
