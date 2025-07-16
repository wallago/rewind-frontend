use serde::{Deserialize, Serialize};

use crate::models::{Priority, Status};

#[derive(Clone, PartialEq, Deserialize)]
pub struct Task {
    pub name: String,
    pub priority: Priority,
    pub status: Status,
}

#[derive(Serialize)]
pub struct NewTask {
    pub name: String,
    pub list_uuid: String,
    pub priority: Option<Priority>,
    pub status: Option<Status>,
    pub position: Option<i32>,
}

#[derive(Serialize)]
pub struct UpdateTask {
    pub name: Option<String>,
    pub priority: Option<Priority>,
    pub status: Option<Status>,
    pub position: Option<i32>,
}
