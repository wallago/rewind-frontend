use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Deserialize)]
pub struct List {
    pub uuid: String,
    pub name: String,
}

#[derive(Serialize)]
pub struct NewList {
    pub name: String,
    pub board_uuid: String,
    pub position: Option<i32>,
}

#[derive(Serialize)]
pub struct UpdateList {
    pub name: Option<String>,
    pub position: Option<i32>,
}
