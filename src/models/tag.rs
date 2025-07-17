use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct Tag {
    pub uuid: String,
    pub board_uuid: String,
    pub name: String,
    pub color: String,
}

#[derive(Serialize, Clone)]
pub struct NewTag {
    pub board_uuid: String,
    pub name: String,
    pub color: String,
}
