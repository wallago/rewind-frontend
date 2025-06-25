use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Board {
    pub uuid: String,
    pub name: String,
    pub description: Option<String>,
    pub position: i32,
}

#[derive(Serialize, Clone)]
pub struct NewBoard {
    pub name: String,
    pub description: Option<String>,
    pub position: i32,
}

#[derive(Serialize, Clone)]
pub struct UpdateBoard {
    pub name: Option<String>,
    pub description: Option<String>,
    pub position: Option<i32>,
}
