use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::{
        fa_regular_icons::FaCircle as FaCircleEmpty,
        fa_solid_icons::{FaCircle, FaCircleHalfStroke},
    },
};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Deserialize, Serialize)]
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

impl ToString for Status {
    fn to_string(&self) -> String {
        match self {
            Status::Todo => "Todo".to_string(),
            Status::InProgress => "InProgress".to_string(),
            Status::Done => "Done".to_string(),
        }
    }
}

impl From<&str> for Status {
    fn from(s: &str) -> Self {
        match s {
            "Todo" => Status::Todo,
            "InProgress" => Status::InProgress,
            "Done" => Status::Done,
            _ => Status::Todo,
        }
    }
}
