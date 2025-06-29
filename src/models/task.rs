use crate::models::{Priority, Status};

#[derive(Clone, PartialEq)]
pub struct Task {
    pub text: String,
    pub priority: Priority,
    pub status: Status,
}
