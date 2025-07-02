use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::fa_solid_icons::FaTag};

#[derive(Clone, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl Into<Element> for Priority {
    fn into(self) -> Element {
        match self {
            Priority::Low => rsx!(Icon {
                class: "text-priority-low",
                height: 16,
                icon: FaTag,
            }),
            Priority::Medium => rsx!(Icon {
                class: "text-priority-medium",
                height: 16,
                icon: FaTag,
            }),
            Priority::High => rsx!(Icon {
                class: "text-priority-high",
                height: 16,
                icon: FaTag,
            }),
        }
    }
}

impl ToString for Priority {
    fn to_string(&self) -> String {
        match self {
            Priority::Low => "Low".to_string(),
            Priority::Medium => "Medium".to_string(),
            Priority::High => "High".to_string(),
        }
    }
}

impl From<&str> for Priority {
    fn from(s: &str) -> Self {
        match s {
            "Low" => Priority::Low,
            "Medium" => Priority::Medium,
            "High" => Priority::High,
            _ => Priority::Low,
        }
    }
}
