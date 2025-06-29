use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Task {
    pub uuid: String,
    pub list_uuid: String,
    pub name: String,
    pub description: Option<String>,
    pub position: i32,
    pub status: String,
    pub priority: String,
}

#[derive(Serialize, Clone)]
pub struct NewTask {
    pub name: String,
    pub description: Option<String>,
    pub list_uuid: String,
    pub position: i32,
    pub status: String,
    pub priority: String,
}

#[derive(Serialize, Clone)]
pub struct UpdateTask {
    pub name: Option<String>,
    pub description: Option<String>,
    pub position: Option<i32>,
    pub status: Option<String>,
    pub priority: Option<String>,
}
