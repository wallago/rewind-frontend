use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct List {
    pub uuid: String,
    pub board_uuid: String,
    pub name: String,
    pub description: Option<String>,
    pub position: i32,
}

#[derive(Serialize, Clone)]
pub struct NewList {
    pub name: String,
    pub board_uuid: String,
    pub description: Option<String>,
    pub position: i32,
}

#[derive(Serialize, Clone)]
pub struct UpdateList {
    pub name: Option<String>,
    pub description: Option<String>,
    pub position: Option<i32>,
}
