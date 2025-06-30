use crate::models::{Priority, Status, Tag};

#[derive(Clone, PartialEq)]
pub struct Task {
    pub name: String,
    pub priority: Priority,
    pub status: Status,
    pub tags: Vec<Tag>,
}
