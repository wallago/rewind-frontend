use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Deserialize)]
pub struct Board {
    pub uuid: String,
    pub name: String,
    pub position: i32,
}

#[derive(Serialize)]
pub struct NewBoard {
    pub name: String,
    pub position: Option<i32>,
}
